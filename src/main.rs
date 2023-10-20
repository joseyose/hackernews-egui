use std::error::Error;

use hackernewsapi::HackerNewsAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = HackerNewsAPI::new();

    let response = api.collect_all_stories().await?;
    response.debug_print_stories(5).await?;

    Ok(())
}
