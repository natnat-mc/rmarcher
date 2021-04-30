use crate::object::Obj;
use crate::structs::{Mat3, Vec3, O, I3};
use crate::material::Material;
use crate::light::Light;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AffineTransform<T: Obj + Clone> {
    obj: T,
    transform: Mat3,
    transform_inv: Mat3,
    translate: Vec3,
    scale: f64
}

impl<T: Obj + Clone> AffineTransform<T> {
    pub fn new(obj: T, transform: Mat3, translate: Vec3) -> AffineTransform<T> {
        AffineTransform { obj, transform, transform_inv: transform.invert(), translate, scale: transform.det().cbrt() }
    }
    pub fn new_linear(obj: T, transform: Mat3) -> AffineTransform<T> {
        AffineTransform { obj, transform, transform_inv: transform.invert(), translate: O, scale: transform.det().cbrt() }
    }
    pub fn new_translate(obj: T, translate: Vec3) -> AffineTransform<T> {
        AffineTransform { obj, transform: I3, transform_inv: I3, translate, scale: 1. }
    }

    fn apply_rev(&self, point: Vec3) -> Vec3 {
        self.transform_inv*point - self.translate
    }
    fn apply_fwd(&self, point: Vec3) -> Vec3 {
        self.transform*point + self.translate
    }
}

pub const SWAP_XY: Mat3 = Mat3::new(
    0., 1., 0.,
    1., 0., 0.,
    0., 0., 1.,
);
pub const SWAP_XZ: Mat3 = Mat3::new(
    0., 0., 1.,
    0., 1., 0.,
    1., 0., 0.,
);
pub const SWAP_YZ: Mat3 = Mat3::new(
    1., 0., 0.,
    0., 0., 1.,
    0., 1., 0.,
);

pub const fn scale_xyz(x: f64, y: f64, z: f64) -> Mat3 {
    Mat3::new(
        x , 0., 0.,
        0., y , 0.,
        0., 0., z ,
    )
}
pub const fn scale(k: f64) -> Mat3 { scale_xyz(k, k, k) }
pub const fn scale_x(k: f64) -> Mat3 { scale_xyz(k, 1., 1.) }
pub const fn scale_y(k: f64) -> Mat3 { scale_xyz(1., k, 1.) }
pub const fn scale_z(k: f64) -> Mat3 { scale_xyz(1., 1., k) }

impl<T: Obj + Clone> Obj for AffineTransform<T> {
    fn distance_to(&self, point: Vec3) -> f64 {
        self.obj.distance_to(self.apply_rev(point)) * self.scale
    }
    fn normal_at(&self, point: Vec3) -> Vec3 {
        self.apply_fwd(self.obj.normal_at(self.apply_rev(point))).unit()
    }
    fn material_at(&self, point: Vec3) -> Material {
        self.obj.material_at(self.apply_rev(point))
    }
    fn get_lights(&self) -> Vec<Light> {
        self.obj
            .get_lights()
            .into_iter()
            .map(|light| {
                Light::new(self.apply_fwd(light.pos()), light.color())
            })
            .collect()
    }
    fn node_count(&self) -> u32 {
        self.obj.node_count() + 1
    }
}