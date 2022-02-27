pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}

pub fn clamp01(x: f64) -> f64 {
    clamp(x, 0.0, 1.0)
}
