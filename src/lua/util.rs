use rlua::{Context, Table};

pub fn util<'lua>(ctx: Context<'lua>, env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    ctx.load(include_str!("util.lua")).call(env)
}