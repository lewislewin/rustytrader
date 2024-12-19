mod config;
mod data_fetcher;
mod strategy;
mod simulator;
mod logger;

use config::Config;
use data_fetcher::DataFetcher;
use strategy::{MovingAverageStrategy, TradingStrategy};
use simulator::Simulator;
use logger::log_trade;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::new();

    let data_fetcher = DataFetcher::new(config.api_key.clone(), config.base_url.clone());
    let strategy = MovingAverageStrategy::new(10, 50);
    let mut simulator = Simulator::new(config.initial_balance);

    let data = data_fetcher.fetch_stock_data("AAPL").await?;
    println!("Fetched {} data points", data.len());

    if let Some(decision) = strategy.decide(&data) {
        simulator.execute_trade(decision.clone(), data.last().unwrap().close);
        let balance = simulator.get_balance();

        log_trade(&format!("Decision: {:?}, Balance: {}", decision, balance));
        println!("Decision: {:?}, Balance: {}", decision, balance);
    }

    Ok(())
}
