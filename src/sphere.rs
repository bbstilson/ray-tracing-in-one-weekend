use crate::{hit::Hit, hittable::Hittable, point3d::Point3d, vector3::Vector3};

pub struct Sphere {
    center: Point3d,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant.is_sign_positive() {
            let t = (-half_b - discriminant.sqrt()) / a;
            if t.is_sign_positive() {
                Some(Hit { t, point: Point3d::zero(), normal: Vector3::zero() })
            } else {
                None
            }
        } else {
            None
        }
    }
}
