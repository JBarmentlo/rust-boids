use super::boid::Boid;
use std::iter;

#[derive(Debug)]
pub struct Flock {
    pub boids: Vec<Boid>,
}

impl Flock {
    pub fn random(number_of_boids: usize) -> Self {
        Flock {
            boids: iter::repeat_with(Boid::random).take(number_of_boids).collect()
        }
    }
}