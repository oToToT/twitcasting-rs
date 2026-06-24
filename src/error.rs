use std::{collections::BTreeMap, fmt};

use reqwest::StatusCode;
use serde::Deserialize;

use crate::RateLimit;

/// Server-side validation detail values keyed by request field.
pub type ValidationDetails = BTreeMap<String, Vec<String>>;

/// A structured TwitCasting API error.
#[derive(Clone, Debug)]
pub struct ApiError {
    /// HTTP response status.
    pub status: StatusCode,
    /// Numeric TwitCasting error code.
    pub code: i64,
    /// Human-readable server message.
    pub message: String,
    /// Validation details, when code 1001 is returned.
    pub details: Option<ValidationDetails>,
    /// Rate-limit metadata parsed from the error response.
    pub rate_limit: Option<RateLimit>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TwitCasting API error {} (HTTP {}): {}",
            self.code, self.status, self.message
        )
    }
}

impl std::error::Error for ApiError {}

/// Errors produced by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A custom API URL cannot be used as a hierarchical base URL.
    #[error("URL cannot be used as an API base: {url}")]
    InvalidBaseUrl {
        /// Rejected URL.
        url: url::Url,
    },
    /// URL construction failed.
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),
    /// The HTTP transport failed.
    #[error("HTTP transport failed: {0}")]
    Transport(#[from] reqwest::Error),
    /// A successful response had an unexpected content type.
    #[error("unexpected content type {actual:?}; expected {expected}")]
    UnexpectedContentType {
        /// Received content type, if present.
        actual: Option<String>,
        /// Expected media type class.
        expected: &'static str,
    },
    /// A JSON response could not be decoded.
    #[error("failed to decode JSON response: {source}")]
    Decode {
        /// JSON decoding failure.
        source: serde_json::Error,
        /// Response bytes, truncated to a safe diagnostic size.
        body: Vec<u8>,
    },
    /// The server returned a structured API error.
    #[error(transparent)]
    Api(#[from] ApiError),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorBody {
    pub code: i64,
    pub message: String,
    #[serde(default)]
    pub details: Option<ValidationDetails>,
}
