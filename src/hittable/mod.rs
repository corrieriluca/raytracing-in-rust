pub mod hittable_list;
pub mod sphere;

use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub intersection: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Sync + Send>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        outward_normal: Vec3,
        t: f64,
        material: Arc<dyn Material + Sync + Send>,
    ) -> HitRecord {
        let front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            intersection: r.at(t),
            normal,
            material,
            t,
            front_face,
        }
    }
}

/// Object that can be hit
pub trait Hittable {
    /// Tries to hit an object on the given range of the ray.
    /// Returns a record of the hit ([`HitRecord`]) in case of success.
    /// Returns [`None`] in case of failure.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
