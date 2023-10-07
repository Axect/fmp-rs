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
