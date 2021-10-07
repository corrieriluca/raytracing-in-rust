use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray_in.direction().normalized(), record.normal);
        let scattered = Ray::new(record.intersection, reflected);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
