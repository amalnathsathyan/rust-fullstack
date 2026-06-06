use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    let api_key = env::var("CRYPTORANK_API_KEY")
        .expect("API KEY NOT SET");
    let client = Client::new();

    let response = client
        .get("https://api.cryptorank.io/v2/global")
        .header("X-Api-Key", api_key)
        .send()
        .await?;

    let status = response.status();
    println!("Status: {}",status);

    let body = response.text().await?;
    println!("Response:\n{}",body);
    Ok(())
}
