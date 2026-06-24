use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use url::Url;

use crate::{AccessToken, ApiError, ApiResponse, Error, SecretString, error::ErrorEnvelope};

const DEFAULT_BASE_URL: &str = "https://apiv2.twitcasting.tv/";

/// OAuth authorization and token-exchange client.
#[derive(Clone, Debug)]
pub struct OAuthClient {
    client_id: String,
    client_secret: SecretString,
    redirect_uri: Url,
    base_url: Url,
    http: reqwest::Client,
}

/// Builder for [`OAuthClient`].
#[derive(Clone, Debug)]
pub struct OAuthClientBuilder {
    client_id: String,
    client_secret: SecretString,
    redirect_uri: Url,
    base_url: Url,
    http: Option<reqwest::Client>,
}

impl OAuthClient {
    /// Creates an OAuth builder.
    pub fn builder(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: Url,
    ) -> Result<OAuthClientBuilder, Error> {
        Ok(OAuthClientBuilder {
            client_id: client_id.into(),
            client_secret: SecretString::new(client_secret),
            redirect_uri,
            base_url: Url::parse(DEFAULT_BASE_URL)?,
            http: None,
        })
    }

    /// Generates an authorization-code grant URL.
    #[must_use]
    pub fn authorization_code_url(&self, state: Option<&str>) -> Url {
        self.authorization_url("code", state)
    }

    /// Generates an implicit grant URL. Authorization-code flow is recommended.
    #[must_use]
    pub fn implicit_authorization_url(&self, state: Option<&str>) -> Url {
        self.authorization_url("token", state)
    }

    fn authorization_url(&self, response_type: &str, state: Option<&str>) -> Url {
        let mut url = self.base_url.join("oauth2/authorize").expect("static path");
        {
            let mut query = url.query_pairs_mut();
            query
                .append_pair("client_id", &self.client_id)
                .append_pair("response_type", response_type);
            if let Some(state) = state {
                query.append_pair("state", state);
            }
        }
        url
    }

    /// Exchanges an authorization code for a bearer token.
    pub async fn exchange_code(&self, code: &str) -> Result<ApiResponse<AccessToken>, Error> {
        #[derive(Serialize)]
        struct Form<'a> {
            code: &'a str,
            grant_type: &'static str,
            client_id: &'a str,
            client_secret: &'a str,
            redirect_uri: &'a str,
        }
        let response = self
            .http
            .post(self.base_url.join("oauth2/access_token")?)
            .form(&Form {
                code,
                grant_type: "authorization_code",
                client_id: &self.client_id,
                client_secret: self.client_secret.expose_secret(),
                redirect_uri: self.redirect_uri.as_str(),
            })
            .send()
            .await?;
        let status = response.status();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let bytes = response.bytes().await?;
        if !status.is_success() {
            if let Ok(envelope) = serde_json::from_slice::<ErrorEnvelope>(&bytes) {
                return Err(ApiError {
                    status,
                    code: envelope.error.code,
                    message: envelope.error.message,
                    details: envelope.error.details,
                    rate_limit: None,
                }
                .into());
            }
            return Err(ApiError {
                status,
                code: i64::from(status.as_u16()),
                message: status
                    .canonical_reason()
                    .unwrap_or("OAuth error")
                    .to_owned(),
                details: None,
                rate_limit: None,
            }
            .into());
        }
        if !content_type
            .as_deref()
            .is_some_and(|value| value.starts_with("application/json"))
        {
            return Err(Error::UnexpectedContentType {
                actual: content_type,
                expected: "application/json",
            });
        }
        let value = serde_json::from_slice(&bytes).map_err(|source| Error::Decode {
            source,
            body: bytes[..bytes.len().min(4096)].to_vec(),
        })?;
        Ok(ApiResponse {
            value,
            rate_limit: None,
        })
    }
}

impl OAuthClientBuilder {
    /// Overrides the OAuth base URL.
    #[must_use]
    pub fn base_url(mut self, value: Url) -> Self {
        self.base_url = value;
        self
    }

    /// Injects an HTTP client.
    #[must_use]
    pub fn http_client(mut self, value: reqwest::Client) -> Self {
        self.http = Some(value);
        self
    }

    /// Builds the OAuth client.
    pub fn build(self) -> Result<OAuthClient, Error> {
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder().gzip(true).build()?,
        };
        Ok(OAuthClient {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: self.redirect_uri,
            base_url: self.base_url,
            http,
        })
    }
}
