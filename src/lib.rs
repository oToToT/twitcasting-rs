//! Async bindings for every documented TwitCasting HTTP API v2 endpoint.
//!
//! Realtime/WebSocket APIs are intentionally outside this crate's current scope.

mod auth;
mod client;
mod error;
mod id;
mod model;
#[cfg(feature = "oauth")]
mod oauth;
mod request;
mod resources;
#[cfg(feature = "webhooks")]
mod webhook;

pub use auth::{AppAuth, Authentication, BearerAuth, SecretString, Unauthenticated};
pub use client::{ApiResponse, Client, ClientBuilder, RateLimit, Thumbnail};
pub use error::{ApiError, Error, ValidationDetails};
pub use id::{CommentId, LiveScheduleId, MovieId, ScreenId, UserId, UserRef};
pub use model::*;
#[cfg(feature = "oauth")]
pub use oauth::{OAuthClient, OAuthClientBuilder};
pub use request::*;
pub use resources::{
    Broadcasting, Categories, Comments, Gifts, Movies, Search, Supporters, Users,
};
#[cfg(feature = "webhooks")]
pub use resources::Webhooks;
#[cfg(feature = "webhooks")]
pub use webhook::{WebhookPayload, decode_webhook};
