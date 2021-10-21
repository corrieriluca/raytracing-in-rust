use crate::vec3::Vec3;
use image::{Rgb, RgbImage};

/// Alias of Vec3 representing a color
pub type Color = Vec3;

impl Color {
    pub fn write(&self, img: &mut RgbImage, x: u32, y: u32, samples_per_pixel: i32) {
        // Divide the color by the number of samples
        let scale = 1.0 / samples_per_pixel as f64;

        let red = (self.x() * scale).sqrt();
        let green = (self.y() * scale).sqrt();
        let blue = (self.z() * scale).sqrt();

        img.put_pixel(
            x,
            y,
            Rgb([
                (255.99 * red.clamp(0.0, 0.999)) as u8,
                (255.99 * green.clamp(0.0, 0.999)) as u8,
                (255.99 * blue.clamp(0.0, 0.999)) as u8,
            ]),
        );
    }
}
