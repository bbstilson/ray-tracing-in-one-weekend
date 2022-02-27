use std::ops::Range;

use rand::Rng;

pub fn get_random() -> f64 {
    get_random_range(0.0..1.0)
}

pub fn get_random_range(range: Range<f64>) -> f64 {
    rand::thread_rng().gen_range(range)
}
