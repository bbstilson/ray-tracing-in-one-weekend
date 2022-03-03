pub struct Image {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
        }
    }
}
