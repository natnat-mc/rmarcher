use crate::structs::vec_n::Vec;
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat<const N: usize> {
    v: [[f64; N]; N],
}

impl<const N: usize> Mat<N> {
    pub const fn new(v: [[f64; N]; N]) -> Mat<N> {
        Mat { v }
    }

    pub const fn new_zero() -> Mat<N> {
        Mat { v: [[0.; N]; N] }
    }

    pub const fn new_unit() -> Mat<N> {
        let mut v = [[0.; N]; N];
        let mut i = 0;
        while i<N {
            v[i][i] = 1.;
            i += 1;
        }
        Mat { v }
    }

    pub const fn new_one(i: usize, j: usize) -> Mat<N> {
        let mut v = [[0.; N]; N];
        v[i][j] = 1.;
        Mat { v }
    }

    pub const fn new_from_diagonal(d: Vec<N>) -> Mat<N> {
        let mut v = [[0.; N]; N];
        let mut i = 0;
        while i<N {
            v[i][i] = d.v(i);
            i += 1;
        }
        Mat { v }
    }

    pub const fn v(&self, i: usize, j: usize) -> f64 {
        self.v[i][j]
    }
}

impl<const N: usize> Add for Mat<N> {
    type Output = Mat<N>;

    fn add(self, other: Mat<N>) -> Mat<N> {
        let mut v = [[0.; N]; N];
        for i in 0..N {
            for j in 0..N {
                v[i][j] = self.v[i][j] + other.v[i][j];
            }
        }
        Mat { v }
    }
}

impl<const N: usize> Sub for Mat<N> {
    type Output = Mat<N>;

    fn sub(self, other: Mat<N>) -> Mat<N> {
        let mut v = [[0.; N]; N];
        for i in 0..N {
            for j in 0..N {
                v[i][j] = self.v[i][j] - other.v[i][j];
            }
        }
        Mat { v }
    }
}

impl<const N: usize> Neg for Mat<N> {
    type Output = Mat<N>;

    fn neg(self) -> Mat<N> {
        let mut v = [[0.; N]; N];
        for i in 0..N {
            for j in 0..N {
                v[i][j] = -self.v[i][j];
            }
        }
        Mat { v }
    }
}

impl<const N: usize> Mul<f64> for Mat<N> {
    type Output = Mat<N>;

    fn mul(self, k: f64) -> Mat<N> {
        let mut v = [[0.; N]; N];
        for i in 0..N {
            for j in 0..N {
                v[i][j] = self.v[i][j] * k;
            }
        }
        Mat { v }
    }
}

impl<const N: usize> Mul<Vec<N>> for Mat<N> {
    type Output = Vec<N>;

    fn mul(self, other: Vec<N>) -> Vec<N> {
        let mut v = [0.; N];
        for i in 0..N {
            let mut x = 0.;
            for j in 0..N {
                x += self.v[i][j] * other.v(i);
            }
            v[i] = x;
        }
        Vec::new(v)
    }
}

impl<const N: usize> Div<f64> for Mat<N> {
    type Output = Mat<N>;

    fn div(self, k: f64) -> Mat<N> {
        let mut v = [[0.; N]; N];
        for i in 0..N {
            for j in 0..N {
                v[i][j] = self.v[i][j] / k;
            }
        }
        Mat { v }
    }
}