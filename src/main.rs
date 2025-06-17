// Entry point of the system. Initializes logging, loads data, and runs the backtest.

mod logger;
mod data;
mod strategy;
mod execution;
mod backtest;

use log::info;

fn main() {
    logger::init_logger(); // Initialize logger
    info!("Starting Algo Trading System in Rust");

    let price_data = data::load_csv("data/prices.csv"); // Load market data
    backtest::run_backtest(&price_data); // Run backtest using loaded data
}
