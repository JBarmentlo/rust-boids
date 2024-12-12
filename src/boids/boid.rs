use std::ops::{Add, Sub, Div, Mul};
use rand::Rng;
use std::fmt;


use super::constants::*;
use crate::vec2d::Vec2D;


#[derive(Debug, Clone, Copy, Default)]
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

    pub fn distance(&self, other: &Boid) -> f32 {
        self.position.distance(&other.position)
    }


    pub fn time_step<'a, I>(&self, others: I) -> Self 
    where
        I: Iterator<Item= &'a Self> // TODO: understand lifetimes
    {
        let other_vec: Vec<&Boid> = others.collect();
        // println!("other boids: {}", other_vec.len());

        let visible   = other_vec.iter().filter(|b| self.distance(b) < PERCEPTION_RANGE).map(|b| *b);
        let avoidable = other_vec.iter().filter(|b| self.distance(b) < AVOIDANCE_RANGE).map(|b| *b);

        let (sum, visible_count) = visible.fold((Boid::default(), 0), |(b, c), x| (b + *x, c + 1));
        // println!("Sum: {}, Count: {}" , sum, visible_count);
        let average_visible = sum / visible_count as f32;

        let (sum, avoid_count) = avoidable.fold((Boid::default(), 0), |(b, c), x| (b + *x, c + 1));
        let average_avoidable = sum / avoid_count as f32;

        let avoid_vec    = self.position - average_avoidable.position;
        let aling_vec    = self.position - average_visible.velocity;
        let cohesion_vec = average_visible.position - self.position;
        
        // println!("cohesion_vec: {}", cohesion_vec * 0.);
        // println!("avoid_vec: {}", avoid_vec);

        let mut velocity = self.velocity;
    
        if visible_count > 0 {
            velocity = velocity * (1. - COHESION_FACTOR  ) +  cohesion_vec * COHESION_FACTOR;
            velocity = velocity * (1. - ALIGNEMENT_FACTOR) +  aling_vec    * ALIGNEMENT_FACTOR;
        }

            
        if avoid_count > 0 {
            velocity = velocity * (1. - AVOIDANCE_FACTOR) +  avoid_vec * AVOIDANCE_FACTOR;
        }

        if self.position.x < EDGE_MARGIN {
            velocity.x += EDGE_TURN_FACTOR;
        } else if self.position.x > WORLD_SIZE_X as f32 - EDGE_MARGIN {
            velocity.x -= EDGE_TURN_FACTOR;
        }

        if self.position.y < EDGE_MARGIN {
            velocity.y += EDGE_TURN_FACTOR;
        } else if self.position.y > WORLD_SIZE_Y as f32 - EDGE_MARGIN {
            velocity.y -= EDGE_TURN_FACTOR;
        }

        let speed = velocity.norm_2();
        if speed < MIN_SPEED {
            velocity = velocity * (MIN_SPEED / speed);
        } else if speed > MAX_SPEED {
            velocity = velocity * (MAX_SPEED / speed);
        }

        Self {
            position: self.position + velocity,
            velocity: velocity
        }
    }
}


impl fmt::Display for Boid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Boid {{ p: ({}, {}), v: ({}, {}) }}",
            self.position.x, self.position.y,
            self.velocity.x, self.velocity.y
        )
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

