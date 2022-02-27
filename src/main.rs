use raytracer::{
    camera::Camera, image::Image, point3d::Point3d, renderer::Renderer, sphere::Sphere,
    world::World,
};

fn main() {
    let image = Image::new(400, 255);
    let camera = Camera::new(2.0, 1.0, image.aspect_ratio);
    let world = World::new(vec![
        Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0),
    ]);
    let renderer = Renderer::new(image, camera, world);

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
