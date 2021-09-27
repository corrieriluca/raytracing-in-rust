use crate::vec3::Vec3;
use std::io::Write;

/// Alias of Vec3 representing a color
pub type Color = Vec3;

impl Color {
    pub fn write(&self, stream: &mut impl Write) -> std::io::Result<()> {
        writeln!(
            stream,
            "{} {} {}",
            (255.99 * self.x()) as i32,
            (255.99 * self.y()) as i32,
            (255.99 * self.z()) as i32
        )
    }
}
