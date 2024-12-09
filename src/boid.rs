use std::ops::{Add, Sub, Div, Mul};

f32 PERCEPTION_RANGE 20
f32 AVOIDANCE_RANGE  2
f32 EDGE_MARGIN      50


f32 AVOIDANCE_FACTOR  0.05
f32 ALIGNEMENT_FACTOR 0.05
f32 COHESION_FACTOR   0.0005
f32 EDGE_TURN_FACTOR  0.2
f32 MIN_SPEED         2
f32 MAX_SPEED         3


#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub position: Vec2D,
    pub velocity: Vec2D,
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

