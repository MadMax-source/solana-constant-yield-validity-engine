use std::sync::Mutex;
use std::time::Duration;

use chrono::Utc;
use once_cell::sync::Lazy;
use tokio::time::interval;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
//use solana_sdk::signers::Signers;

mod constants;
mod price_feed;
mod swap;

use constants::BUY_IN_SOL;
use price_feed::fetch_sol_price_usd;
use swap::{
    get_buy_quote,
    build_buy_swap_tx,
    sign_and_send_tx,
};

#[derive(Debug)]
struct Hand {
    entry_sol_price_usd: f64,
    buy_in_usd: f64,
    buy_in_sol: f64,
    opened_at: String,
}

static HANDS: Lazy<Mutex<Vec<Hand>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

static HAS_BOUGHT: Lazy<Mutex<bool>> =
    Lazy::new(|| Mutex::new(false));


fn open_new_hand(sol_price_usd: f64) -> Hand {
    Hand {
        entry_sol_price_usd: sol_price_usd,
        buy_in_usd: sol_price_usd * BUY_IN_SOL,
        buy_in_sol: BUY_IN_SOL,
        opened_at: Utc::now().to_rfc3339(),
    }
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        match fetch_sol_price_usd().await {
            Ok(sol_price_usd) => {
                process_price(sol_price_usd).await;
            }
            Err(e) => {
                eprintln!("❌ Price error: {}", e);
            }
        }
    }
}

async fn process_price(sol_price_usd: f64) {
    
    let mut has_bought = HAS_BOUGHT.lock().unwrap();
    if *has_bought {
        return;
    }

    println!("🟢 Price update: ${:.4}", sol_price_usd);

    let keypair = Keypair::from_base58_string(
        &std::env::var("WALLET_PRIVATE_KEY")
            .expect("Missing WALLET_PRIVATE_KEY env variable"),
    );

    let input_mint =
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let amount: u64 = 1_000_000; // 1 USDT (6 decimals)
    let slippage_bps = 50;

    println!("🚀 Executing BUY via Jupiter...");

    let quote = match get_buy_quote(input_mint, amount, slippage_bps).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ Quote failed: {}", e);
            return;
        }
    };

    let base64_tx = match build_buy_swap_tx(
        &quote,
        &keypair.pubkey().to_string(),
    )
    .await
    {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("❌ Build tx failed: {}", e);
            return;
        }
    };

    match sign_and_send_tx(
        "https://api.mainnet-beta.solana.com",
        &base64_tx,
        &keypair,
    ) {
        Ok(sig) => {
            println!(
                "✅ BUY CONFIRMED: https://solscan.io/tx/{}",
                sig
            );

            let hand = open_new_hand(sol_price_usd);
            let mut hands = HANDS.lock().unwrap();
            hands.push(hand);

            *has_bought = true;
        }
        Err(e) => {
            eprintln!("❌ Transaction failed: {}", e);
        }
    }
}




/* 
use std::sync::Mutex;
use std::time::Duration;

use chrono::Utc;
use once_cell::sync::Lazy;
use tokio::time::interval;

mod constants;
mod price_feed;

use constants::BUY_IN_SOL;
use price_feed::fetch_sol_price_usd;

#[derive(Debug)]
struct Hand {
    entry_sol_price_usd: f64,
    buy_in_usd: f64,
    buy_in_sol: f64,
    opened_at: String,
}

static HANDS: Lazy<Mutex<Vec<Hand>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

fn open_new_hand(sol_price_usd: f64) -> Hand {
    Hand {
        entry_sol_price_usd: sol_price_usd,
        buy_in_usd: sol_price_usd * BUY_IN_SOL,
        buy_in_sol: BUY_IN_SOL,
        opened_at: Utc::now().to_rfc3339(),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        match fetch_sol_price_usd().await {
            Ok(sol_price_usd) => {
                process_price(sol_price_usd);
            }
            Err(e) => {
                eprintln!("Price error: {}", e);
            }
        }
    }
}
fn process_price(sol_price_usd: f64) {
    
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
}


*/
