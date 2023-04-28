use backoff::future::retry;
use backoff::ExponentialBackoff;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    retry(ExponentialBackoff::default(), || async {
        println!("Fetching {}", url);
        Ok(reqwest::get(url).await?.text().await?)
    })
    .await
}
async fn request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    retry(ExponentialBackoff::default(), || async {
        println!("Fetching {}", url);
        Ok(reqwest::get(url).await?)
    })
    .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match fetch_url("https://www.rust-lang.org").await {
        Ok(page) => println!("Successfully fetched {}", page),
        Err(err) => panic!("Failed to fetch: {}", err),
    }
    match request("https://www.rust-lang.org").await?.text().await {
        Ok(page) => println!("Successfully fetched {}", page),
        Err(err) => panic!("Failed to fetch: {}", err),
    }
    Ok(())
}
