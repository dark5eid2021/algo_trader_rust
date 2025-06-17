// Runs the backtest simulation: for each bar, apply strategy and simulate execution.

use crate::data::Bar;
use crate::execution::{execute_order, OrderResult};
use crate::strategy::{Signal, moving_average_strategy};
use log::{info, error};

// Process each signal, log results, and handle execution rejections.
pub fn run_backtest(data: &[Bar]) {
    let signals = moving_average_strategy(data, 5, 20);

    for (bar, signal) in data.iter().zip(signals.iter()) {
        match signal {
            Signal::Hold => continue,
            Signal::Buy => match execute_order("BUY") {
                OrderResult::Filled => info!("BUY at ${:.2} on {}", bar.close, bar.timestamp),
                OrderResult::Rejected(reason) => error!("Rejected: {}", reason),
            },
            Signal::Sell => match execute_order("SELL") {
                OrderResult::Filled => info!("SELL at ${:.2} on {}", bar.close, bar.timestamp),
                OrderResult::Rejected(reason) => error!("Rejected: {}", reason),
            },
        }
    }
}
