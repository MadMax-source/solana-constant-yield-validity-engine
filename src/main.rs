use std::env;
use std::time::Duration;

use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde_json::Value;
use tokio::time::interval;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const JUPITER_PRICE_URL: &str =
    "https://api.jup.ag/price/v3?ids=So11111111111111111111111111111111111111112";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;
        if let Err(e) = get_sol_price().await {
            eprintln!("Jupiter price error: {}", e);
        }
    }
}

async fn get_sol_price() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("JUP_API_KEY")
        .unwrap_or_else(|_| "c208a0a1-7b2f-4415-9641-d26a474012c2".to_string());

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        "x-api-key",
        HeaderValue::from_str(&api_key)?,
    );

    let client = reqwest::Client::new();
    let res = client
        .get(JUPITER_PRICE_URL)
        .headers(headers)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status()).into());
    }

    let json: Value = res.json().await?;

    let sol = json.get(SOL_MINT);
    if sol.is_none() {
        println!("SOL price not available");
        return Ok(());
    }

    let sol = sol.unwrap();

    let usd_price = sol["usdPrice"].as_f64().unwrap_or(0.0);

    println!(
        "{:?}",
        serde_json::json!({
            "rawPrice": usd_price,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    );

    Ok(())
}








/*
fn main() {
    println!("Hello, world!");
}

*/