use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::random::canonical_random;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::io::stdout;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod material;
mod random;
mod ray;
mod vec3;

fn ray_color(r: Ray, world: &impl Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(&r, 0.001, f64::INFINITY) {
        let target = hit_record.intersection + hit_record.normal + Vec3::random_unit_vector();
        return 0.5
            * ray_color(
                Ray::new(hit_record.intersection, target - hit_record.intersection),
                world,
                depth - 1,
            );
    }
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(aspect_ratio);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + canonical_random()) / (image_width - 1) as f64;
                let v = (j as f64 + canonical_random()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            pixel_color
                .write(&mut stdout(), samples_per_pixel)
                .expect("An error occurred while writing to standard output");
        }
    }

    world.clear();

    eprintln!("\nDone!");
}
