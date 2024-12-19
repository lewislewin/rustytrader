use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StockData {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub struct DataFetcher {
    client: Client,
    api_key: String,
    base_url: String,
}

impl DataFetcher {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn fetch_stock_data(&self, symbol: &str) -> Result<Vec<StockData>, reqwest::Error> {
        let url = format!(
            "{}/stock/candle?symbol={}&resolution=1&from=1609459200&to=1672531200&token={}",
            self.base_url, symbol, self.api_key
        );

        let response = self.client.get(&url).send().await?.json::<Vec<StockData>>().await?;
        Ok(response)
    }
}
