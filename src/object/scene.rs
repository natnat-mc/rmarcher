use crate::object::{Obj, ObjClone};
use crate::structs::Vec3;
use crate::material::Material;
use crate::light::Light;

pub struct Scene {
    scene: Box<dyn ObjClone>
}

impl Scene {
    pub fn new<T: 'static + ObjClone>(scene: T) -> Scene {
        Scene { scene: Box::new(scene) }
    }
    pub fn new_from_box(scene: Box<dyn ObjClone>) -> Scene {
        Scene { scene }
    }
}

impl Obj for Scene {
    fn distance_to(&self, point: Vec3) -> f64 { self.scene.distance_to(point) }
    fn normal_at(&self, point: Vec3) -> Vec3 { self.scene.normal_at(point) }
    fn material_at(&self, point: Vec3) -> Material { self.scene.material_at(point) }
    fn get_lights(&self) -> Vec<Light> { self.scene.get_lights() }
    fn node_count(&self) -> u32 { self.scene.node_count() + 1 }
}

impl Clone for Scene {
    fn clone(&self) -> Scene {
        Scene { scene: self.scene.clone_obj() }
    }
}