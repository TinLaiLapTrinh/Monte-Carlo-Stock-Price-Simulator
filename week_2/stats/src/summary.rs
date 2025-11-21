pub fn log_returns(prices: &[f64]) -> Vec<f64> {
    prices.windows(2)
        .map(|w| (w[1] / w[0]).ln())
        .collect()
}

pub fn mean_std(returns: &[f64]) -> (f64, f64) {
    let n = returns.len() as f64;
    let mean = returns.iter().sum::<f64>() / n;
    let var = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (n - 1.0);
    (mean, var.sqrt())
}
