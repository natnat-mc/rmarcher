use rlua::{Context, Table, Error};
use crate::object::{SWAP_XY, SWAP_YZ, SWAP_XZ, scale_xyz, scale, scale_x, scale_y, scale_z};
use crate::structs::{Mat3, I3};

pub fn transform<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("SWAPXY", SWAP_XY)?;
    module.set("SWAPXZ", SWAP_XZ)?;
    module.set("SWAPYZ", SWAP_YZ)?;

    module.set("scalexyz", ctx.create_function(
        |ctx, (x, y, z)| Ok(scale_xyz(x, y, z))
    )?)?;
    module.set("scale", ctx.create_function(
        |ctx, k| Ok(scale(k))
    )?)?;

    module.set("scalex", ctx.create_function(
        |ctx, k| Ok(scale_x(k))
    )?)?;
    module.set("scaley", ctx.create_function(
        |ctx, k| Ok(scale_y(k))
    )?)?;
    module.set("scalez", ctx.create_function(
        |ctx, k| Ok(scale_z(k))
    )?)?;

    module.set("stack", ctx.create_function(
        |ctx, transforms: Vec<Mat3>| {
            let mut acc = I3;
            for trans in transforms.iter().rev().cloned() {
                acc = trans * I3;
            }
            Ok(acc)
        }
    )?)?;

    Ok(module)
}