pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn to_rgb(self) -> (i32, i32, i32) {
        let r = (self.r * 256.0).round() as i32;
        let g = (self.g * 256.0).round() as i32;
        let b = (self.b * 256.0).round() as i32;
        (r, g, b)
    }
}
