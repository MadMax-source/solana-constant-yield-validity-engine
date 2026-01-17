use std::env;
use std::sync::Mutex;
use std::time::Duration;

use chrono::Utc;
use once_cell::sync::Lazy;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde_json::Value;
use tokio::time::interval;

// =====================
// Constants (FR-3)
// =====================
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const JUPITER_PRICE_URL: &str =
    "https://api.jup.ag/price/v3?ids=So11111111111111111111111111111111111111112";

const BUY_IN_SOL: f64 = 0.1333;

// =====================
// Hand Definition
// =====================
#[derive(Debug)]
struct Hand {
    entry_sol_price_usd: f64,
    buy_in_usd: f64,
    buy_in_sol: f64,
    opened_at: String,
}

// =====================
// Global Hand Store
// =====================
static HANDS: Lazy<Mutex<Vec<Hand>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

// =====================
// Hand Creation (FR-4)
// =====================
fn open_new_hand(sol_price_usd: f64) -> Hand {
    Hand {
        entry_sol_price_usd: sol_price_usd,
        buy_in_usd: sol_price_usd * BUY_IN_SOL,
        buy_in_sol: BUY_IN_SOL,
        opened_at: Utc::now().to_rfc3339(),
    }
}

// =====================
// Main Loop
// =====================
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        if let Err(e) = fetch_and_process_price().await {
            eprintln!("Price error: {}", e);
        }
    }
}

// =====================
// Price Fetch + Step 4
// =====================
async fn fetch_and_process_price() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("JUP_API_KEY").unwrap_or_else(|_| String::new());

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    if !api_key.is_empty() {
        headers.insert("x-api-key", HeaderValue::from_str(&api_key)?);
    }

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

    let sol = json
        .get(SOL_MINT)
        .ok_or("SOL price not available")?;

    let sol_price_usd = sol["usdPrice"]
        .as_f64()
        .ok_or("Invalid price")?;

    // =====================
    // TRADE OPPORTUNITY
    // (placeholder trigger)
    // =====================
    if sol_price_usd > 0.0 {
        let hand = open_new_hand(sol_price_usd);

        let mut hands = HANDS.lock().unwrap();
        hands.push(hand);

        let last = hands.last().unwrap();

        println!(
            "NEW HAND | SOL: ${:.6} | Buy USD: ${:.6} | Buy SOL: {:.4} | Time: {}",
            last.entry_sol_price_usd,
            last.buy_in_usd,
            last.buy_in_sol,
            last.opened_at
        );
    }

    Ok(())
}





/* 
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

 */ 