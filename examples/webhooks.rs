//! Registers and decodes webhooks.

use twitcasting::{Client, UserId, WebhookEvent, WebhookEvents, WebhookPayload, decode_webhook};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::application(
        std::env::var("TWITCASTING_CLIENT_ID")?,
        std::env::var("TWITCASTING_CLIENT_SECRET")?,
    )?;
    let user_id = UserId::new(std::env::var("TWITCASTING_USER_ID")?);
    let events = WebhookEvents::new([WebhookEvent::LiveStart, WebhookEvent::LiveEnd]);
    client.webhooks().register(&user_id, &events).await?;

    let incoming = br#"{"event":"future-event","signature":"opaque","data":{}}"#;
    if let WebhookPayload::Unknown { event, .. } = decode_webhook(incoming)? {
        println!("unknown event preserved: {event}");
    }
    Ok(())
}
