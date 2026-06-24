//! Verifies a bearer token.

use twitcasting::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN")?)?;
    println!(
        "{:#?}",
        client.users().verify_credentials().await?.value.user
    );
    Ok(())
}
