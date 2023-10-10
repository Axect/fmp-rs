use peroxide::fuga::*;
use crate::ta::*;

pub trait Strategy {
    fn daily_return(&self) -> Vec<f64>;
}

#[derive(Debug, Clone)]
pub struct BuyAndHold {
    pub init: f64,
    pub close: Vec<f64>,
}

impl BuyAndHold {
    pub fn new(init: f64, close: &[f64]) -> Self {
        Self {
            init,
            close: close.to_vec(),
        }
    }
}

impl Strategy for BuyAndHold {
    fn daily_return(&self) -> Vec<f64> {
        let mut res = vec![0f64; self.close.len()];
        res[0] = (self.close[0] - self.init) / self.init;
        for i in 1..self.close.len() {
            res[i] = (self.close[i] - self.close[i - 1]) / self.close[i - 1];
        }
        res
    }
}
