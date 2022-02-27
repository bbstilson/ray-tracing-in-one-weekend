use raytracer::ppm;

fn main() {
    ppm::write_image(256, 256).unwrap();
}
