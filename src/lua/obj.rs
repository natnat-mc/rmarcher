use crate::object::*;
use crate::structs::{Vec3, Mat3};
use std::sync::Arc;
use crate::material::Material;
use crate::light::Light;
use rlua::{UserData, Context, Table};

#[derive(Clone)]
pub struct LuaObject(Arc<Scene>);

impl LuaObject {
    fn new<T: 'static + Obj + Clone>(obj: T) -> rlua::Result<LuaObject> {
        Ok(LuaObject(Arc::new(Scene::new(obj))))
    }
    fn get(self) -> Scene {
        (*self.0).clone()
    }
}

impl Obj for LuaObject {
    fn distance_to(&self, point: Vec3) -> f64 { self.0.distance_to(point) }
    fn normal_at(&self, point: Vec3) -> Vec3 { self.0.normal_at(point) }
    fn material_at(&self, point: Vec3) -> Material { self.0.material_at(point) }
    fn get_lights(&self) -> Vec<Light> { self.0.get_lights() }
    fn node_count(&self) -> u32 { 1 + self.0.node_count() }
}

impl UserData for LuaObject {}

pub fn obj(ctx: Context, _: ()) -> rlua::Result<Table> {
    let module = ctx.create_table()?;

    module.set("cuboid", ctx.create_function(
        |ctx, (pos, radius): (Vec3, Vec3)| LuaObject::new(Cuboid::new(pos, radius))
    )?)?;

    module.set("cylinder", ctx.create_function(
        |ctx, (pos, radius): (Vec3, f64)| LuaObject::new(Cylinder::new(pos, radius))
    )?)?;

    module.set("exclusion", ctx.create_function(
        |ctx, (a, b): (LuaObject, LuaObject)| LuaObject::new(Exclusion::new(a.get(), b.get()))
    )?)?;

    module.set("intersection", ctx.create_function(
        |ctx, (a, b): (LuaObject, LuaObject)| LuaObject::new(Intersection::new(a.get(), b.get()))
    )?)?;

    module.set("plane", ctx.create_function(
        |ctx, (normal, offset): (Vec3, f64)| LuaObject::new(Plane::new(normal, offset))
    )?)?;

    module.set("sphere", ctx.create_function(
        |ctx, (pos, radius): (Vec3, f64)| LuaObject::new(Sphere::new(pos, radius))
    )?)?;

    module.set("torus", ctx.create_function(
        |ctx, (center, radius, thickness): (Vec3, f64, f64)| LuaObject::new(Torus::new(center, radius, thickness))
    )?)?;

    module.set("affinetransform", ctx.create_function(
        |ctx, (obj, trans, disp): (LuaObject, Mat3, Vec3)| LuaObject::new(AffineTransform::new(obj.get(), trans, disp))
    )?)?;

    module.set("union", ctx.create_function(
        |ctx, (a, b): (LuaObject, LuaObject)| LuaObject::new(Union::new(a.get(), b.get()))
    )?)?;

    module.set("waves", ctx.create_function(
        |ctx, ()| LuaObject::new(Waves::new())
    )?)?;

    module.set("withlights", ctx.create_function(
        |ctx, (obj, light): (LuaObject, Light)| LuaObject::new(WithLights::new_one(obj.get(), light))
    )?)?;

    module.set("withmaterial", ctx.create_function(
        |ctx, (obj, material): (LuaObject, Material)| LuaObject::new(WithMaterial::new(obj.get(), material))
    )?)?;

    //TODO add the rest of the objects

    Ok(module)
}
