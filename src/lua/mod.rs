use rlua::{Context, Table, Lua, Function};

mod obj;
mod vec3;
mod mat3;
mod color;
mod material;
mod light;

pub use obj::{LuaObject, obj};
pub use vec3::vec3;
pub use mat3::mat3;
pub use color::{color_vec, color_mat};
pub use material::{surface_type, material};
pub use light::light;

pub fn env(ctx: Context, _: ()) -> rlua::Result<Table> {
    let module = ctx.create_table()?;

    module.set("obj", obj(ctx, ())?)?;
    module.set("vec3", vec3(ctx, ())?)?;
    module.set("mat3", mat3(ctx, ())?)?;
    module.set("colorvec", color_vec(ctx, ())?)?;
    module.set("colormat", color_mat(ctx, ())?)?;
    module.set("surfacetype", surface_type(ctx, ())?)?;
    module.set("material", material(ctx, ())?)?;
    module.set("light", light(ctx, ())?)?;

    Ok(module)
}

pub fn scene_from_file(file: String) -> rlua::Result<LuaObject> {
    Lua::new().context(|ctx| {
        let env = env(ctx, ())?;
        let meta = ctx.create_table()?;
        meta.set("__index", ctx.globals())?;
        env.set_metatable(Some(meta));

        let loadfile: Function = ctx.globals().get("loadfile")?;
        let chunk: Function = loadfile.call((file, "t", env))?;
        ctx.unpack(chunk.call(())?)
    })
}