//! Looks up a user with application authentication.

use twitcasting::{Client, ScreenId, UserRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::application(
        std::env::var("TWITCASTING_CLIENT_ID")?,
        std::env::var("TWITCASTING_CLIENT_SECRET")?,
    )?;
    let user = UserRef::from(ScreenId::new("twitcasting_jp"));
    println!("{:#?}", client.users().get(&user).await?.value);
    Ok(())
}
