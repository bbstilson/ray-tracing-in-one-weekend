use raytracer::{camera::Camera, image::Image, renderer::render};

fn main() {
    let image = Image::new(720, 405);
    let camera = Camera::new(2.0, 1.0, image.aspect_ratio);

    render(image, camera).unwrap();
}
