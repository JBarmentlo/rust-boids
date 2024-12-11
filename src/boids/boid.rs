use std::ops::{Add, Sub, Div, Mul};
use rand::Rng;
use super::constants::*;
use crate::vec2d::Vec2D;


#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub position: Vec2D,
    pub velocity: Vec2D,
}

impl Boid {
    /// Constructs a new Boid with random position and velocity within given ranges.
    pub fn random_range(min_position: Vec2D, max_position: Vec2D, min_velocity: Vec2D, max_velocity: Vec2D) -> Self {
        let mut rng = rand::thread_rng();

        let position = Vec2D {
            x: rng.gen_range(min_position.x..=max_position.x),
            y: rng.gen_range(min_position.y..=max_position.y),
        };

        let velocity = Vec2D {
            x: rng.gen_range(min_velocity.x..=max_velocity.x),
            y: rng.gen_range(min_velocity.y..=max_velocity.y),
        };

        Boid { position, velocity }
    }

    pub fn random() -> Self {
        Self::random_range(
            Vec2D { x: EDGE_MARGIN               ,       y: EDGE_MARGIN }  ,
            Vec2D { x: WORLD_SIZE_X as f32 - EDGE_MARGIN, y: WORLD_SIZE_Y as f32 - EDGE_MARGIN },
            Vec2D { x: MIN_SPEED / 2_f32.sqrt()  ,       y: MIN_SPEED / 2_f32.sqrt() } ,
            Vec2D { x: MAX_SPEED / 2_f32.sqrt()  ,       y: MIN_SPEED / 2_f32.sqrt() }  ,
        )
    }

    // fn find_min<'a, I>(vals: I) -> Option<&'a u32>
    // where
    //     I: Iterator<Item = &'a u32>,
    // I: IntoIterator<Item = &'a u32>,
    // {
    //     vals.min()
    // }

    pub fn time_step<'a, I>(&self, others: I) -> Self 
    where
        I: Iterator<Item= &'a Self> // TODO: understand lifetimes
    {
        Self {
            position: Vec2D {
                x: self.position.x + 10.,
                y: self.position.y,
            },
            velocity: self.velocity,
        }
    }

    pub fn baby_step(&self) -> Self 
    {
        Self {
            position: Vec2D {
                x: self.position.x + 10.,
                y: self.position.y,
            },
            velocity: self.velocity,
        }
    }
}

impl Add for Boid {
    type Output = Boid;

    fn add(self, other: Boid) -> Boid {
        Boid {
            position: self.position + other.position,
            velocity: self.velocity + other.velocity,
        }
    }
}

impl Sub for Boid {
    type Output = Boid;

    fn sub(self, other: Boid) -> Boid {
        Boid {
            position: self.position - other.position,
            velocity: self.velocity - other.velocity,
        }
    }
}


impl Div<f32> for Boid {
    type Output = Boid;

    fn div(self, scalar: f32) -> Boid {
        Boid {
            position: self.position / scalar,
            velocity: self.velocity / scalar,
        }
    }
}

impl Mul<f32> for Boid {
    type Output = Boid;

    fn mul(self, scalar: f32) -> Boid {
        Boid {
            position: self.position * scalar,
            velocity: self.velocity * scalar,
        }
    }
}

