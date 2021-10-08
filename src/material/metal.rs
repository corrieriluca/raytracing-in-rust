use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray_in.direction().normalized(), record.normal);
        let scattered = Ray::new(
            record.intersection,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
