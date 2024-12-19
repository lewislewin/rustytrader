pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub initial_balance: f64,
    pub fetch_interval_secs: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("API_KEY").unwrap_or_else(|_| "demo".to_string()),
            base_url: "https://finnhub.io/api/v1".to_string(),
            initial_balance: 10_000.0,
            fetch_interval_secs: 5,
        }
    }
}
