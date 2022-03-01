use crate::{color::Color, hit::Hit, ray::Ray};

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}
