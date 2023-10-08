use peroxide::fuga::*;

/// Simple Moving Average
pub fn sma(v: &[f64], period: usize) -> Vec<f64> {
    let mut res = vec![0f64; v.len()];
    let mut sum = 0.0;
    for i in 0..v.len() {
        sum += v[i];
        if i >= period {
            sum -= v[i - period];
            res[i] = sum / period as f64;
        } else {
            res[i] = sum / (i + 1) as f64;
        }
    }
    res
}

/// Exponential Moving Average
pub fn ema(v: &[f64], period: usize) -> Vec<f64> {
    let mut res = vec![0f64; v.len()];
    res[0] = v[0];
    let mut ema = v[0];
    let alpha = 2.0 / (period as f64 + 1.0);
    for i in 1..v.len() {
        ema = v[i] * alpha + ema * (1.0 - alpha);
        res[i] = ema;
    }
    res
}

/// Weighted Moving Average
pub fn wma(v: &[f64], period: usize) -> Vec<f64> {
    let mut res = vec![0f64; v.len()];
    let denom = ((period * (period + 1)) / 2) as f64;
    let mut total = 0f64;
    let mut numer = 0f64;
    for i in 0..v.len() {
        if i >= period {
            numer = numer + period as f64 * v[i] - total;
            total = total + v[i] - v[i - period];
            res[i] = numer / denom;
        } else {
            numer = numer + (i + 1) as f64 * v[i];
            total = total + v[i];
            res[i] = numer / (((i + 1) * (i + 2)) / 2) as f64;
        }
    }
    res
}

/// Double Exponential Moving Average
pub fn dema(v: &[f64], period: usize) -> Vec<f64> {
    let ema_ = ema(v, period);
    let ema_ema = ema(&ema_, period);
    zip_with(|x, y| 2.0 * x - y, &ema_, &ema_ema)
}

/// Triple Exponential Moving Average
pub fn tema(v: &[f64], period: usize) -> Vec<f64> {
    let ema_ = ema(v, period);
    let ema_ema = ema(&ema_, period);
    let ema_ema_ema = ema(&ema_ema, period);
    ema_.iter()
        .zip(ema_ema.iter())
        .zip(ema_ema_ema.iter())
        .map(|((x, y), z)| 3.0 * x - 3.0 * y + z)
        .collect::<Vec<f64>>()
}

/// Williams %R
pub fn williams_r(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
    let mut res = vec![0f64; high.len()];
    for i in 0..high.len() {
        let mut highest = high[i];
        let mut lowest = low[i];
        for j in 1..period {
            if i >= j {
                highest = highest.max(high[i - j]);
                lowest = lowest.min(low[i - j]);
            }
        }
        res[i] = (highest - close[i]) / (highest - lowest) * -100.0;
    }
    res
}

/// Smoothed Moving Average
pub fn smma(v: &[f64], period: usize) -> Vec<f64> {
    ema(v, 2 * period - 1)
}

/// Relative Strength Index
pub fn rsi(v: &[f64], period: usize) -> Vec<f64> {
    let mut u = vec![0f64; v.len()];
    let mut d = vec![0f64; v.len()];
    for i in 1..v.len() {
        let diff = v[i] - v[i - 1];
        if diff > 0.0 {
            u[i] = diff;
        } else {
            d[i] = -diff;
        }
    }
    let au = smma(&u, period);
    let ad = smma(&d, period);
    zip_with(|x, y| 100f64 * x / (x + y + 1e-3), &au, &ad)
}

/// Moving Average Convergence Divergence
pub fn macd(v: &[f64], period1: usize, period2: usize) -> Vec<f64> {
    let ema1 = ema(v, period1);
    let ema2 = ema(v, period2);
    zip_with(|x, y| x - y, &ema1, &ema2)
}
