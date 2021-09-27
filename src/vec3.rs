#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A simple three dimensional vector
#[derive(Debug, Copy, Clone)]
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

    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
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

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, u: Vec3) -> Self::Output {
        u * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        (1.0 / t) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn length_works() {
        assert_eq!(6.0, Vec3::new(4.0, 4.0, 2.0).length());
    }

    fn dot_product_works() {
        assert_eq!(
            11.0,
            Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn eq_works() {
        assert_eq!(Vec3::new(0.0, 1.0, 3.0), Vec3::new(0.0, 1.0, 3.0));
    }

    #[test]
    fn neq_works() {
        assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::zero());
    }

    #[test]
    fn negation_works() {
        assert_eq!(Vec3::new(-1.0, 2.0, -3.0), -Vec3::new(1.0, -2.0, 3.0));
    }

    #[test]
    fn add_works() {
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(
            expected,
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn add_assign_works() {
        let mut u = Vec3::new(1.0, 0.0, 0.0);
        u += Vec3::new(0.0, 2.0, 3.0);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), u);
    }

    #[test]
    fn sub_works() {
        let expected = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(
            expected,
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn sub_assign_works() {
        let mut u = Vec3::new(1.0, 2.0, 3.0);
        u -= Vec3::new(1.0, 2.0, 1.0);
        assert_eq!(Vec3::new(0.0, 0.0, 2.0), u);
    }

    #[test]
    fn mul_vec3_vec3_works() {
        let expected = Vec3::new(1.0, 4.0, 9.0);
        assert_eq!(
            expected,
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn mul_vec3_f64_works() {
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(expected, Vec3::new(1.0, 2.0, 3.0) * 2.0);
        assert_eq!(expected, 2.0 * Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn mul_assign_vec3_f64_works() {
        let mut u = Vec3::new(1.0, 2.0, 3.0);
        u *= 2.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), u);
    }

    #[test]
    fn div_vec3_f64_works() {
        let expected = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(expected, Vec3::new(2.0, 4.0, 6.0) / 2.0);
    }

    #[test]
    fn div_assign_f64_works() {
        let mut u = Vec3::new(2.0, 4.0, 6.0);
        u /= 2.0;
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), u);
    }

    #[test]
    fn normalized_works() {
        assert_eq!(
            Vec3::new(4.0, 4.0, 2.0).normalized(),
            Vec3::new(4.0 / 6.0, 4.0 / 6.0, 2.0 / 6.0)
        );
    }
}
