pub fn sma(v: &[f64], period: usize) -> Vec<f64> {
    let mut res = Vec::new();
    let mut sum = 0.0;
    for i in 0..v.len() {
        sum += v[i];
        if i >= period {
            sum -= v[i - period];
            res.push(sum / period as f64);
        } else {
            res.push(sum / (i + 1) as f64);
        }
    }
    res
}
