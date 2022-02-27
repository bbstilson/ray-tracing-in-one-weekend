use crate::camera::Camera;
use crate::color::{Color, WHITE};
use crate::hittable::Hittable;
use crate::image::Image;
use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::world::World;

use indicatif::ProgressIterator;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time;

pub struct Renderer {
    image: Image,
    camera: Camera,
    world: World,
}

impl Renderer {
    pub fn new(image: Image, camera: Camera, world: World) -> Renderer {
        Renderer {
            image,
            camera,
            world,
        }
    }

    pub fn render(&self) -> Result<PathBuf, io::Error> {
        let file_path = Renderer::get_output_file();
        let output = file_path.clone();
        let file = fs::File::create(file_path)?;
        let mut writer = io::LineWriter::new(file);

        writer
            .write(format!("P3\n{} {}\n255\n", self.image.width, self.image.height).as_bytes())?;
        for h in (0..self.image.height).rev().progress() {
            for w in 0..self.image.width {
                let u = w as f64 / (self.image.width - 1) as f64;
                let v = h as f64 / (self.image.height - 1) as f64;
                let ray = Ray::new(
                    self.camera.origin,
                    self.camera.lower_left_corner
                        + (self.camera.horizontal * u)
                        + (self.camera.vertical * v)
                        - self.camera.origin,
                );
                let (r, g, b) = self.ray_color(ray).to_rgb();
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

    fn ray_color(&self, ray: Ray) -> Color {
        match self.world.hit(&ray, 0.0, f64::INFINITY) {
            Some(hit) => (WHITE + hit.normal) * 0.5,
            None => Renderer::hit_sky(ray),
        }
    }

    fn hit_sky(ray: Ray) -> Color {
        let unit_direction = Vector3::unit(ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (WHITE * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
