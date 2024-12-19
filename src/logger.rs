use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_trade(trade: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("trades.log")
        .unwrap();

    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "[{}] {}", now, trade).unwrap();
}
