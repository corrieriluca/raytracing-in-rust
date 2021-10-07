pub mod lambertian;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

/// Represents a material
pub trait Material {
    /// Produces a scattered ray ([`Ray`] in return value), if scattered
    /// say how much the ray should be attenuated ([`Color`] in return value)
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}
