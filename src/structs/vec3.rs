use std::ops::{Add, Sub, Neg, Mul, Div, BitXor};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub const X: Vec3 = Vec3 { x: 1., y: 0., z: 0. };
pub const Y: Vec3 = Vec3 { x: 0., y: 1., z: 0. };
pub const Z: Vec3 = Vec3 { x: 0., y: 0., z: 1. };
pub const O: Vec3 = Vec3 { x: 0., y: 0., z: 0. };

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const fn x(&self) -> f64 { self.x }
    pub const fn y(&self) -> f64 { self.y }
    pub const fn z(&self) -> f64 { self.z }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn unit(self) -> Vec3 {
        self / self.magnitude()
    }

    pub fn distance_to(self, other: Vec3) -> f64 {
        (self - other).magnitude()
    }

    pub fn angle_to(self, other: Vec3) -> f64 {
        ((self * other) / (self.magnitude() * other.magnitude())).acos()
    }

    pub fn member_mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x*other.x,
            y: self.y*other.y,
            z: self.z*other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl BitXor for Vec3 {
    type Output = Vec3;

    fn bitxor(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, k: f64) -> Vec3 {
        Vec3 {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}