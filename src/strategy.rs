use peroxide::fuga::*;
use crate::ta::*;

pub trait Strategy {
    fn daily_return(&self) -> Vec<f64>;
    fn cumulative_return(&self) -> Vec<f64> {
        let dr = self.daily_return();
        dr.into_iter().scan(1f64, |state, x| {
            *state *= 1f64 + x;
            Some(*state)
        }).collect::<Vec<f64>>()
    }
    fn volatility(&self, period: usize) -> Vec<f64> {
        let dr = self.daily_return();
        let mut vol = vec![0f64; dr.len()];
        for i in period..dr.len() {
            vol[i] = dr[i - period..i].to_vec().sd() * (period as f64).sqrt();
        }
        vol
    }
    fn sharpe_ratio(&self, period: usize) -> Vec<f64> {
        let dr = self.daily_return();
        let mut sr = vec![0f64; dr.len()];
        for i in period..dr.len() {
            let dr_roll = dr[i - period..i].to_vec();
            sr[i] = dr_roll.mean() / dr_roll.sd();
        }
        sr
    }
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
