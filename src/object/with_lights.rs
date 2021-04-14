use crate::light::Light;
use crate::object::Obj;
use crate::structs::Vec3;
use crate::material::Material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WithLights<T: Obj, const N: usize> {
    obj: T,
    lights: [Light; N]
}

impl<T: Obj, const N: usize> WithLights<T, N> {
    pub fn new(obj: T, lights: [Light; N]) -> WithLights<T, N> {
        WithLights { obj, lights }
    }
}

impl<T: Obj, const N: usize> Obj for WithLights<T, N> {
    fn distance_to(&self, point: Vec3) -> f64 {
        self.obj.distance_to(point)
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        self.obj.normal_at(point)
    }
    fn material_at(&self, point: Vec3) -> Material {
        self.obj.material_at(point)
    }
    fn get_lights(&self) -> Vec<Light> {
        let mut l = self.obj.get_lights();
        l.extend(&self.lights);
        l
    }
    fn node_count(&self) -> u32 {
        self.obj.node_count() + 1
    }
}

pub type WithLight<T> = WithLights<T, 1>;

impl<T: Obj> WithLight<T> {
    pub fn new_one(obj: T, light: Light) -> WithLight<T> {
        WithLight { obj, lights: [light; 1] }
    }
}