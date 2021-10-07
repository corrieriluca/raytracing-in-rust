use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.intersection, scatter_direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
