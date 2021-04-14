use crate::structs::Vec3;
use crate::object::Obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    normal: Vec3,
    offset: f64
}

impl Plane {
    pub fn new(normal: Vec3, offset: f64) -> Plane {
        let l = normal.magnitude();
        Plane { normal: normal/l, offset: offset/l }
    }
    pub fn new_xyz(x: f64, y: f64, z: f64, offset: f64) -> Plane {
        let normal = Vec3::new(x, y, z);
        Plane::new(normal, offset)
    }
    pub const unsafe fn new_raw(normal: Vec3, offset: f64) -> Plane {
        Plane { normal, offset }
    }
}

impl Obj for Plane {
    fn distance_to(&self, point: Vec3) -> f64 {
        point*self.normal + self.offset
    }
    fn normal_at(&self, _point: Vec3) -> Vec3 {
        self.normal
    }
}