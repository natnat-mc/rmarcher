use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec<const N: usize> {
    v: [f64; N],
}

impl<const N: usize> Vec<N> {
    pub const fn new(v: [f64; N]) -> Vec<N> {
        Vec { v }
    }

    pub const fn new_zero() -> Vec<N> {
        Vec { v: [0.; N] }
    }

    pub const fn new_one(i: usize) -> Vec<N> {
        let mut v = [0.; N];
        v[i] = 1.;
        Vec { v }
    }

    pub const fn v(&self, i: usize) -> f64 {
        self.v[i]
    }

    pub fn magnitude(self) -> f64 {
        let mut x = 0.;
        for i in 0..N {
            let v = self.v[i];
            x += v*v;
        }
        f64::sqrt(x)
    }

    pub fn unit(self) -> Vec<N> {
        self / self.magnitude()
    }

    pub fn distance_to(self, other: Vec<N>) -> f64 {
        (self - other).magnitude()
    }

    pub fn member_mul(self, other: Vec<N>) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = self.v[i] * other.v[i];
        }
        Vec { v }
    }
}

impl<const N: usize> Add for Vec<N> {
    type Output = Vec<N>;

    fn add(self, other: Vec<N>) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = self.v[i] + other.v[i];
        }
        Vec { v }
    }
}

impl<const N: usize> Sub for Vec<N> {
    type Output = Vec<N>;

    fn sub(self, other: Vec<N>) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = self.v[i] - other.v[i];
        }
        Vec { v }
    }
}

impl<const N: usize> Neg for Vec<N> {
    type Output = Vec<N>;

    fn neg(self) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = -self.v[i];
        }
        Vec { v }
    }
}

impl<const N: usize> Mul<f64> for Vec<N> {
    type Output = Vec<N>;

    fn mul(self, k: f64) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = self.v[i] * k;
        }
        Vec { v }
    }
}

impl<const N: usize> Div<f64> for Vec<N> {
    type Output = Vec<N>;

    fn div(self, k: f64) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            v[i] = self.v[i] / k;
        }
        Vec { v }
    }
}
