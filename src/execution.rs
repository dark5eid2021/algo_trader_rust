// Simulates an order execution engine with rejection logic.

use log::{info, warn};
use rand::Rng;

pub enum OrderResult {
    Filled,
    Rejected(String),
}

// Simulate order execution with a 10% chance of rejection.
pub fn execute_order(signal: &str) -> OrderResult {
    let mut rng = rand::thread_rng();
    let chance: f64 = rng.gen(); // 0.0 to 1.0

    if chance < 0.9 {
        info!("Order executed: {}", signal);
        OrderResult::Filled
    } else {
        let reason = format!("Order for '{}' rejected due to simulated market issue.", signal);
        warn!("{}", reason);
        OrderResult::Rejected(reason)
    }
}
