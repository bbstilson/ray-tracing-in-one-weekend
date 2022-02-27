use crate::vector3::Vector3;

pub type Color = Vector3;

pub const WHITE: Color = Vector3(1.0, 1.0, 1.0);
pub const BLACK: Color = Vector3(0.0, 0.0, 0.0);

impl Color {
    pub fn r(self) -> f64 {
        self.0
    }
    pub fn g(self) -> f64 {
        self.1
    }
    pub fn b(self) -> f64 {
        self.2
    }

    pub fn to_rgb(self) -> (i32, i32, i32) {
        let r = (self.r() * 256.0).round() as i32;
        let g = (self.g() * 256.0).round() as i32;
        let b = (self.b() * 256.0).round() as i32;
        (r, g, b)
    }
}
