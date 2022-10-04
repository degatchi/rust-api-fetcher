use rust_api_fetcher::api::RickAndMorty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = RickAndMorty::new();
    api.get_data().await;

    Ok(())
}
