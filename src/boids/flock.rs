use super::boid::Boid;
use super::constants::*;
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


    pub fn next_step(&self) -> Self {
        Flock {
            boids: self.boids.iter().enumerate()
            .map(|(i, main_boid)| {
                (main_boid,
                self.boids.iter()
                    .enumerate()
                    .filter(move |(j, _)| *j != i)
                    .map(|(_, b)| b)
                )
            })
            .map(|(main_boid, other_boids)| {
                main_boid.time_step(other_boids)
            })
            .collect()
        }
    }
}