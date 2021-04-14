use crate::structs::vec3::Vec3;
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3 {
    a: f64, b: f64, c: f64,
    d: f64, e: f64, f: f64,
    g: f64, h: f64, i: f64,
}

pub const I3: Mat3 = Mat3 {
    a: 1., b: 0., c: 0.,
    d: 0., e: 1., f: 0.,
    g: 0., h: 0., i: 1.,
};

pub const O3: Mat3 = Mat3 {
    a: 0., b: 0., c: 0.,
    d: 0., e: 0., f: 0.,
    g: 0., h: 0., i: 0.,
};

pub const COFACTORS: Mat3 = Mat3 {
    a:  1., b: -1., c:  1.,
    d: -1., e:  1., f: -1.,
    g:  1., h: -1., i:  1.,
};

impl Mat3 {
    pub const fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Mat3 {
        Mat3 {
            a, b, c,
            d, e, f,
            g, h, i,
        }
    }

    pub const fn new_cols(a: Vec3, b: Vec3, c: Vec3) -> Mat3 {
        Mat3 {
            a: a.x(), b: b.x(), c: c.x(),
            d: a.y(), e: b.y(), f: c.y(),
            g: a.z(), h: b.z(), i: c.z(),
        }
    }

    pub const fn new_rows(a: Vec3, d: Vec3, g: Vec3) -> Mat3 {
        Mat3 {
            a: a.x(), b: a.y(), c: a.z(),
            d: d.x(), e: d.y(), f: d.z(),
            g: g.x(), h: g.y(), i: g.z(),
        }
    }

    pub const fn a(&self) -> f64 { self.a }
    pub const fn b(&self) -> f64 { self.b }
    pub const fn c(&self) -> f64 { self.c }
    pub const fn d(&self) -> f64 { self.d }
    pub const fn e(&self) -> f64 { self.e }
    pub const fn f(&self) -> f64 { self.f }
    pub const fn g(&self) -> f64 { self.g }
    pub const fn h(&self) -> f64 { self.h }
    pub const fn i(&self) -> f64 { self.i }

    pub fn det(&self) -> f64 {
        let (a, b, c) = (self.a(), self.b(), self.c());
        let (d, e, f) = (self.d(), self.e(), self.f());
        let (g, h, i) = (self.g(), self.h(), self.i());

        a*(e*i - f*h) - b*(d*i - f*g) + c*(d*h - e*g)
    }

    pub const fn trans(&self) -> Mat3 {
        Mat3 {
            a: self.a, b: self.d, c: self.g,
            d: self.b, e: self.e, f: self.h,
            g: self.c, h: self.f, i: self.i,
        }
    }

    pub fn minor(&self) -> Mat3 {
        let (a, b, c) = (self.a(), self.b(), self.c());
        let (d, e, f) = (self.d(), self.e(), self.f());
        let (g, h, i) = (self.g(), self.h(), self.i());

        Mat3 {
            a: e*i - f*h, b: d*i - f*g, c: d*h - e*g,
            d: b*i - c*h, e: a*i - c*g, f: a*h - b*g,
            g: b*f - c*e, h: a*f - c*d, i: a*e - b*d,
        }
    }

    pub fn invert(&self) -> Mat3 {
        (self.minor().member_mul(COFACTORS)).trans() / self.det()
    }

    pub fn member_mul(&self, other: Mat3) -> Mat3 {
        Mat3 {
            a: self.a*other.a, b: self.b*other.b, c: self.c*other.c,
            d: self.d*other.d, e: self.e*other.e, f: self.f*other.f,
            g: self.g*other.g, h: self.h*other.h, i: self.i*other.i,
        }
    }
}

impl Add for Mat3 {
    type Output = Mat3;

    fn add(self, other: Mat3) -> Mat3 {
        Mat3 {
            a: self.a+other.a, b: self.b+other.b, c: self.c+other.c,
            d: self.d+other.d, e: self.e+other.e, f: self.f+other.f,
            g: self.g+other.g, h: self.h+other.h, i: self.i+other.i,
        }
    }
}

impl Sub for Mat3 {
    type Output = Mat3;

    fn sub(self, other: Mat3) -> Mat3 {
        Mat3 {
            a: self.a-other.a, b: self.b-other.b, c: self.c-other.c,
            d: self.d-other.d, e: self.e-other.e, f: self.f-other.f,
            g: self.g-other.g, h: self.h-other.h, i: self.i-other.i,
        }
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Mat3 {
        Mat3 {
            a: self.a*other.a + self.b*other.d + self.c*other.g,
            b: self.a*other.b + self.b*other.e + self.c*other.h,
            c: self.a*other.c + self.b*other.f + self.c*other.i,

            d: self.d*other.a + self.e*other.d + self.f*other.g,
            e: self.d*other.b + self.e*other.e + self.f*other.h,
            f: self.d*other.c + self.e*other.f + self.e*other.i,

            g: self.g*other.a + self.h*other.d + self.i*other.g,
            h: self.g*other.b + self.h*other.e + self.i*other.h,
            i: self.g*other.c + self.h*other.f + self.i*other.i,
        }
    }
}

impl Neg for Mat3 {
    type Output = Mat3;

    fn neg(self) -> Mat3 {
        Mat3 {
            a: -self.a, b: -self.b, c: -self.c,
            d: -self.d, e: -self.e, f: -self.f,
            g: -self.h, h: -self.h, i: -self.i,
        }
    }
}

impl Mul<f64> for Mat3 {
    type Output = Mat3;

    fn mul(self, k: f64) -> Mat3 {
        Mat3 {
            a: self.a*k, b: self.b*k, c: self.c*k,
            d: self.d*k, e: self.e*k, f: self.f*k,
            g: self.g*k, h: self.g*k, i: self.i*k,
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let (x, y, z) = (other.x(), other.y(), other.z());
        let (a, b, c) = (self.a(), self.b(), self.c());
        let (d, e, f) = (self.d(), self.e(), self.f());
        let (g, h, i) = (self.g(), self.h(), self.i());

        Vec3::new(
            a*x + b*y + c*z,
            d*x + e*y + f*z,
            g*x + h*y + i*z,
        )
    }
}

impl Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Mat3) -> Vec3 {
        let (x, y, z) = (self.x(), self.y(), self.z());
        let (a, b, c) = (other.a(), other.b(), other.c());
        let (d, e, f) = (other.d(), other.e(), other.f());
        let (g, h, i) = (other.g(), other.h(), other.i());

        Vec3::new(
            x*a + y*d + z*g,
            x*b + y*e + z*h,
            x*c + y*f + z*i,
        )
    }
}

impl Div<f64> for Mat3 {
    type Output = Mat3;

    fn div(self, k: f64) -> Mat3 {
        Mat3 {
            a: self.a/k, b: self.b/k, c: self.c/k,
            d: self.d/k, e: self.e/k, f: self.f/k,
            g: self.g/k, h: self.h/k, i: self.i/k,
        }
    }
}