use crate::camera::Camera;
use crate::color::{Color, WHITE};
use crate::image::Image;
use crate::ray::Ray;
use crate::vector3::Vector3;

use indicatif::ProgressIterator;
use std::env;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::time;

pub fn render(image: Image, camera: Camera) -> Result<(), io::Error> {
    let mut file = get_output_file()?;
    file.write(format!("P3\n{} {}\n255\n", image.width, image.height).as_bytes())?;
    for h in (0..image.height).rev().progress() {
        for w in 0..image.width {
            let u = w as f64 / (image.width - 1) as f64;
            let v = h as f64 / (image.height - 1) as f64;
            let ray = Ray::new(
                camera.origin,
                camera.lower_left_corner + (camera.horizontal * u) + (camera.vertical * v)
                    - camera.origin,
            );
            let (r, g, b) = ray_color(ray).to_rgb();
            file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }
    Ok(())
}

fn get_output_file() -> Result<io::LineWriter<File>, io::Error> {
    let file_name = format!(
        "{:?}.ppm",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let mut file_path = env::current_dir().expect("Couldn't get current directory.");
    file_path.push(file_name);
    println!("Writing file {}", file_path.display());

    let file = fs::File::create(file_path)?;
    Ok(io::LineWriter::new(file))
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = Vector3::unit(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (WHITE * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}
