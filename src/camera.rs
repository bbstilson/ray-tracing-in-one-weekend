use crate::{point3d::Point3d, ray::Ray, vector3::Vector3};

pub struct Camera {
    pub origin: Point3d,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Vector3,
}

impl Camera {
    pub fn new(viewport_height: f64, focal_length: f64, aspect_ratio: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vector3::zero();
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}
