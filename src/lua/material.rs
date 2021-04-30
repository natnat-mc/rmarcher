use crate::material::{SurfaceType, Material};
use rlua::{UserData, Context, Table};

impl UserData for SurfaceType {}
impl UserData for Material {}

pub fn surface_type<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("DIFFUSE", SurfaceType::Diffuse)?;
    module.set("REFLECTIVE", SurfaceType::Reflective)?;
    module.set("STOP", SurfaceType::Stop)?;

    Ok(module)
}

pub fn material<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (emission, reflection, surface)| Ok(Material::new(emission, reflection, surface))
    )?)?;

    module.set("newfromdiagonal", ctx.create_function(
        |ctx, (emission, reflection, surface)| Ok(Material::new_from_diagonal(emission, reflection, surface))
    )?)?;

    Ok(module)
}

