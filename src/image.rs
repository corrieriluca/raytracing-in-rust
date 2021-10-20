use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::parallel::ThreadPool;
use crate::random::canonical_random;
use crate::ray::Ray;
use crate::vec3::Vec3;

use image::DynamicImage;
use pbr::ProgressBar;

/// Represent an image to be rendered
pub struct Image {
    image_width: u32,
    image_height: u32,
    world: Arc<dyn Hittable + Sync + Send>,
    camera: Arc<Camera>,
}

/// Represent a Pixel to be rendered on the resulting image
struct Pixel {
    x: u32,
    y: u32,
    color: Color,
}

impl Image {
    /// Constructs a new image (height is calculated with `aspect_ratio` and `image_width`)
    /// with its camera
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        world: Arc<dyn Hittable + Sync + Send>,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        focus_dist: f64,
        aperture: f64,
        vertical_fov: f64,
    ) -> Image {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let camera = Arc::new(Camera::new(
            lookfrom,
            lookat,
            vup,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_dist,
        ));

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

        let mut img = DynamicImage::new_rgb8(self.image_width, self.image_height);

        let pb = Arc::new(Mutex::new(ProgressBar::new(self.image_height as u64)));

        // Split the image into chunks (one chunk per thread in the pool)

        let lines: Vec<u32> = (0..self.image_height).rev().collect();
        let mut chunks = Vec::new();
        for chunk in lines.chunks(self.image_height as usize / thread_number) {
            chunks.push(chunk.to_owned());
        }
        let chunk_number = chunks.len();

        // Render all the chunks in its own thread

        let (sender, receiver) = channel();

        for chunk in chunks {
            let pb = Arc::clone(&pb);
            let world = Arc::clone(&self.world);
            let camera = Arc::clone(&self.camera);

            let image_width = self.image_width;
            let image_height = self.image_height;

            let sender = sender.clone();

            pool.execute(move || {
                let mut colors = Vec::with_capacity(chunk.len() * image_width as usize);
                for j in chunk {
                    for i in 0..image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let u = (i as f64 + canonical_random()) / (image_width - 1) as f64;
                            let v = (j as f64 + canonical_random()) / (image_height - 1) as f64;
                            let r = camera.get_ray(u, v);
                            pixel_color += Image::ray_color(Arc::clone(&world), r, max_depth);
                        }

                        colors.push(Pixel {
                            x: i,
                            y: j,
                            color: pixel_color,
                        });
                    }

                    pb.lock().unwrap().inc();
                }

                sender.send(colors).expect("Cannot send colors from thread");
            });
        }

        pool.wait_all();

        pb.lock().unwrap().finish_print("Done!");

        // Collect all the pixels from the threads and put them on the image

        let pixels = receiver
            .iter()
            .take(chunk_number)
            .flatten()
            .collect::<Vec<Pixel>>();

        for p in pixels {
            p.color
                .write(img.as_mut_rgb8().unwrap(), p.x, p.y, samples_per_pixel);
        }

        img
    }
}
