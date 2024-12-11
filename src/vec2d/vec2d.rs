use std::ops::{Add, Sub, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    pub x : f32,
    pub y : f32,
}


impl Vec2D {
    pub fn distance(&self, other: &Vec2D) -> f32 {
        let diff = *other - *self;
        ((diff.x * diff.x) + (diff.y * diff.y)).sqrt()
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}


impl Div<f32> for Vec2D {
    type Output = Vec2D;

    fn div(self, scalar: f32) -> Vec2D {
        Vec2D {
            x : self.x / scalar,
            y : self.y / scalar,
        }
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, scalar: f32) -> Vec2D {
        Vec2D {
            x : self.x * scalar,
            y : self.y * scalar,
        }
    }
}

