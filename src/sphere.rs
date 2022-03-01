use crate::{hit::Hit, hittable::Hittable, material::Material, point3d::Point3d, ray::Ray};

pub struct Sphere {
    center: Point3d,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3d, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant.is_sign_positive() {
            let sqrtd = discriminant.sqrt();
            // Find the nearest root that lies in the acceptable range
            let root_a = (-half_b - sqrtd) / a;
            let root_b = (-half_b + sqrtd) / a;
            [root_a, root_b]
                .iter()
                .find(|root| **root < t_max && **root > t_min)
                .map(|root| {
                    let t = *root;
                    let point = ray.at(t);
                    let normal = (point - self.center) / self.radius;
                    let is_front_face = ray.direction.dot(normal).is_sign_negative();
                    Hit {
                        t,
                        point,
                        material: self.material,
                        normal: if is_front_face { normal } else { -normal },
                    }
                })
        } else {
            None
        }
    }
}
