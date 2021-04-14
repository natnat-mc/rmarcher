use crate::structs::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    from: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(from: Vec3, dir: Vec3) -> Ray {
        Ray { from, dir }
    }

    pub fn new_from_to(from: Vec3, to: Vec3) -> Ray {
        Ray { from, dir: to - from }
    }

    pub fn unit(&self) -> Ray {
        Ray { from: self.from, dir: self.dir.unit() }
    }

    pub fn from(&self) -> Vec3 { self.from }
    pub fn dir(&self) -> Vec3 { self.dir }

    pub fn point(&self, steps: f64) -> Vec3 {
        self.from + self.dir * steps
    }
}