use crate::object::Obj;
use crate::structs::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Waves {}

impl Waves {
    pub const fn new() -> Waves {
        Waves {}
    }
}

impl Obj for Waves {
    fn distance_to(&self, point: Vec3) -> f64 {
        let dist = point.x().sin() + point.y().sin() + point.z();
        dist / f64::sqrt(3.)
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        Vec3::new(-point.x().cos(), -point.y().cos(), 1.).unit()
    }
}