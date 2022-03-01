use crate::{
    color::{Color, WHITE},
    hit::Hit,
    ray::Ray,
    scatterable::Scatterable,
    vector3::Vector3,
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambartian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        match self {
            &Material::Lambartian(color) => lambartian(color, hit),
            &Material::Metal(color, fuzz) => metal(color, fuzz, ray_in, hit),
            &Material::Dielectric(index_of_refraction) => {
                dielectric(index_of_refraction, ray_in, hit)
            }
        }
    }
}

fn lambartian(color: Color, hit: &Hit) -> Option<(Ray, Color)> {
    let scatter_direction = hit.normal + Vector3::random_unit_vector();
    let scatter_direction = if scatter_direction.is_near_zero() {
        hit.normal
    } else {
        scatter_direction
    };
    let scattered = Ray::new(
        hit.point,
        match scatter_direction.is_near_zero() {
            true => hit.normal,
            false => scatter_direction,
        },
    );
    Some((scattered, color))
}

fn metal(color: Color, fuzz: f64, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
    let reflected = ray_in.direction.unit().reflect(hit.normal);
    let scattered = Ray::new(
        hit.point,
        reflected * Vector3::random_in_unit_sphere() * fuzz,
    );

    if scattered.direction.dot(hit.normal).is_sign_positive() {
        Some((scattered, color))
    } else {
        None
    }
}

fn dielectric(index_of_refraction: f64, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
    let refraction_ratio = if hit.is_front_face {
        1.0 / index_of_refraction
    } else {
        index_of_refraction
    };
    let unit_direction = ray_in.direction.unit();
    let refracted = unit_direction.refract(hit.normal, refraction_ratio);
    let scattered = Ray::new(hit.point, refracted);
    Some((scattered, WHITE))
}
