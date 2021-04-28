use crate::object::Obj;
use crate::light::Light;
use crate::structs::Vec3;
use std::vec;
use crate::material::Material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Exclusion<A: Obj, B: Obj> {
    a: A,
    b: B
}

impl<A: Obj, B: Obj> Exclusion<A, B> {
    pub fn new(a: A, b: B) -> Exclusion<A, B> {
        Exclusion { a, b }
    }
}

impl<A: Obj, B: Obj> Obj for Exclusion<A, B> {
    fn distance_to(&self, point: Vec3) -> f64 {
        f64::max(self.a.distance_to(point), -self.b.distance_to(point))
    }
    fn material_at(&self, point: Vec3) -> Material {
        self.a.material_at(point)
    }
    fn get_lights(&self) -> vec::Vec<Light> {
        self.a.get_lights()
    }
    fn node_count(&self) -> u32 {
        self.a.node_count() + self.b.node_count() + 1
    }
}
