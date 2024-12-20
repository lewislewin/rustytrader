use crate::data_fetcher::StockData;

pub trait TradingStrategy: Send + Sync {
    fn decide(&self, data: &[StockData]) -> Option<TradeDecision>;
}

#[derive(Debug, Clone)]
pub enum TradeDecision {
    Buy,
    Sell,
    Hold,
}

pub struct MovingAverageStrategy {
    short_window: usize,
    long_window: usize,
}

impl MovingAverageStrategy {
    pub fn new(short_window: usize, long_window: usize) -> Self {
        Self {
            short_window,
            long_window,
        }
    }

    fn moving_average(&self, data: &[f64], window: usize) -> Vec<f64> {
        data.windows(window).map(|w| w.iter().sum::<f64>() / window as f64).collect()
    }
}

impl TradingStrategy for MovingAverageStrategy {
    fn decide(&self, data: &[StockData]) -> Option<TradeDecision> {
        if data.len() < self.long_window {
            return None;
        }

        let close_prices: Vec<f64> = data.iter().map(|d| d.close).collect();
        let short_ma = self.moving_average(&close_prices, self.short_window);
        let long_ma = self.moving_average(&close_prices, self.long_window);

        if let (Some(&s), Some(&l)) = (short_ma.last(), long_ma.last()) {
            if s > l {
                Some(TradeDecision::Buy)
            } else if s < l {
                Some(TradeDecision::Sell)
            } else {
                Some(TradeDecision::Hold)
            }
        } else {
            None
        }
    }
}
