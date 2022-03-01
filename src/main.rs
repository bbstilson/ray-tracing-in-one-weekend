use raytracer::{
    camera::Camera, color::Color, image::Image, material::Material, point3d::Point3d,
    renderer::Renderer, sphere::Sphere, world::World,
};

fn main() {
    let image = Image::new(400, 255);
    let camera = Camera::new(2.0, 1.0, image.aspect_ratio);

    let material_ground = Material::Lambartian(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambartian(Color::new(0.1, 0.2, 0.5));
    let material_left = Material::Dielectric(1.5);
    let material_right = Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0);

    let world = World::new(vec![
        Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0, material_ground),
        Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5, material_center),
        Sphere::new(Point3d::new(-1.0, 0.0, -1.0), 0.5, material_left),
        Sphere::new(Point3d::new(1.0, 0.0, -1.0), 0.5, material_right),
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
