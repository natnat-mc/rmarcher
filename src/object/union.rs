use crate::object::Obj;
use crate::light::Light;
use crate::structs::Vec3;
use std::vec;
use crate::material::Material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Union<A: Obj, B: Obj> {
    a: A,
    b: B
}

impl<A: Obj, B: Obj> Union<A, B> {
    pub fn new(a: A, b: B) -> Union<A, B> {
        Union { a, b }
    }
}

impl<A: Obj, B: Obj> Obj for Union<A, B> {
    fn distance_to(&self, point: Vec3) -> f64 {
        f64::min(self.a.distance_to(point), self.b.distance_to(point))
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        if self.a.distance_to(point) < self.b.distance_to(point) {
            self.a.normal_at(point)
        } else {
            self.b.normal_at(point)
        }
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