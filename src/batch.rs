use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::hand::Hand;

#[derive(Debug, Clone)]
pub struct Batch {
    pub id: usize,
    pub hands: Vec<Hand>,
}

static BATCHES: Lazy<Mutex<Vec<Batch>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

static ACTIVE_BATCH_INDEX: Lazy<Mutex<usize>> =
    Lazy::new(|| Mutex::new(0));

static LOCKED: Lazy<Mutex<bool>> =
    Lazy::new(|| Mutex::new(false));



pub fn lock() {
    *LOCKED.lock().unwrap() = true;
}

pub fn unlock() {
    *LOCKED.lock().unwrap() = false;
}

pub fn is_locked() -> bool {
    *LOCKED.lock().unwrap()
}

pub fn get_batches() -> Vec<Batch> {
    BATCHES.lock().unwrap().clone()
}

pub fn active_batch_id() -> usize {
    *ACTIVE_BATCH_INDEX.lock().unwrap()
}


pub fn get_or_create_active_batch() -> usize {
    let mut batches = BATCHES.lock().unwrap();
    let mut active = ACTIVE_BATCH_INDEX.lock().unwrap();

    if batches.is_empty() {
        batches.push(Batch {
            id: 0,
            hands: Vec::new(),
        });
        *active = 0;
    }

    *active
}

pub fn add_hand_to_batch(hand: Hand) {
    if is_locked() {
        return;
    }

    let mut batches = BATCHES.lock().unwrap();
    let active = *ACTIVE_BATCH_INDEX.lock().unwrap();

    if let Some(batch) = batches.get_mut(active) {
        batch.hands.push(hand.clone());
        println!(
            "🖐️ Hand added → batch {} ({} / 10)",
            batch.id,
            batch.hands.len()
        );
    }
}

pub fn rotate_batch_if_needed() {
    let mut batches = BATCHES.lock().unwrap();
    let mut active = ACTIVE_BATCH_INDEX.lock().unwrap();

    // Check the active batch via index instead of get()
    if *active < batches.len() {
        let hands_len = batches[*active].hands.len();
        let batch_id = batches[*active].id;

        if hands_len >= 10 {
            let new_id = batches.len();
            batches.push(Batch { id: new_id, hands: Vec::new() });
            *active = new_id;

            println!("📦 Batch {} closed → New batch {}", batch_id, new_id);
        }
    }
}
