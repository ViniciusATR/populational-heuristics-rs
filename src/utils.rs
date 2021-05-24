use rand::{distributions::Uniform, rngs::StdRng, Rng};
use std::f64::consts::PI;

pub fn random_vec(rng: &mut StdRng, size: &usize, range: &f64) -> Vec<f64> {
    let uni = Uniform::from(-range..*range);
    let vec: Vec<f64> = rng.sample_iter(&uni).take(*size).collect();
    vec
}

pub fn rastrigin(vec: &Vec<f64>) -> f64 {
    let mut inner_sum: f64 = 0.0;
    for x in vec.iter() {
        inner_sum += 10.0 + x.powi(2) - 10.0 * (2.0 * PI * x).cos();
    }

    inner_sum
}

pub fn func_rastrigin(vec: &Vec<f64>) -> f64 {
    vec.iter().fold(0.0, |acc, x| {
        acc + 10.0 + x.powi(2) - 10.0 * (2.0 * PI * x).cos()
    })
}
