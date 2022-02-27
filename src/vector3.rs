use crate::rng::get_random;

#[derive(Debug, Clone, Copy)]
pub struct Vector3(pub f64, pub f64, pub f64);

impl Vector3 {
    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn z(self) -> f64 {
        self.2
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3(x, y, z)
    }

    pub fn zero() -> Vector3 {
        Vector3(0.0, 0.0, 0.0)
    }

    // Returns a Vector from the center of a unit Sphere to a random point
    // on its surface.
    // https://karthikkaranth.me/blog/generating-random-points-in-a-sphere/
    pub fn random_unit() -> Vector3 {
        let c = get_random();
        let mut x = get_random();
        let mut y = get_random();
        let mut z = get_random();

        let mag = (x * x + y * y + z * z).sqrt();
        x /= mag;
        y /= mag;
        z /= mag;

        Vector3::new(x * c, y * c, z * c)
    }

    pub fn unit(self) -> Vector3 {
        self / self.length()
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }
}
