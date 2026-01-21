// src/swap.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
    signers::Signers,
  };
use crate::constants::SOL_MINT;
use bincode;
use base64;
const JUP_QUOTE_URL: &str = "https://lite-api.jup.ag/swap/v1/quote";
const JUP_SWAP_URL: &str = "https://lite-api.jup.ag/swap/v1/swap";

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub route_plan: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct SwapRequest<'a> {
    quote_response: &'a QuoteResponse,
    user_public_key: String,
    dynamic_compute_unit_limit: bool,
    dynamic_slippage: bool,
    prioritization_fee_lamports: PriorityFee,
}

#[derive(Debug, Serialize)]
struct PriorityFee {
    priority_level_with_max_lamports: PriorityLevel,
}

#[derive(Debug, Serialize)]
struct PriorityLevel {
    max_lamports: u64,
    priority_level: String,
}

pub async fn get_buy_quote(
    input_mint: &str,
    amount: u64,
    slippage_bps: u16,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = format!(
        "{}?inputMint={}&outputMint={}&amount={}&slippageBps={}&restrictIntermediateTokens=true",
        JUP_QUOTE_URL,
        input_mint,
        SOL_MINT,
        amount,
        slippage_bps
    );

    let res = client.get(&url).send().await?;

    if !res.status().is_success() {
        return Err(format!("Quote HTTP {}", res.status()).into());
    }

    let quote: QuoteResponse = res.json().await?;

    if quote.route_plan.is_empty() {
        return Err("No valid route found".into());
    }

    Ok(quote)
}

pub async fn build_buy_swap_tx(
    quote: &QuoteResponse,
    user_pubkey: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let body = SwapRequest {
        quote_response: quote,
        user_public_key: user_pubkey.to_string(),
        dynamic_compute_unit_limit: true,
        dynamic_slippage: true,
        prioritization_fee_lamports: PriorityFee {
            priority_level_with_max_lamports: PriorityLevel {
                max_lamports: 1_000_000,
                priority_level: "veryHigh".to_string(),
            },
        },
    };

    let res = client
        .post(JUP_SWAP_URL)
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err = res.text().await?;
        return Err(format!("Swap API failed: {}", err).into());
    }

    let json: serde_json::Value = res.json().await?;

    let tx = json
        .get("swapTransaction")
        .and_then(|v| v.as_str())
        .ok_or("swapTransaction missing")?;

    Ok(tx.to_string())
}

pub fn sign_and_send_tx(
    rpc_url: &str,
    base64_tx: &str,
    keypair: &Keypair,
) -> Result<String, Box<dyn std::error::Error>> {
    let rpc = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    let tx_bytes = base64::decode(base64_tx)?;

    let tx: VersionedTransaction = bincode::deserialize(&tx_bytes)?;

    let message = tx.message.clone();

    let signers: &[&dyn Signer] = &[keypair];

    let signed_tx = VersionedTransaction::try_new(message, signers)?;

    let sig = rpc.send_and_confirm_transaction(&signed_tx)?;

    Ok(sig.to_string())
}
