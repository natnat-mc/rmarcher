use crate::light::Light;
use rlua::{UserData, Context, Table};

impl UserData for Light {}

pub fn light<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (pos, color)| Ok(Light::new(pos, color))
    )?)?;

    Ok(module)
}