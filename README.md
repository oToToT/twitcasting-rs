# twitcasting

Async Rust bindings for every documented TwitCasting HTTP API v2 endpoint.
Realtime/WebSocket support is intentionally excluded.

The crate uses distinct identifier types, authentication-specific methods, URL
types, redacted secrets, structured API errors, and optional rate-limit
metadata. TwitCasting remains authoritative for request constraints: values are
sent to the server without duplicating mutable length and pagination rules in
the client.

```rust,no_run
use twitcasting::{Client, ScreenId, UserRef};

# async fn run() -> Result<(), twitcasting::Error> {
let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN").unwrap())?;
let user = UserRef::from(ScreenId::new("twitcasting_jp"));
let response = client.users().get(&user).await?;
println!("{}", response.value.user.name);
# Ok(())
# }
```

See the `examples/` directory for OAuth, application authentication,
pagination, comments, webhooks, and RTMP publishing credentials.

