use crate::utils::{func_rastrigin, random_vec};
use rand::{rngs::StdRng, Rng};

#[derive(Debug, Clone)]
pub struct Particle {
    position: Vec<f64>,
    best_position: Vec<f64>,
    velocity: Vec<f64>,
    cost: f64,
    best_cost: f64,
}

fn sum_with_bounds(a: &f64, b: &f64, bound: &f64) -> f64 {
    let sum = a + b;
    if sum > *bound {
        *bound
    } else if sum < -bound {
        -bound
    } else {
        sum
    }
}

impl Particle {
    pub fn new_random(
        size: &usize,
        position_range: &f64,
        velocity_range: &f64,
        rng: &mut StdRng,
    ) -> Particle {
        let pos = random_vec(rng, size, position_range);
        let vel = random_vec(rng, size, velocity_range);
        Particle {
            position: pos.clone(),
            best_position: pos.clone(),
            velocity: vel,
            cost: func_rastrigin(&pos),
            best_cost: func_rastrigin(&pos),
        }
    }

    pub fn hit_boundaries(&mut self, bound: &f64) {
        for (idx, _x) in self.position.iter().enumerate() {
            if (self.position[idx] >= *bound) || (self.position[idx] <= -bound) {
                self.velocity[idx] *= -1.0
            }
        }
    }

    pub fn update_pos_cost(&mut self, bound: &f64) {
        self.position = self
            .position
            .iter()
            .zip(&self.velocity)
            .map(|(a, b)| a + b)
            .collect();
        self.hit_boundaries(bound);
        self.cost = func_rastrigin(&self.position)
    }

    pub fn update_vel(
        &mut self,
        rng: &mut StdRng,
        gbest: &Particle,
        v_bound: &f64,
        lbias: &f64,
        gbias: &f64,
    ) {
        for (idx, x) in self.position.iter().enumerate() {
            let local_coef = lbias * rng.gen_range(0.0..1.0) * (self.best_position[idx] - x);
            let global_coef = gbias * rng.gen_range(0.0..1.0) * (gbest.position[idx] - x);
            self.velocity[idx] =
                sum_with_bounds(&self.velocity[idx], &(local_coef + global_coef), v_bound);
        }
    }

    pub fn update_best_position(&mut self) {
        if self.cost > self.best_cost {
            {}
        } else {
            self.best_cost = self.cost;
            self.best_position = self.position.clone();
        }
    }
}

pub fn get_best_particle(population: &mut Vec<Particle>) -> Particle {
    population.sort_by(|a, b| b.cost.partial_cmp(&a.cost).unwrap());
    population.last().unwrap().clone()
}

pub fn search(
    rng: &mut StdRng,
    max_iter: u32,
    pos_bound: f64,
    vel_bound: f64,
    particle_size: usize,
    pop_size: u32,
    local_bias: f64,
    global_bias: f64,
) -> Particle {
    let mut population: Vec<Particle> = Vec::new();
    for _p in 1..pop_size {
        population.push(Particle::new_random(
            &particle_size,
            &pos_bound,
            &vel_bound,
            rng,
        ))
    }
    let mut best_particle = get_best_particle(&mut population);
    for _i in 1..max_iter {
        population.iter_mut().for_each(|a| {
            a.update_vel(rng, &best_particle, &vel_bound, &local_bias, &global_bias);
            a.update_pos_cost(&pos_bound);
            a.update_best_position();
        });
        best_particle = get_best_particle(&mut population);
        println!("Current best {}", best_particle.cost);
    }
    best_particle
}
