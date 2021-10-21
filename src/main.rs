use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::image::Image;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::random::{canonical_random, random_range};
use crate::vec3::{Point3, Vec3};

use std::path::PathBuf;
use std::sync::Arc;

use camera::Camera;
use structopt::StructOpt;

mod camera;
mod color;
mod hittable;
mod image;
mod material;
mod parallel;
mod random;
mod ray;
mod vec3;

#[derive(StructOpt)]
#[structopt(name = "Raytracing in Rust")]
struct Opt {
    #[structopt(short, long, help = "Use multithreading for rendering")]
    parallel: bool,

    #[structopt(
        short = "j",
        help = "Number of threads to spawn. Default is number of logical cores"
    )]
    thread_number: Option<usize>,

    #[structopt(parse(from_os_str), help = "Where to save the result (BMP file)")]
    output: PathBuf,

    #[structopt(short, long, help = "Print debug information")]
    debug: bool,
}

/// Generate a scene with random small spheres and three big spheres
fn random_scene() -> Arc<dyn Hittable + Sync + Send> {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = canonical_random();
            let center = Point3::new(
                a as f64 + 0.9 * canonical_random(),
                0.2,
                b as f64 + 0.9 * canonical_random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    Arc::new(world)
}

fn main() {
    let opt = Opt::from_args();

    // Determine number of threads to spawn
    let thread_number = if !opt.parallel {
        1
    } else if let Some(n) = opt.thread_number {
        n
    } else {
        num_cpus::get()
    };

    if opt.debug {
        eprintln!("--- DEBUG ---");
        eprintln!("Thread number: {}", thread_number);
        eprintln!("Output file: {:?}", opt.output);
        eprintln!();
    }

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 500; // 1200

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let vertical_fov = 20.0;

    let world = random_scene();

    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        vertical_fov,
        aspect_ratio,
        aperture,
        focus_dist,
    ));

    let image = Image::new(
        aspect_ratio,
        image_width,
        Arc::clone(&world),
        Arc::clone(&camera),
    );

    // Rendering
    let samples_per_pixel = 100; // 500
    let max_depth = 50;

    image
        .render_image(samples_per_pixel, max_depth, thread_number)
        .flipv()
        .save_with_format(opt.output, ::image::ImageFormat::Bmp)
        .expect("An error occurred while writing the image to the file.");
}
