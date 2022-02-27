use std::ops;

use crate::vector3::Vector3;

//
// Vector3 ops
//
impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

//
// Vector3 mutable ops
//

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

//
// f64 ops
//
impl ops::Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f64) -> Vector3 {
        Vector3::new(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl ops::Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f64) -> Vector3 {
        Vector3::new(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        self * (1.0 / rhs)
    }
}

//
// f64 mutable ops
//

impl ops::AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl ops::SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
