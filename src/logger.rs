// Custom logger setup using simplelog.
// Logs go to both console and file.

use simplelog::*;
use std::fs::File;

pub fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("algo.log").unwrap()),
    ])
    .unwrap();
}
