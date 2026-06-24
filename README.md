# twitcasting

[![Crates.io](https://img.shields.io/crates/v/twitcasting.svg)](https://crates.io/crates/twitcasting)
[![Documentation](https://docs.rs/twitcasting/badge.svg)](https://docs.rs/twitcasting)
[![CI](https://github.com/oToToT/twitcasting-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/oToToT/twitcasting-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://ototot.mit-license.org/)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)

Async, strongly typed Rust bindings for the
[TwitCasting API v2](https://apiv2-doc.twitcasting.tv/).

The crate covers every documented HTTP API v2 endpoint. Realtime/WebSocket
support is intentionally outside the current scope.

## Features

- Separate `Client<BearerAuth>` and `Client<AppAuth>` types, with
  authentication-specific methods enforced at compile time
- Resource-oriented APIs through `users()`, `movies()`, `comments()`,
  `gifts()`, `supporters()`, `categories()`, `search()`, `webhooks()`, and
  `broadcasting()`
- Strongly typed identifiers such as `UserId`, `ScreenId`, `MovieId`, and
  `CommentId`
- Structured API errors and optional rate-limit metadata
- Redacted access tokens, application secrets, webhook signatures, and RTMP
  credentials
- Typed thumbnail responses containing bytes, media type, and final URL
- Forward-compatible webhook and response enum decoding
- Custom base URL and `reqwest::Client` injection for testing
- Rustls-backed HTTPS and gzip support

Request values are serialized without duplicating TwitCasting's mutable
business-rule validation. The server remains authoritative for limits,
permissions, and accepted values.

## Installation

```toml
[dependencies]
twitcasting = "0.1"
```

The minimum supported Rust version is 1.85, using Rust 2024 Edition.

## Quick start

```rust
use twitcasting::{Client, ScreenId, UserRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN")?)?;
    let user = UserRef::from(ScreenId::new("twitcasting_jp"));

    let response = client.users().get(&user).await?;
    println!("{}", response.value.user.name);

    if let Some(rate_limit) = response.rate_limit {
        println!("requests remaining: {}", rate_limit.remaining);
    }

    Ok(())
}
```

Your application needs an async runtime compatible with `reqwest`, such as
Tokio:

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
twitcasting = "0.1"
```

## Authentication

Bearer authentication is used for operations associated with a TwitCasting
user:

```rust
use twitcasting::Client;

fn bearer_client() -> Result<Client<twitcasting::BearerAuth>, twitcasting::Error> {
    Client::bearer("access-token")
}
```

Application authentication is used for application-level operations such as
webhook registration:

```rust
use twitcasting::Client;

fn application_client() -> Result<Client<twitcasting::AppAuth>, twitcasting::Error> {
    Client::application("client-id", "client-secret")
}
```

OAuth authorization URL generation and authorization-code exchange are
provided separately by `OAuthClient`.

## API coverage

| Resource | Operations |
| --- | --- |
| Users | User lookup, credential verification, live thumbnails, upcoming schedules |
| Movies | Movie lookup, user movie history, current live, subtitle and hashtag mutations |
| Comments | List, post, and delete comments |
| Gifts | Poll recently received gifts |
| Supporters | Relationship status, support/unsupport, supporting and supporter lists |
| Categories | List active live categories |
| Search | Search users and live broadcasts |
| Webhooks | List, register, remove, and decode webhook payloads |
| Broadcasting | Retrieve RTMP publishing credentials |
| OAuth | Authorization URLs and authorization-code exchange |

See the complete API documentation on [docs.rs](https://docs.rs/twitcasting).

## Examples

Runnable examples are available in the
[examples directory](https://github.com/oToToT/twitcasting-rs/tree/main/examples):

- [Application authentication](https://github.com/oToToT/twitcasting-rs/blob/main/examples/application_auth.rs)
- [Bearer authentication](https://github.com/oToToT/twitcasting-rs/blob/main/examples/bearer_auth.rs)
- [OAuth code exchange](https://github.com/oToToT/twitcasting-rs/blob/main/examples/oauth_code_exchange.rs)
- [Pagination](https://github.com/oToToT/twitcasting-rs/blob/main/examples/pagination.rs)
- [Posting comments](https://github.com/oToToT/twitcasting-rs/blob/main/examples/post_comment.rs)
- [Webhook registration and decoding](https://github.com/oToToT/twitcasting-rs/blob/main/examples/webhooks.rs)
- [RTMP credentials](https://github.com/oToToT/twitcasting-rs/blob/main/examples/rtmp.rs)

Run an example with the required environment variables:

```console
cargo run --example bearer_auth
```

## Webhook signatures

`decode_webhook` preserves the signature included in incoming webhook bodies.
It does not claim to verify that signature because the official TwitCasting
documentation does not define a verification algorithm.

## License

Released under the [MIT License](https://ototot.mit-license.org/). A copy is
included in [LICENSE](https://github.com/oToToT/twitcasting-rs/blob/main/LICENSE).
