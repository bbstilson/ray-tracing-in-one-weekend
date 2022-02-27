use crate::{hit::Hit, hittable::Hittable, ray::Ray, sphere::Sphere};

pub struct World {
    hittables: Vec<Sphere>,
}

impl World {
    pub fn new(hittables: Vec<Sphere>) -> World {
        World { hittables }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.hittables
            .iter()
            .fold(None, |previous_hit, hittable| match previous_hit {
                Some(hit) => hittable.hit(ray, t_min, hit.t).or(Some(hit)),
                None => hittable.hit(ray, t_min, t_max),
            })
    }
}
