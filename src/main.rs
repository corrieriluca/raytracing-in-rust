use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::random::canonical_random;
use crate::ray::Ray;
use crate::vec3::Point3;
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
        return match hit_record.material.scatter(&r, &hit_record) {
            None => Color::new(0.0, 0.0, 0.0),
            Some((scattered, attenuation)) => attenuation * ray_color(scattered, world, depth - 1),
        };
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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

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
