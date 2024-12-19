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

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    AddStock(String),
    RemoveStock(String),
    List,
    Exit,
}

struct StockInfo {
    simulator: Simulator,
    strategy: Box<dyn TradingStrategy>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::new();

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(32);
    let data_fetcher = DataFetcher::new(config.api_key.clone(), config.base_url.clone());

    tokio::spawn(async move {
        let mut stocks: HashMap<String, StockInfo> = HashMap::new();

        loop {
            tokio::select! {
                Some(cmd) = cmd_rx.recv() => {
                    match cmd {
                        Command::AddStock(symbol) => {
                            let sim = Simulator::new(10_000.0);
                            let strat = Box::new(MovingAverageStrategy::new(10, 50));
                            stocks.insert(symbol, StockInfo { simulator: sim, strategy: strat });
                            println!("Added stock");
                        },
                        Command::RemoveStock(symbol) => {
                            stocks.remove(&symbol);
                            println!("Removed stock");
                        },
                        Command::List => {
                            if stocks.is_empty() {
                                println!("No stocks being tracked.");
                            } else {
                                println!("Currently tracking: {}", stocks.keys().cloned().collect::<Vec<_>>().join(", "));
                            }
                        },
                        Command::Exit => {
                            println!("Exiting simulation...");
                            break;
                        },
                    }
                }

                _ = tokio::time::sleep(std::time::Duration::from_secs(config.fetch_interval_secs)) => {
                    for (symbol, info) in stocks.iter_mut() {
                        match data_fetcher.fetch_stock_data(symbol).await {
                            Ok(data) if !data.is_empty() => {
                                if let Some(decision) = info.strategy.decide(&data) {
                                    let last_price = data.last().map(|d| d.close).unwrap_or(100.0);
                                    info.simulator.execute_trade(decision.clone(), last_price);
                                    let balance = info.simulator.get_balance(last_price);
                                    let msg = format!("Symbol: {}, Decision: {:?}, Balance: {:.2}", symbol, decision, balance);
                                    log_trade(&msg);
                                    println!("{}", msg);
                                }
                            }
                            Ok(_) => {
                                println!("No data returned for {} at this time.", symbol);
                            }
                            Err(e) => {
                                println!("Error fetching data for {}: {}", symbol, e);
                            }
                        }
                    }
                }
            }
        }
    });

    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    println!("Commands: add <symbol>, remove <symbol>, list, exit");
    println!("Type 'add AAPL' to start tracking AAPL, for example.");

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["add", symbol] => {
                cmd_tx.send(Command::AddStock(symbol.to_string())).await?;
            }
            ["remove", symbol] => {
                cmd_tx.send(Command::RemoveStock(symbol.to_string())).await?;
            }
            ["list"] => {
                cmd_tx.send(Command::List).await?;
            }
            ["exit"] => {
                cmd_tx.send(Command::Exit).await?;
                break;
            }
            _ => {
                println!("Unknown command");
            }
        }
    }

    Ok(())
}
