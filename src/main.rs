use crate::color::Color;
use std::io::stdout;

mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            color
                .write(&mut stdout())
                .expect("An error occurred while writing to standard output");
        }
    }

    eprintln!("\nDone!");
}
