use rlua::{Context, Table, Lua, Function};

mod obj;
mod vec3;
mod mat3;
mod color;
mod material;
mod light;
mod transform;
mod util;

pub use obj::{LuaObject, obj};
pub use vec3::vec3;
pub use mat3::mat3;
pub use color::{color_vec, color_mat};
pub use material::{surface_type, material};
pub use light::light;
pub use transform::transform;
pub use util::util;

pub fn add_scene_env<'lua>(ctx: Context<'lua>, env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    env.set("obj", obj(ctx, env.clone())?)?;
    env.set("vec3", vec3(ctx, env.clone())?)?;
    env.set("mat3", mat3(ctx, env.clone())?)?;
    env.set("colorvec", color_vec(ctx, env.clone())?)?;
    env.set("colormat", color_mat(ctx, env.clone())?)?;
    env.set("surfacetype", surface_type(ctx, env.clone())?)?;
    env.set("material", material(ctx, env.clone())?)?;
    env.set("light", light(ctx, env.clone())?)?;
    env.set("transform", transform(ctx, env.clone())?)?;
    env.set("util", util(ctx, env.clone())?)?;

    Ok(env)
}

pub fn scene_from_file(file: String) -> rlua::Result<LuaObject> {
    Lua::new().context(|ctx| {
        let env = ctx.create_table()?;
        add_scene_env(ctx, env.clone())?;
        let meta = ctx.create_table()?;
        meta.set("__index", ctx.globals())?;
        env.set_metatable(Some(meta));

        let loadfile: Function = ctx.globals().get("loadfile")?;
        let chunk: Function = loadfile.call((file, "t", env))?;
        ctx.unpack(chunk.call(())?)
    })
}