use crate::{color::Color, hit::Hit, ray::Ray, scatterable::Scatterable, vector3::Vector3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambartian(Color),
    Metal(Color),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        match self {
            &Material::Lambartian(color) => lambartian(color, hit),
            &Material::Metal(color) => metal(color, ray_in, hit),
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

fn metal(color: Color, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
    let reflected = ray_in.direction.unit().reflect(hit.normal);
    let scattered = Ray::new(hit.point, reflected);

    if scattered.direction.dot(hit.normal).is_sign_positive() {
        Some((scattered, color))
    } else {
        None
    }
}
