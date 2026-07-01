use std::fmt;

use reqwest::RequestBuilder;
use serde::Deserialize;

/// A secret-bearing string whose `Debug` and `Display` representations are redacted.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    /// Wraps a secret value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Exposes the secret to code that must transmit it.
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretString([REDACTED])")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

mod private {
    pub trait Sealed {}
}

/// Authentication accepted by a TwitCasting API client.
pub trait Authentication: private::Sealed + Clone + Send + Sync + 'static {
    #[doc(hidden)]
    fn apply(&self, request: RequestBuilder) -> RequestBuilder;
}

/// Marker for a client without credentials.
///
/// This client type is only useful for endpoints that explicitly do not require
/// authentication, such as live thumbnails.
///
/// ```compile_fail
/// use twitcasting::{Client, ScreenId, UserRef};
///
/// let client = Client::unauthenticated()?;
/// let user = UserRef::from(ScreenId::new("twitcasting_jp"));
/// client.users().get(&user);
/// # Ok::<(), twitcasting::Error>(())
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Unauthenticated;

/// User-level OAuth bearer authentication.
#[derive(Clone, Debug)]
pub struct BearerAuth {
    token: SecretString,
}

impl BearerAuth {
    /// Creates bearer authentication from an OAuth access token.
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: SecretString::new(token),
        }
    }

    /// Returns the redacted token wrapper.
    #[must_use]
    pub fn token(&self) -> &SecretString {
        &self.token
    }
}

impl private::Sealed for BearerAuth {}

impl Authentication for BearerAuth {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(self.token.expose_secret())
    }
}

/// Application-level HTTP Basic authentication.
#[derive(Clone, Debug)]
pub struct AppAuth {
    client_id: String,
    client_secret: SecretString,
}

impl AppAuth {
    /// Creates application authentication.
    #[must_use]
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: SecretString::new(client_secret),
        }
    }

    /// Returns the public client identifier.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns the redacted client secret wrapper.
    #[must_use]
    pub fn client_secret(&self) -> &SecretString {
        &self.client_secret
    }
}

impl private::Sealed for AppAuth {}

impl Authentication for AppAuth {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(&self.client_id, Some(self.client_secret.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::SecretString;

    #[test]
    fn secrets_are_redacted() {
        let secret = SecretString::new("do-not-print");
        assert_eq!(format!("{secret}"), "[REDACTED]");
        assert_eq!(format!("{secret:?}"), "SecretString([REDACTED])");
        assert!(!format!("{secret:?}").contains("do-not-print"));
    }
}
