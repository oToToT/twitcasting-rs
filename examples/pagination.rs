//! Fetches one page of a user's movie history.

use twitcasting::{Client, MovieListRequest, ScreenId, UserRef};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::bearer(std::env::var("TWITCASTING_ACCESS_TOKEN")?)?;
    let user = UserRef::from(ScreenId::new("twitcasting_jp"));
    let options = MovieListRequest::default().offset(20).limit(20);
    let movies = client.movies().by_user(&user, &options).await?;
    println!("{} movies returned", movies.value.movies.len());
    Ok(())
}
