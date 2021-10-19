use crate::vec3::Vec3;
use image::{Rgb, RgbImage};

/// Alias of Vec3 representing a color
pub type Color = Vec3;

impl Color {
    pub fn write(&self, img: &mut RgbImage, x: u32, y: u32, samples_per_pixel: i32) {
        // Divide the color by the number of samples
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.x() * scale).sqrt();
        let g = (self.y() * scale).sqrt();
        let b = (self.z() * scale).sqrt();

        img.put_pixel(
            x,
            y,
            Rgb([
                (255.99 * r.clamp(0.0, 0.999)) as u8,
                (255.99 * g.clamp(0.0, 0.999)) as u8,
                (255.99 * b.clamp(0.0, 0.999)) as u8,
            ]),
        );
    }
}
