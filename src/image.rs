use std::sync::{Arc, Mutex};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::parallel::ThreadPool;
use crate::random::canonical_random;
use crate::ray::Ray;

use image::DynamicImage;
use pbr::ProgressBar;

/// Represent an image to be rendered
pub struct Image {
    image_width: u32,
    image_height: u32,
    world: Arc<dyn Hittable + Sync + Send>,
    camera: Arc<Camera>,
}

impl Image {
    /// Constructs a new image (height is calculated with `aspect_ratio` and `image_width`)
    /// with its camera
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        world: Arc<dyn Hittable + Sync + Send>,
        camera: Arc<Camera>,
    ) -> Image {
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        Image {
            image_width,
            image_height,
            world,
            camera,
        }
    }

    /// Computes the color rendered for a given ray `r` with a maximum
    /// recursion depth of `depth`
    fn ray_color(world: Arc<dyn Hittable + Sync + Send>, r: Ray, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(&r, 0.001, f64::INFINITY) {
            return match hit_record.material.scatter(&r, &hit_record) {
                None => Color::new(0.0, 0.0, 0.0),
                Some((scattered, attenuation)) => {
                    attenuation * Image::ray_color(world, scattered, depth - 1)
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
        samples_per_pixel: i32,
        max_depth: i32,
        thread_number: usize,
    ) -> DynamicImage {
        let mut pool = ThreadPool::new(thread_number);

        let img = Arc::new(Mutex::new(DynamicImage::new_rgb8(
            self.image_width,
            self.image_height,
        )));
        let pb = Arc::new(Mutex::new(ProgressBar::new(self.image_height as u64)));

        for j in (0..self.image_height).rev() {
            let pb = Arc::clone(&pb);
            let world = Arc::clone(&self.world);
            let img = Arc::clone(&img);
            let camera = Arc::clone(&self.camera);

            let image_width = self.image_width;
            let image_height = self.image_height;

            // Each line rendering is sent to the thread pool
            pool.execute(move || {
                for i in 0..image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + canonical_random()) / (image_width - 1) as f64;
                        let v = (j as f64 + canonical_random()) / (image_height - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += Image::ray_color(Arc::clone(&world), r, max_depth);
                    }
                    pixel_color.write(
                        img.lock().unwrap().as_mut_rgb8().unwrap(),
                        i,
                        j,
                        samples_per_pixel,
                    )
                }

                pb.lock().unwrap().inc();
            });
        }

        pool.wait_all();

        pb.lock().unwrap().finish_print("Done!");

        let img = img.lock().unwrap().clone();
        img
    }
}
