//! Posts a comment using bearer authentication.

use twitcasting::{Client, CommentText, MovieId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN")?)?;
    let movie_id = MovieId::new(std::env::var("TWITCASTING_MOVIE_ID")?);
    let comment = CommentText::new("モイ！");
    println!(
        "{:#?}",
        client.comments().post(&movie_id, &comment).await?.value
    );
    Ok(())
}
