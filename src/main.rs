use raytracer::{camera::Camera, image::Image, point3d::Point3d, renderer::render};

fn main() {
    let image = Image::new(400, 255);
    let camera = Camera::new(2.0, 1.0, image.aspect_ratio);

    let file = render(&image, &camera).unwrap();
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
