use reqwest::Client;

pub struct LiveTrader {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LiveTrader {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn execute_trade(&self, symbol: &str, quantity: f64, action: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/orders", self.base_url);

        let body = serde_json::json!({
            "symbol": symbol,
            "qty": quantity,
            "side": action,
            "type": "market",
            "time_in_force": "gtc"
        });

        self.client
            .post(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}
