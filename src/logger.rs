use std::fs::OpenOptions;
use std::io::Write;

pub fn log_trade(trade: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("trades.log")
        .unwrap();

    writeln!(file, "{}", trade).unwrap();
}
