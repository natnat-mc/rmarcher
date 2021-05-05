use crate::consts::{EPSILON, MAX_DIST};
use crate::structs::{Vec3, X, Y, Z};
use crate::material::Material;
use crate::material;
use crate::light::Light;
use std::vec::Vec;

mod sphere;
mod plane;
mod union;
mod smoothunion;
mod intersection;
mod exclusion;
mod negation;
mod cuboid;
mod cylinder;
mod torus;
mod waves;
mod with_material;
mod with_lights;
mod transform;
mod scene;

pub trait Obj: Send + Sync {
    fn distance_to(&self, _point: Vec3) -> f64 {
        MAX_DIST
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        let v = self.distance_to(point);
        let x = self.distance_to(point + X*EPSILON) - v;
        let y = self.distance_to(point + Y*EPSILON) - v;
        let z = self.distance_to(point + Z*EPSILON) - v;

        Vec3::new(x, y, z).unit()
    }
    fn material_at(&self, _point: Vec3) -> Material {
        material::WHITE
    }
    fn get_lights(&self) -> Vec<Light> {
        Vec::new()
    }
    fn node_count(&self) -> u32 {
        1
    }
}

impl<T: Obj> Obj for &T {
    fn distance_to(&self, point: Vec3) -> f64 { (*self).distance_to(point) }
    fn normal_at(&self, point: Vec3) -> Vec3 { (*self).normal_at(point) }
    fn material_at(&self, point: Vec3) -> Material { (*self).material_at(point) }
    fn get_lights(&self) -> Vec<Light> { (*self).get_lights() }
    fn node_count(&self) -> u32 { (*self).node_count() }
}

pub trait ObjClone: Obj {
    fn clone_obj(&self) -> Box<dyn ObjClone>;
}

impl<T: 'static + Obj + Clone> ObjClone for T {
    fn clone_obj(&self) -> Box<dyn ObjClone> {
        Box::new(self.clone())
    }
}

pub use sphere::Sphere;
pub use plane::Plane;
pub use union::Union;
pub use smoothunion::SmoothUnion;
pub use intersection::Intersection;
pub use exclusion::Exclusion;
pub use negation::Negation;
pub use cuboid::Cuboid;
pub use cylinder::Cylinder;
pub use torus::Torus;
pub use waves::Waves;
pub use with_material::{WithMaterial, WithDynamicMaterial};
pub use with_lights::{WithLights, WithLight, WithAnyLights};
pub use transform::*;
pub use scene::Scene;