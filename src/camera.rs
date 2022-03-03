use std::f64::consts::PI;

use crate::{point3d::Point3d, ray::Ray, vector3::Vector3};

pub struct Camera {
    pub origin: Point3d,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub lower_left_corner: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3d,
        look_at: Point3d,
        view_up: Vector3,
        v_fov: i32,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = Camera::degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vector3::random_in_unit_circle() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
        )
    }

    fn degrees_to_radians(degrees: i32) -> f64 {
        (degrees as f64) / 180.0 * PI
    }
}
