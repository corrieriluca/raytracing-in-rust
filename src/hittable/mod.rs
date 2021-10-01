pub mod sphere;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub intersection: Point3,
    pub normal: Vec3,
    pub t: f64,
}

/// Object that can be hit
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
