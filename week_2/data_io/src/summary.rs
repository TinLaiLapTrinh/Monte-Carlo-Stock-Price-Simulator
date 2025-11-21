use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Summary {
    pub mean: f64,
    pub std: f64,
    pub median: f64,
    pub p5: f64,
    pub p25: f64,
    pub p75: f64,
    pub p95: f64,
}

pub fn summary(values: &[f64]) -> Summary {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let std = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt();

    let get_pct = |p: f64| {
        let idx = ((p / 100.0) * (v.len() as f64 - 1.0)).round() as usize;
        v[idx]
    };

    Summary {
        mean,
        std,
        median: get_pct(50.0),
        p5: get_pct(5.0),
        p25: get_pct(25.0),
        p75: get_pct(75.0),
        p95: get_pct(95.0),
    }
}
