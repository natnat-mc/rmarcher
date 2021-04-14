use crate::structs::Vec3;
use crate::object::Obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Torus {
    center: Vec3,
    radius: f64,
    thickness: f64
}

impl Torus {
    pub const fn new(center: Vec3, radius: f64, thickness: f64) -> Torus {
        Torus { center, radius, thickness }
    }
    pub const fn new_xyz(x: f64, y: f64, z: f64, radius: f64, thickness: f64) -> Torus {
        Torus::new(Vec3::new(x, y, z), radius, thickness)
    }
}

impl Obj for Torus {
    fn distance_to(&self, point: Vec3) -> f64 {
        let (dx, dy) = (point.x()-self.center.x(), point.y()-self.center.y());
        let dh = f64::abs(f64::sqrt(dx*dx + dy*dy) - self.radius);
        let dz = point.z() - self.center.z();
        f64::sqrt(dh*dh + dz*dz) - self.thickness
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        let centered = point - self.center;
        let centered_no_z= Vec3::new(centered.x(), centered.y(), 0.);
        let closest = centered_no_z.unit() * self.radius;
        (centered - closest).unit()
    }
}