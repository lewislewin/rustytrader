use crate::strategy::TradeDecision;

pub struct Simulator {
    balance: f64,
    position: f64,
}

impl Simulator {
    pub fn new(balance: f64) -> Self {
        Self {
            balance,
            position: 0.0,
        }
    }

    pub fn execute_trade(&mut self, decision: TradeDecision, price: f64) {
        match decision {
            TradeDecision::Buy => {
                if self.balance > 0.0 {
                    let amount = self.balance / price;
                    self.position += amount;
                    self.balance -= amount * price;
                }
            }
            TradeDecision::Sell => {
                if self.position > 0.0 {
                    self.balance += self.position * price;
                    self.position = 0.0;
                }
            }
            TradeDecision::Hold => {}
        }
    }

    pub fn get_balance(&self, current_price: f64) -> f64 {
        self.balance + (self.position * current_price)
    }
}
