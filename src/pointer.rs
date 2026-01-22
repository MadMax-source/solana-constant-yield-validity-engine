// src/pointer.rs

#[derive(Debug, Clone)]
pub enum PointerMode {
    FixedUsd,     // e.g. $0.05 per step
    Percentage,   // e.g. 0.1% per step
}

#[derive(Debug, Clone)]
pub struct PointerConfig {
    pub mode: PointerMode,
    pub step_value: f64,
}

#[derive(Debug)]
pub struct Pointer {
    pub base_price: f64,
    pub last_price: f64,
    pub index: i64,
    pub config: PointerConfig,
}

#[derive(Debug)]
pub struct DirectionTracker {
    pub up: u32,
    pub down: u32,
}

impl DirectionTracker {
    pub fn new() -> Self {
        Self { up: 0, down: 0 }
    }

    pub fn update(&mut self, steps: i64) {
        if steps > 0 {
            self.up += steps as u32;
            self.down = 0;
        } else if steps < 0 {
            self.down += (-steps) as u32;
            self.up = 0;
        }
    }
}

impl Pointer {
    pub fn new(start_price: f64, config: PointerConfig) -> Self {
        Self {
            base_price: start_price,
            last_price: start_price,
            index: 0,
            config,
        }
    }

    fn step_size(&self) -> f64 {
        match self.config.mode {
            PointerMode::FixedUsd => self.config.step_value,
            PointerMode::Percentage => self.last_price * self.config.step_value,
        }
    }

    /// Update pointer with new price
    /// Returns number of steps moved (+ / -)
    pub fn update(&mut self, new_price: f64) -> i64 {
        let step = self.step_size();
        let diff = new_price - self.last_price;

        let steps_moved = (diff / step).trunc() as i64;

        if steps_moved != 0 {
            self.index += steps_moved;
            self.last_price = new_price;
        }

        steps_moved
    }
}
