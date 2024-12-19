pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub initial_balance: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("API_KEY").expect("API_KEY not set"),
            base_url: "https://finnhub.io/api/v1".to_string(),
            initial_balance: 10_000.0,
        }
    }
}
