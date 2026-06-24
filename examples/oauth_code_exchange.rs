//! Builds an authorization URL and exchanges an authorization code.

use twitcasting::OAuthClient;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthClient::builder(
        std::env::var("TWITCASTING_CLIENT_ID")?,
        std::env::var("TWITCASTING_CLIENT_SECRET")?,
        Url::parse(&std::env::var("TWITCASTING_REDIRECT_URI")?)?,
    )?
    .build()?;

    println!("{}", oauth.authorization_code_url(Some("csrf-state")));
    let token = oauth
        .exchange_code(&std::env::var("TWITCASTING_AUTHORIZATION_CODE")?)
        .await?;
    println!("token type: {:?}", token.value.token_type);
    Ok(())
}
