use std::sync::Arc;

use bytes::Bytes;
use reqwest::{
    Method, RequestBuilder,
    header::{ACCEPT, CONTENT_TYPE, HeaderMap},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    AppAuth, BearerAuth, Error, Unauthenticated,
    auth::Authentication,
    error::{ApiError, ErrorEnvelope},
    model::UnixTimestamp,
    resources::{
        Broadcasting, Categories, Comments, Gifts, Movies, Search, Supporters, Users, Webhooks,
    },
};

const DEFAULT_BASE_URL: &str = "https://apiv2.twitcasting.tv/";

/// Parsed API rate-limit metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RateLimit {
    /// Request ceiling.
    pub limit: u64,
    /// Requests remaining.
    pub remaining: u64,
    /// Reset time.
    pub reset: UnixTimestamp,
}

/// A decoded API result and response metadata.
#[derive(Clone, Debug)]
pub struct ApiResponse<T> {
    /// Decoded response.
    pub value: T,
    /// Rate-limit headers, when all three were present and valid.
    pub rate_limit: Option<RateLimit>,
}

/// A live thumbnail binary response.
#[derive(Clone, Debug)]
pub struct Thumbnail {
    /// Image bytes.
    pub bytes: Bytes,
    /// `image/jpeg` or `image/png`.
    pub media_type: String,
    /// Final URL after redirects.
    pub final_url: Url,
}

/// Builder for an authenticated API client.
#[derive(Clone, Debug)]
pub struct ClientBuilder<A> {
    auth: A,
    base_url: Url,
    http: Option<reqwest::Client>,
}

impl<A> ClientBuilder<A> {
    /// Creates a builder using the production API base URL.
    pub fn new(auth: A) -> Result<Self, Error> {
        Ok(Self {
            auth,
            base_url: Url::parse(DEFAULT_BASE_URL)?,
            http: None,
        })
    }

    /// Overrides the API base URL, primarily for testing or proxies.
    #[must_use]
    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    /// Injects a preconfigured HTTP client.
    #[must_use]
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// Builds the client.
    pub fn build(self) -> Result<Client<A>, Error> {
        if self.base_url.cannot_be_a_base() {
            return Err(Error::InvalidBaseUrl { url: self.base_url });
        }
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder().gzip(true).build()?,
        };
        Ok(Client {
            inner: Arc::new(Inner {
                auth: self.auth,
                base_url: self.base_url,
                http,
            }),
        })
    }
}

struct Inner<A> {
    auth: A,
    base_url: Url,
    http: reqwest::Client,
}

/// Generic TwitCasting API client.
pub struct Client<A> {
    inner: Arc<Inner<A>>,
}

impl<A> Clone for Client<A> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Client<BearerAuth> {
    /// Creates a user-authenticated client.
    pub fn bearer(token: impl Into<String>) -> Result<Self, Error> {
        ClientBuilder::new(BearerAuth::new(token))?.build()
    }
}

impl Client<AppAuth> {
    /// Creates an application-authenticated client.
    pub fn application(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Result<Self, Error> {
        ClientBuilder::new(AppAuth::new(client_id, client_secret))?.build()
    }
}

impl Client<Unauthenticated> {
    /// Creates a client for endpoints that do not require credentials.
    pub fn unauthenticated() -> Result<Self, Error> {
        ClientBuilder::new(Unauthenticated)?.build()
    }
}

impl<A> Client<A> {
    /// User operations.
    #[must_use]
    pub fn users(&self) -> Users<'_, A> {
        Users::new(self)
    }

    pub(crate) fn endpoint(&self, segments: &[&str]) -> Url {
        let mut url = self.inner.base_url.clone();
        {
            let mut path = url
                .path_segments_mut()
                .expect("ClientBuilder rejects non-hierarchical base URLs");
            path.pop_if_empty();
            path.extend(segments);
        }
        url
    }

    pub(crate) fn unauthenticated_request(
        &self,
        method: Method,
        segments: &[&str],
    ) -> RequestBuilder {
        self.inner.http.request(method, self.endpoint(segments))
    }

    pub(crate) async fn send_thumbnail(
        &self,
        request: RequestBuilder,
    ) -> Result<ApiResponse<Thumbnail>, Error> {
        let response = request.send().await?;
        let status = response.status();
        let final_url = response.url().clone();
        let headers = response.headers().clone();
        let media_type = media_type(&headers);
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
                message: status.canonical_reason().unwrap_or("HTTP error").to_owned(),
                details: None,
                rate_limit: None,
            }
            .into());
        }
        let Some(media_type) = media_type else {
            return Err(Error::UnexpectedContentType {
                actual: None,
                expected: "image/jpeg or image/png",
            });
        };
        if !matches!(media_type.as_str(), "image/jpeg" | "image/png") {
            return Err(Error::UnexpectedContentType {
                actual: Some(media_type),
                expected: "image/jpeg or image/png",
            });
        }
        Ok(ApiResponse {
            value: Thumbnail {
                bytes,
                media_type,
                final_url,
            },
            rate_limit: None,
        })
    }
}

impl<A: Authentication> Client<A> {
    /// Starts a custom client builder.
    pub fn builder(auth: A) -> Result<ClientBuilder<A>, Error> {
        ClientBuilder::new(auth)
    }

    /// Movie operations.
    #[must_use]
    pub fn movies(&self) -> Movies<'_, A> {
        Movies::new(self)
    }

    /// Comment operations.
    #[must_use]
    pub fn comments(&self) -> Comments<'_, A> {
        Comments::new(self)
    }

    /// Gift operations. Methods are available only with bearer authentication.
    #[must_use]
    pub fn gifts(&self) -> Gifts<'_, A> {
        Gifts::new(self)
    }

    /// Support relationship operations.
    #[must_use]
    pub fn supporters(&self) -> Supporters<'_, A> {
        Supporters::new(self)
    }

    /// Category operations.
    #[must_use]
    pub fn categories(&self) -> Categories<'_, A> {
        Categories::new(self)
    }

    /// Search operations.
    #[must_use]
    pub fn search(&self) -> Search<'_, A> {
        Search::new(self)
    }

    /// Application webhook operations.
    #[must_use]
    pub fn webhooks(&self) -> Webhooks<'_, A> {
        Webhooks::new(self)
    }

    /// Broadcasting operations.
    #[must_use]
    pub fn broadcasting(&self) -> Broadcasting<'_, A> {
        Broadcasting::new(self)
    }

    pub(crate) fn request(&self, method: Method, segments: &[&str]) -> RequestBuilder {
        let request = self
            .inner
            .http
            .request(method, self.endpoint(segments))
            .header(ACCEPT, "application/json")
            .header("X-Api-Version", "2.0");
        self.inner.auth.apply(request)
    }

    pub(crate) async fn send_json<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<ApiResponse<T>, Error> {
        let response = request.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let rate_limit = parse_rate_limit(&headers);
        let content_type = media_type(&headers);
        let body = response.bytes().await?;

        if !status.is_success() {
            let envelope =
                serde_json::from_slice::<ErrorEnvelope>(&body).map_err(|source| Error::Decode {
                    source,
                    body: truncate(&body),
                })?;
            return Err(ApiError {
                status,
                code: envelope.error.code,
                message: envelope.error.message,
                details: envelope.error.details,
                rate_limit,
            }
            .into());
        }

        if !content_type
            .as_deref()
            .is_some_and(|value| value == "application/json" || value.ends_with("+json"))
        {
            return Err(Error::UnexpectedContentType {
                actual: content_type,
                expected: "application/json",
            });
        }
        let value = serde_json::from_slice(&body).map_err(|source| Error::Decode {
            source,
            body: truncate(&body),
        })?;
        Ok(ApiResponse { value, rate_limit })
    }
}

fn media_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .map(str::to_ascii_lowercase)
}

fn parse_rate_limit(headers: &HeaderMap) -> Option<RateLimit> {
    let number = |name: &'static str| {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
    };
    Some(RateLimit {
        limit: number("X-RateLimit-Limit")?,
        remaining: number("X-RateLimit-Remaining")?,
        reset: UnixTimestamp(
            headers
                .get("X-RateLimit-Reset")?
                .to_str()
                .ok()?
                .parse()
                .ok()?,
        ),
    })
}

fn truncate(bytes: &[u8]) -> Vec<u8> {
    bytes[..bytes.len().min(4096)].to_vec()
}

#[cfg(test)]
mod tests {
    use super::{Client, ClientBuilder};
    use crate::{BearerAuth, ScreenId, Unauthenticated};
    use url::Url;

    #[test]
    fn path_segments_are_encoded() {
        let client = ClientBuilder::new(BearerAuth::new("token"))
            .unwrap()
            .base_url(Url::parse("http://localhost/root/").unwrap())
            .build()
            .unwrap();
        let value = ScreenId::new("a/b c");
        assert_eq!(
            client.endpoint(&["users", value.as_str()]).as_str(),
            "http://localhost/root/users/a%2Fb%20c"
        );
    }

    #[test]
    fn constructors_have_expected_types() {
        let _: Client<BearerAuth> = Client::bearer("token").unwrap();
        let _: Client<Unauthenticated> = Client::unauthenticated().unwrap();
    }
}
