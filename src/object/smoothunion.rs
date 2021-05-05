use crate::object::Obj;
use crate::light::Light;
use crate::structs::Vec3;
use std::vec;
use crate::material::Material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SmoothUnion<A: Obj, B: Obj> {
    a: A,
    b: B,
    r: f64
}

impl<A: Obj, B: Obj> SmoothUnion<A, B> {
    pub fn new(a: A, b: B, r: f64) -> SmoothUnion<A, B> {
        SmoothUnion { a, b, r }
    }
}

impl<A: Obj, B: Obj> Obj for SmoothUnion<A, B> {
    fn distance_to(&self, point: Vec3) -> f64 {
        let d1 = self.a.distance_to(point);
        let d2 = self.b.distance_to(point);
        let h = f64::max(self.r - f64::abs(d1-d2),0.);
        d1.min(d2) - h*h*0.25 / self.r
    }
    fn material_at(&self, point: Vec3) -> Material {
        if self.a.distance_to(point) < self.b.distance_to(point) {
            self.a.material_at(point)
        } else {
            self.b.material_at(point)
        }
    }
    fn get_lights(&self) -> vec::Vec<Light> {
        let mut l = self.a.get_lights();
        l.extend(self.b.get_lights());
        l
    }
    fn node_count(&self) -> u32 {
        self.a.node_count() + self.b.node_count() + 1
    }
}
