use crate::structs::Vec3;
use crate::object::Obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pos: Vec3,
    radius: f64
}

impl Sphere {
    pub const fn new(pos: Vec3, radius: f64) -> Sphere {
        Sphere { pos, radius }
    }
    pub const fn new_xyz(x: f64, y: f64, z: f64, radius: f64) -> Sphere {
        let pos = Vec3::new(x, y, z);
        Sphere { pos, radius }
    }
}

impl Obj for Sphere {
    fn distance_to(&self, point: Vec3) -> f64 {
        self.pos.distance_to(point) - self.radius
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.pos).unit()
    }
}