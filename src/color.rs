use crate::{rng::get_random, vector3::Vector3};

const GOLDEN_RATIO_CONJUGATE: f64 = 0.618033988749895;

pub type Color = Vector3;

pub const WHITE: Color = Vector3(1.0, 1.0, 1.0);
pub const BLACK: Color = Vector3(0.0, 0.0, 0.0);
pub const SALMON: Color = Vector3(0.91015625, 0.5859375, 0.4765625);

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

    pub fn to_rgb(self) -> (u8, u8, u8) {
        let r = (self.r() * 256.0).round() as u8;
        let g = (self.g() * 256.0).round() as u8;
        let b = (self.b() * 256.0).round() as u8;
        (r, g, b)
    }

    pub fn random() -> Color {
        let mut h = get_random() + GOLDEN_RATIO_CONJUGATE;
        h %= 1.0;
        let (r, g, b) = Color::hsv_to_rgb(h, 0.5, 0.95);
        Color::new(r, g, b)
    }

    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
        let i = (h * 6.0).round();
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        match i as i32 {
            ii if ii == 1 => (q, v, p),
            ii if ii == 2 => (p, v, t),
            ii if ii == 3 => (p, q, v),
            ii if ii == 4 => (t, p, v),
            ii if ii == 5 => (v, p, q),
            _ => (v, t, p),
        }
    }
}
