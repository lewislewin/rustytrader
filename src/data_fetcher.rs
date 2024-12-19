use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct StockData {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Deserialize)]
struct FinnhubCandle {
    c: Option<Vec<f64>>, // Close prices
    h: Option<Vec<f64>>, // High prices
    l: Option<Vec<f64>>, // Low prices
    o: Option<Vec<f64>>, // Open prices
    t: Option<Vec<i64>>, // Timestamps
    v: Option<Vec<f64>>, // Volumes
    s: String,           // Status
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
    
        let raw_response = self.client.get(&url).send().await?.text().await?;
    
        if raw_response.contains("\"error\"") {
            println!("API error for {}: {}", symbol, raw_response);
            return Ok(Vec::new());
        }
    
        let response: Result<FinnhubCandle, _> = serde_json::from_str(&raw_response);
    
        match response {
            Ok(parsed) => {
                if parsed.s != "ok" {
                    println!("API returned a non-ok status for {}: {}", symbol, parsed.s);
                    return Ok(Vec::new());
                }
    
                let (c, h, l, o, t, v) = match (
                    parsed.c,
                    parsed.h,
                    parsed.l,
                    parsed.o,
                    parsed.t,
                    parsed.v,
                ) {
                    (Some(c), Some(h), Some(l), Some(o), Some(t), Some(v)) if c.len() == t.len() => {
                        (c, h, l, o, t, v)
                    }
                    _ => {
                        println!("Incomplete or mismatched data for {}", symbol);
                        return Ok(Vec::new());
                    }
                };
    
                let stock_data: Vec<StockData> = t
                    .into_iter()
                    .enumerate()
                    .map(|(i, timestamp)| StockData {
                        timestamp: timestamp.to_string(),
                        open: o[i],
                        high: h[i],
                        low: l[i],
                        close: c[i],
                        volume: v[i] as u64,
                    })
                    .collect();
    
                Ok(stock_data)
            }
            Err(e) => {
                println!("Failed to parse JSON response for {}: {}", symbol, e);
                Ok(Vec::new()) 
            }
        }
    }    
}
