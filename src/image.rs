pub struct Image {
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f64,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        Image {
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
        }
    }
}
