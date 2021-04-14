use crate::structs::Vec3;
use crate::object::Obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cylinder {
    center: Vec3,
    radius: f64
}

impl Cylinder {
    pub const fn new(center: Vec3, radius: f64) -> Cylinder {
        Cylinder { center, radius }
    }
}

impl Obj for Cylinder {
    fn distance_to(&self, point: Vec3) -> f64 {
        Vec3::new(self.center.x(), 0., self.center.z()).distance_to(point) - self.radius
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        point - Vec3::new(self.center.x(), 0.,self.center.z())
    }
}
