use crate::object::Obj;
use crate::material::Material;
use crate::structs::Vec3;
use crate::light::Light;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WithMaterial<T: Obj> {
    obj: T,
    material: Material
}

impl<T: Obj> WithMaterial<T> {
    pub fn new(obj: T, material: Material) -> WithMaterial<T> {
        WithMaterial { obj, material }
    }
}

impl<T: Obj> Obj for WithMaterial<T> {
    fn distance_to(&self, point: Vec3) -> f64 {
        self.obj.distance_to(point)
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        self.obj.normal_at(point)
    }
    fn material_at(&self, point: Vec3) -> Material {
        self.material
    }
    fn get_lights(&self) -> Vec<Light> {
        self.obj.get_lights()
    }
    fn node_count(&self) -> u32 {
        self.obj.node_count() + 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithDynamicMaterial<T: Obj, F: Clone + Send + Sync + Fn(Vec3) -> Material> {
    obj: T,
    fun: F
}

impl<T: Obj, F: Clone + Send + Sync + Fn(Vec3) -> Material> WithDynamicMaterial<T, F> {
    pub fn new(obj: T, fun: F) -> WithDynamicMaterial<T, F> {
        WithDynamicMaterial { obj, fun }
    }
}

impl<T: Obj, F: Clone + Send + Sync + Fn(Vec3) -> Material> Obj for WithDynamicMaterial<T, F> {
    fn distance_to(&self, point: Vec3) -> f64 {
        self.obj.distance_to(point)
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        self.obj.normal_at(point)
    }
    fn material_at(&self, point: Vec3) -> Material {
        (self.fun)(point)
    }
    fn get_lights(&self) -> Vec<Light> {
        self.obj.get_lights()
    }
    fn node_count(&self) -> u32 {
        self.obj.node_count() + 1
    }
}