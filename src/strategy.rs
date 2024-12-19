use crate::data_fetcher::StockData;

pub trait TradingStrategy {
    fn decide(&self, data: &[StockData]) -> Option<TradeDecision>;
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
        let close_prices: Vec<f64> = data.iter().map(|d| d.close).collect();

        let short_ma = self.moving_average(&close_prices, self.short_window);
        let long_ma = self.moving_average(&close_prices, self.long_window);

        if short_ma.last() > long_ma.last() {
            Some(TradeDecision::Buy)
        } else if short_ma.last() < long_ma.last() {
            Some(TradeDecision::Sell)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum TradeDecision {
    Buy,
    Sell,
    Hold,
}
