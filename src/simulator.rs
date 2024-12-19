use crate::strategy::TradeDecision;

pub struct Simulator {
    balance: f64,
    position: f64,
}

impl Simulator {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            position: 0.0,
        }
    }

    pub fn execute_trade(&mut self, decision: TradeDecision, price: f64) {
        match decision {
            TradeDecision::Buy => {
                let amount = self.balance / price;
                self.position += amount;
                self.balance -= amount * price;
            }
            TradeDecision::Sell => {
                self.balance += self.position * price;
                self.position = 0.0;
            }
            TradeDecision::Hold => {}
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance + (self.position * 100.0)
    }
}
