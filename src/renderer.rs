use crate::camera::Camera;
use crate::color::{Color, SALMON, WHITE};
use crate::image::Image;
use crate::point3d::Point3d;
use crate::ray::Ray;
use crate::vector3::Vector3;

use indicatif::ProgressIterator;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time;

pub fn render(image: &Image, camera: &Camera) -> Result<PathBuf, io::Error> {
    let file_path = get_output_file();
    let output = file_path.clone();
    let file = fs::File::create(file_path)?;
    let mut writer = io::LineWriter::new(file);

    writer.write(format!("P3\n{} {}\n255\n", image.width, image.height).as_bytes())?;
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
            writer.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }

    Ok(output)
}

fn get_output_file() -> PathBuf {
    let file_name = format!(
        "{:?}.ppm",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    env::current_dir()
        .expect("Couldn't get current directory.")
        .join(file_name)
}

fn hit_sphere(ray: &Ray) -> bool {
    let center = Point3d::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant.is_sign_positive()
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(&ray) {
        SALMON
    } else {
        let unit_direction = Vector3::unit(ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (WHITE * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
