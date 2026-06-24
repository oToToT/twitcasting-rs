//! Retrieves redacted RTMP publishing credentials.

use twitcasting::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN")?)?;
    let credentials = client.broadcasting().rtmp_credentials().await?.value;
    println!("RTMP enabled: {}", credentials.enabled);
    // Call expose_secret() only where the publishing software needs the value.
    Ok(())
}
