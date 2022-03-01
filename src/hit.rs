use crate::{material::Material, point3d::Point3d, vector3::Vector3};

pub struct Hit {
    pub point: Point3d,
    pub normal: Vector3,
    pub t: f64,
    pub material: Material,
}
