use rand::{prelude::ThreadRng, Rng};

pub struct RandomNumberGenerator {
    rng: ThreadRng,
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            rng: rand::thread_rng(),
        }
    }

    pub fn get_random(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }
}
