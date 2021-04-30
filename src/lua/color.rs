use crate::structs::{ColorVec, ColorMat};
use rlua::{UserData, Context, Table};

impl UserData for ColorVec {}
impl UserData for ColorMat {}

pub fn color_vec<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (r, g, b, u)| Ok(ColorVec::new([r, g, b, u]))
    )?)?;

    module.set("ZERO", ColorVec::new_zero())?;

    Ok(module)
}

pub fn color_mat<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (
            a, b, c, d,
            e, f, g, h,
            i, j, k, l,
            m, n, o, p
        )| Ok(ColorMat::new([
            [a, b, c, d],
            [e, f, g, h],
            [i, j, k, l],
            [m, n, o, p]
        ]))
    )?)?;

    module.set("newfromvec", ctx.create_function(
        |ctx, vec: ColorVec| Ok(ColorMat::new_from_diagonal(vec))
    )?)?;

    module.set("newfromdiagonal", ctx.create_function(
        |ctx, (r, g, b, u)| Ok(ColorMat::new_from_diagonal(ColorVec::new([r, g, b, u])))
    )?)?;

    module.set("ZERO", ColorMat::new_zero())?;

    Ok(module)
}
