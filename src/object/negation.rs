use crate::object::Obj;
use crate::structs::Vec3;
use crate::material::Material;
use crate::light::Light;

#[derive(Debug, Clone, PartialEq)]
pub struct Negation<T: Obj> {
    obj: T
}

impl<T: Obj> Negation<T> {
    pub fn new(obj: T) -> Negation<T> {
        Negation { obj }
    }
}

impl<T: Obj> Obj for Negation<T> {
    fn distance_to(&self, point: Vec3) -> f64 {
        -self.obj.distance_to(point)
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        -self.obj.normal_at(point)
    }
    fn material_at(&self, point: Vec3) -> Material {
        self.obj.material_at(point)
    }
    fn get_lights(&self) -> Vec<Light> {
        self.obj.get_lights()
    }
    fn node_count(&self) -> u32 {
        self.obj.node_count() + 1
    }
}
