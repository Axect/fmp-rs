use peroxide::fuga::*;
use crate::ta::*;

pub trait Strategy {
    /// Daily Return
    fn daily_return(&self) -> Vec<f64>;

    /// Cumulative Return
    fn cumulative_return(&self) -> Vec<f64> {
        let dr = self.daily_return();
        dr.into_iter().scan(1f64, |state, x| {
            *state *= 1f64 + x;
            Some(*state)
        }).collect::<Vec<f64>>()
    }

    /// Rolling Volatility
    fn roll_volatility(&self, period: usize) -> Vec<f64> {
        let dr = self.daily_return();
        let mut vol = vec![0f64; dr.len()];
        for i in period..dr.len() {
            vol[i] = dr[i - period..i].to_vec().sd() * (252 as f64).sqrt();
        }
        vol
    }

    /// Rolling Sharpe Ratio
    fn roll_sharpe_ratio(&self, risk_free: &[f64], period: usize) -> Vec<f64> {
        let dr = self.daily_return();
        let mut sr = vec![0f64; dr.len()];
        for i in period..dr.len() {
            let dr_roll = dr[i - period..i].to_vec();
            let rf = risk_free[i - period..i].to_vec();
            let excess_return = dr_roll.sub_v(&rf);
            sr[i] = excess_return.mean() / dr.sd() * (252 as f64).sqrt();
        }
        sr
    }

    /// Drawdown
    fn drawdown(&self) -> Vec<f64> {
        let cr = self.cumulative_return();
        let mut dd = vec![0f64; cr.len()];
        let mut max = 0f64;
        for i in 0..cr.len() {
            if cr[i] > max {
                max = cr[i];
            } else {
                dd[i] = (max - cr[i]) / max;
            }
        }
        dd
    }

    /// Volatility
    fn volatility(&self) -> f64 {
        let dr = self.daily_return();
        dr.sd() * (dr.len() as f64).sqrt()
    }

    /// Sharpe Ratio
    fn sharpe_ratio(&self, risk_free: &[f64]) -> f64 {
        let dr = self.daily_return();
        let rf = risk_free.to_vec();
        let excess_return = dr.sub_v(&rf);
        excess_return.mean() / excess_return.sd() * (252 as f64).sqrt()
    }

    /// Cumulative Annual Growth Rate
    fn cagr(&self) -> f64 {
        let cr = self.cumulative_return();
        let n = cr.len();
        (cr[n - 1].powf(1f64 / (n as f64)) - 1f64) * 100f64
    }

    /// Maximum Drawdown
    fn mdd(&self) -> f64 {
        let cr = self.cumulative_return();
        let mut max = 0f64;
        let mut mdd = 0f64;
        for i in 0..cr.len() {
            if cr[i] > max {
                max = cr[i];
            } else {
                let dd = (max - cr[i]) / max;
                if dd > mdd {
                    mdd = dd;
                }
            }
        }
        mdd
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

#[derive(Debug, Clone)]
pub struct MACrossover {
    pub ma1: usize,
    pub ma2: usize,
    pub close: Vec<f64>,
}

impl MACrossover {
    pub fn new(ma1: usize, ma2: usize, close: &[f64]) -> Self {
        Self {
            ma1,
            ma2,
            close: close.to_vec(),
        }
    }
}

impl Strategy for MACrossover {
    fn daily_return(&self) -> Vec<f64> {
        let mut res = vec![0f64; self.close.len()];
        let ma1 = sma(&self.close, self.ma1);
        let ma2 = sma(&self.close, self.ma2);
        for i in 1..self.close.len() {
            if ma1[i] > ma2[i] {
                res[i] = (self.close[i] - self.close[i - 1]) / self.close[i - 1];
            }
        }
        res
    }
}
