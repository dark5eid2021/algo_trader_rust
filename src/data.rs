// Reads historical price data from a CSV file into a vector of Bar structs.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bar {
    pub timestamp: String,
    pub close: f64,
}

// Load CSV file and return vector of Bar structs.
pub fn load_csv(file_path: &str) -> Vec<Bar> {
    let mut rdr = csv::Reader::from_path(file_path).expect("Cannot read CSV");
    rdr.deserialize()
        .filter_map(Result::ok)
        .collect()
}
