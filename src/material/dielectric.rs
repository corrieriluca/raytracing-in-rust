use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Represents a dielectric material (a material that might refract)
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    /// Returns a new Dielectric material with `ir` the given
    /// index of refraction
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = Vec3::dot(&(-unit_direction), &record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, record.normal)
        } else {
            Vec3::refract(unit_direction, record.normal, refraction_ratio)
        };

        let scattered = Ray::new(record.intersection, direction);

        Some((scattered, attenuation))
    }
}
