pub mod  summary;
pub fn summary(terminal_prices: &[f64]) -> Summary {
    let mut sorted = terminal_prices.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
    let std = (sorted.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (sorted.len() - 1) as f64).sqrt();

    let p5 = sorted[(0.05 * sorted.len() as f64) as usize];
    let p95 = sorted[(0.95 * sorted.len() as f64) as usize];
    let var95 = mean - p5;

    Summary { mean, std, p5, p95, var95 }
}

pub struct Summary {
    pub mean: f64,
    pub std: f64,
    pub p5: f64,
    pub p95: f64,
    pub var95: f64,
}
