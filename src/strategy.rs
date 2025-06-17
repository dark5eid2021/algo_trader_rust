// Implements a basic moving average crossover strategy.

use crate::data::Bar;

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

// Generate trading signals using short/long moving average crossover.
pub fn moving_average_strategy(data: &[Bar], short: usize, long: usize) -> Vec<Signal> {
    let mut signals = Vec::new();

    for i in 0..data.len() {
        if i < long {
            signals.push(Signal::Hold); // Not enough data for MA
            continue;
        }

        let short_ma = data[i - short..i].iter().map(|b| b.close).sum::<f64>() / short as f64;
        let long_ma = data[i - long..i].iter().map(|b| b.close).sum::<f64>() / long as f64;

        let signal = if short_ma > long_ma {
            Signal::Buy
        } else if short_ma < long_ma {
            Signal::Sell
        } else {
            Signal::Hold
        };

        signals.push(signal);
    }

    signals
}
