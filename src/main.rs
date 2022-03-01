use std::f64::consts::PI;

use raytracer::{
    camera::Camera, color::Color, image::Image, material::Material, point3d::Point3d,
    renderer::Renderer, sphere::Sphere, world::World,
};

fn main() {
    let image = Image::new(400, 255);
    let camera = Camera::new(90, image.aspect_ratio);

    let material_left = Material::Lambartian(Color::new(0.0, 0.0, 1.0));
    let material_right = Material::Lambartian(Color::new(1.0, 0.0, 0.0));

    let radius = (PI / 4.0).cos();

    let world = World::new(vec![
        Sphere::new(Point3d::new(-radius, 0.0, -1.0), radius, material_left),
        Sphere::new(Point3d::new(radius, 0.0, -1.0), radius, material_right),
    ]);

    let samples_per_pixel = 100;
    let max_ray_depth = 25;

    let mut renderer = Renderer::new(image, camera, world, samples_per_pixel, max_ray_depth);

    let file = renderer.render().unwrap();
    let file_str = format!("{}", file.display());

    std::process::Command::new("open")
        .arg(&file_str)
        .output()
        .expect("couldn't open the file");
    std::process::Command::new("rm")
        .arg(&file_str)
        .output()
        .expect("couldn't delete the file");
}
