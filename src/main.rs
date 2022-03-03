use raytracer::{
    camera::Camera,
    color::Color,
    image::Image,
    material::Material,
    point3d::Point3d,
    renderer::render,
    rng::{get_random, get_random_range},
    scene::Scene,
    sphere::Sphere,
    vector3::Vector3,
    world::World,
};

fn main() {
    let image = Image::new(1200, 800);

    let look_from = Point3d::new(10.0, 2.0, 3.0);
    let look_at = Point3d::new(0.0, 0.0, 0.0);
    let view_up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let samples_per_pixel = 500;
    let max_depth = 50;

    let scene = Scene {
        width: image.width,
        height: image.height,
        samples_per_pixel,
        camera,
        max_depth: max_depth,
        world: make_random_world(),
    };

    let file = render(scene).unwrap();
    let file_str = format!("{}", file.display());

    std::process::Command::new("open")
        .arg(&file_str)
        .output()
        .expect("couldn't open the file");
    // std::process::Command::new("rm")
    //     .arg(&file_str)
    //     .output()
    //     .expect("couldn't delete the file");
}

fn make_random_world() -> World {
    let ground = Material::Lambartian(Color::new(0.5, 0.5, 0.5));
    let glass = Material::Dielectric(1.5);
    let solid = Material::Lambartian(Color::new(0.4, 0.2, 0.1));
    let metal = Material::Metal(Color::new(0.7, 0.6, 0.5), 1.0);

    let mut spheres = vec![
        Sphere::new(Point3d::new(0.0, -1000.0, 0.0), 1000.0, ground),
        Sphere::new(Point3d::new(0.0, 1.0, 0.0), 1.0, glass),
        Sphere::new(Point3d::new(-4.0, 1.0, 0.0), 1.0, solid),
        Sphere::new(Point3d::new(4.0, 1.0, 0.0), 1.0, metal),
    ];

    for a in 0..10 {
        for b in 0..10 {
            let center = Point3d::new(
                a as f64 + 0.9 * get_random(),
                0.2,
                b as f64 + 0.9 * get_random(),
            );
            if (Point3d::new(4.0, 0.2, 0.0) - center).length() > 0.9 {
                let material = match get_random() {
                    n if n < 0.33 => Material::Lambartian(Color::random()),
                    n if n < 0.66 => Material::Metal(Color::random(), get_random_range(0.5..1.0)),
                    _ => Material::Dielectric(1.5),
                };
                spheres.push(Sphere::new(center, 0.2, material));
            }
        }
    }

    World::new(spheres)
}
