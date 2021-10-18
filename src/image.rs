use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::Hittable;
use crate::random::canonical_random;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::io::Write;

/// Represent an image to be rendered
pub struct Image {
    image_width: i32,
    image_height: i32,
    world: HittableList,
    camera: Camera,
}

impl Image {
    /// Constructs a new image (height is calculated with `aspect_ratio` and `image_width`)
    /// with its camera
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        world: HittableList,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        focus_dist: f64,
        aperture: f64,
        vertical_fov: f64,
    ) -> Image {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let camera = Camera::new(
            lookfrom,
            lookat,
            vup,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_dist,
        );

        Image {
            image_width,
            image_height,
            world,
            camera,
        }
    }

    /// Computes the color rendered for a given ray `r` with a maximum
    /// recursion depth of `depth`
    fn ray_color(&self, r: Ray, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = self.world.hit(&r, 0.001, f64::INFINITY) {
            return match hit_record.material.scatter(&r, &hit_record) {
                None => Color::new(0.0, 0.0, 0.0),
                Some((scattered, attenuation)) => {
                    attenuation * self.ray_color(scattered, depth - 1)
                }
            };
        }

        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    /// Renders the image to the PPM format to the specified stream
    /// (may be a file or just standard output)
    pub fn render_image(
        &self,
        stream: &mut impl Write,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> std::io::Result<()> {
        write!(
            stream,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in (0..self.image_height).rev() {
            eprint!("\rLines remaining: {} ", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + canonical_random()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + canonical_random()) / (self.image_height - 1) as f64;
                    let r = self.camera.get_ray(u, v);
                    pixel_color += self.ray_color(r, max_depth);
                }
                pixel_color.write(stream, samples_per_pixel)?
            }
        }

        Ok(())
    }
}
