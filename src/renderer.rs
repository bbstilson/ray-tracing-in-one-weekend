use crate::color::{Color, BLACK, WHITE};
use crate::file::write_image;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::rng::get_random;
use crate::scatterable::Scatterable;
use crate::scene::Scene;
use crate::vector3::Vector3;
use crate::world::World;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io;
use std::path::PathBuf;

pub fn render(scene: Scene) -> Result<PathBuf, io::Error> {
    let mut pixels = vec![0; scene.height as usize * scene.width as usize * 3];
    let lines: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(scene.width as usize * 3)
        .rev()
        .enumerate()
        .collect();

    lines
        .into_par_iter()
        .progress()
        .for_each(|(h, line)| render_line(line, &scene, h as u32));

    write_image(&pixels, scene.height, scene.width)
}

fn render_line(pixels: &mut [u8], scene: &Scene, h: u32) {
    for w in 0..scene.width {
        let pixel_color = (0..scene.samples_per_pixel).fold(BLACK, |mut color, _| {
            let u = (w as f64 + get_random()) / (scene.width - 1) as f64;
            let v = (h as f64 + get_random()) / (scene.height - 1) as f64;
            let ray = scene.camera.get_ray(u, v);
            color += ray_color(ray, &scene.world, scene.max_depth);
            color
        });

        // Divide the color by the number of samples.
        let scale = 1.0 / scene.samples_per_pixel as f64;
        let (r, g, b) = Color::new(
            (pixel_color.r() * scale).sqrt(),
            (pixel_color.g() * scale).sqrt(),
            (pixel_color.b() * scale).sqrt(),
        )
        .to_rgb();
        let u_w = w as usize * 3;
        pixels[u_w] = r;
        pixels[u_w + 1] = g;
        pixels[u_w + 2] = b;
    }
}

fn ray_color(ray: Ray, world: &World, remaining_steps: u32) -> Color {
    if remaining_steps > 0 {
        match world.hit(&ray, 0.0001, f64::INFINITY) {
            Some(hit) => match hit.material.scatter(&ray, &hit) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(scattered, world, remaining_steps - 1)
                }
                None => BLACK,
            },
            None => hit_sky(ray),
        }
    } else {
        BLACK
    }
}

fn hit_sky(ray: Ray) -> Color {
    let unit_direction = Vector3::unit(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (WHITE * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}
