use crate::{
    color::{Color, WHITE},
    hit::Hit,
    ray::Ray,
    rng::get_random,
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
            &Material::Dielectric(refraction_index) => dielectric(refraction_index, ray_in, hit),
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

fn dielectric(refraction_index: f64, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
    let refraction_ratio = if hit.is_front_face {
        1.0 / refraction_index
    } else {
        refraction_index
    };
    let unit_direction = ray_in.direction.unit();

    let cos_theta = (-unit_direction.dot(hit.normal)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let should_reflect = reflectance(cos_theta, refraction_index) > get_random();

    let direction = if cannot_refract || should_reflect {
        unit_direction.reflect(hit.normal)
    } else {
        unit_direction.refract(hit.normal, refraction_ratio)
    };

    let scattered = Ray::new(hit.point, direction);
    Some((scattered, WHITE))
}

// Use Schlick's approximation for reflectance.
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r02 = r0.powi(2);
    r02 + (1.0 - r02) * (1.0 - cosine).powi(5)
}
