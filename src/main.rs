use populational_heuristics::pso::search;
use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let max_iter: u32 = 1000;
    let pos_bound: f64 = 5.12;
    let vel_bound: f64 = (2.0 * pos_bound) * 0.1;
    let particle_size: usize = 2;
    let pop_size: u32 = 40;
    let local_bias: f64 = 1.0;
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
