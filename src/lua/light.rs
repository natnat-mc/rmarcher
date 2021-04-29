use crate::light::Light;
use rlua::{UserData, Context, Table};

impl UserData for Light {}

pub fn light(ctx: Context, _: ()) -> rlua::Result<Table> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (pos, color)| Ok(Light::new(pos, color))
    )?)?;

    Ok(module)
}