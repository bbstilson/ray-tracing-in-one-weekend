use crate::camera::Camera;
use crate::color::{Color, BLACK, WHITE};
use crate::hittable::Hittable;
use crate::image::Image;
use crate::ray::Ray;
use crate::rng::get_random;
use crate::scatterable::Scatterable;
use crate::utils::clamp01;
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
    samples_per_pixel: i32,
    max_ray_depth: i32,
}

impl Renderer {
    pub fn new(
        image: Image,
        camera: Camera,
        world: World,
        samples_per_pixel: i32,
        max_ray_depth: i32,
    ) -> Renderer {
        Renderer {
            image,
            camera,
            world,
            samples_per_pixel,
            max_ray_depth,
        }
    }

    pub fn render(&mut self) -> Result<PathBuf, io::Error> {
        let file_path = Renderer::get_output_file();
        let output = file_path.clone();
        let file = fs::File::create(file_path)?;
        let mut writer = io::LineWriter::new(file);

        writer
            .write(format!("P3\n{} {}\n255\n", self.image.width, self.image.height).as_bytes())?;

        for h in (0..self.image.height).rev().progress() {
            for w in 0..self.image.width {
                let (r, g, b) = self.get_antialiased_pixel_color(w, h).to_rgb();
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

    fn ray_color(&self, ray: Ray, remaining_steps: i32) -> Color {
        if remaining_steps > 0 {
            match self.world.hit(&ray, 0.001, f64::INFINITY) {
                Some(hit) => match hit.material.scatter(&ray, &hit) {
                    Some((scattered, attenuation)) => {
                        attenuation * self.ray_color(scattered, remaining_steps - 1)
                    }
                    None => BLACK,
                },
                None => Renderer::hit_sky(ray),
            }
        } else {
            BLACK
        }
    }

    fn get_antialiased_pixel_color(&mut self, w: i32, h: i32) -> Color {
        let pixel_color = (0..self.samples_per_pixel).fold(BLACK, |mut color, _| {
            let u = (w as f64 + get_random()) / (self.image.width - 1) as f64;
            let v = (h as f64 + get_random()) / (self.image.height - 1) as f64;
            let ray = self.camera.get_ray(u, v);
            color += self.ray_color(ray, self.max_ray_depth);
            color
        });
        self.antialias(pixel_color)
    }

    fn antialias(&self, pixel_color: Color) -> Color {
        let r = pixel_color.r();
        let g = pixel_color.g();
        let b = pixel_color.b();

        // Divide the color by the number of samples.
        let scale = 1.0 / self.samples_per_pixel as f64;
        let r = (r * scale).sqrt();
        let g = (g * scale).sqrt();
        let b = (b * scale).sqrt();

        Color::new(clamp01(r), clamp01(g), clamp01(b))
    }

    fn hit_sky(ray: Ray) -> Color {
        let unit_direction = Vector3::unit(ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (WHITE * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
