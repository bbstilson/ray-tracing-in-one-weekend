use crate::{camera::Camera, world::World};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub camera: Camera,
    pub world: World,
}
