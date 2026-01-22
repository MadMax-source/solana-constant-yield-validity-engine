use chrono::Utc;
use crate::constants::BUY_IN_SOL;
use crate::batch::{get_or_create_active_batch, add_hand_to_batch, rotate_batch_if_needed};

#[derive(Debug, Clone)]
pub struct Hand {
    pub entry_sol_price_usd: f64,
    pub buy_in_usd: f64,
    pub buy_in_sol: f64,
    pub opened_at: String,
    pub batch_id: usize,
}

pub fn create_hand(entry_price_usd: f64) -> Hand {
    let batch_id = get_or_create_active_batch();

    let hand = Hand {
        entry_sol_price_usd: entry_price_usd,
        buy_in_usd: entry_price_usd * BUY_IN_SOL,
        buy_in_sol: BUY_IN_SOL,
        opened_at: Utc::now().to_rfc3339(),
        batch_id,
    };

    add_hand_to_batch(hand.clone());
    rotate_batch_if_needed();

    hand
}
