use populational_heuristics::pso::search;
use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let max_iter: u32 = 100;
    let pos_bound: f64 = 5.12;
    let vel_bound: f64 = 0.02;
    let particle_size: usize = 5;
    let pop_size: u32 = 20;
    let local_bias: f64 = 10.0;
    let global_bias: f64 = 1.0;

    let part = search(
        &mut rng,
        max_iter,
        pos_bound,
        vel_bound,
        particle_size,
        pop_size,
        local_bias,
        global_bias,
    );
    println!("This is my particle {:?}", part);
}
