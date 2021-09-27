#![allow(dead_code)]

use std::ops::Neg;

/// A simple three dimensional vector
#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    /// Constructs a new vector with given x, y and z values
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Returns the zero vector
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product two vectors
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Returns the cross product of two vectors
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq_works() {
        assert_eq!(Vec3::new(0.0, 1.0, 3.0), Vec3::new(0.0, 1.0, 3.0));
    }

    #[test]
    fn neq_works() {
        assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn negation_works() {
        assert_eq!(Vec3::new(-1.0, 2.0, -3.0), -Vec3::new(1.0, -2.0, 3.0));
    }
}
