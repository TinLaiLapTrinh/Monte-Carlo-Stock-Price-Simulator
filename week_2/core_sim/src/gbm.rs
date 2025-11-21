use rand::SeedableRng;
use rand_distr::{Normal, Distribution};

pub fn simulate_gbm(
    s0: f64, mu: f64, sigma: f64,
    dt: f64, horizon: usize, paths: usize, seed: u64
) -> Vec<Vec<f64>> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut results = Vec::with_capacity(paths);

    for _ in 0..paths {
        let mut prices = Vec::with_capacity(horizon);
        let mut s = s0;
        for _ in 0..horizon {
            let z = normal.sample(&mut rng);
            s *= f64::exp((mu - 0.5 * sigma.powi(2)) * dt + sigma * dt.sqrt() * z);
            prices.push(s);
        }
        results.push(prices);
    }
    results
}
