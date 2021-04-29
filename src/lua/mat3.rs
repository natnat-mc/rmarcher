use crate::structs::{Mat3, I3, O3};
use rlua::{UserData, UserDataMethods, Context, MetaMethod, Table};

impl UserData for Mat3 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::Add, |ctx, a: &Self, b: Self| Ok(*a+b));
        methods.add_meta_method(MetaMethod::Sub, |ctx, a: &Self, b: Self| Ok(*a-b));
        methods.add_meta_method(MetaMethod::Mul, |ctx, a: &Self, b: Self| Ok(*a*b));
        methods.add_meta_method(MetaMethod::Unm, |ctx, a: &Self, b: ()| Ok(-*a));
        methods.add_meta_method(MetaMethod::Mul, |ctx, a: &Self, b: f64| Ok(*a*b));
        methods.add_meta_method(MetaMethod::Div, |ctx, a: &Self, b: f64| Ok(*a/b));

        methods.add_method("a", |ctx, x, ()| Ok(x.a()));
        methods.add_method("b", |ctx, x, ()| Ok(x.b()));
        methods.add_method("c", |ctx, x, ()| Ok(x.c()));
        methods.add_method("d", |ctx, x, ()| Ok(x.d()));
        methods.add_method("e", |ctx, x, ()| Ok(x.e()));
        methods.add_method("f", |ctx, x, ()| Ok(x.f()));
        methods.add_method("g", |ctx, x, ()| Ok(x.g()));
        methods.add_method("h", |ctx, x, ()| Ok(x.h()));
        methods.add_method("i", |ctx, x, ()| Ok(x.i()));

        methods.add_method("det", |ctx, x, ()| Ok(x.det()));
        methods.add_method("trans",|ctx, x, ()| Ok(x.trans()));
        methods.add_method("minor", |ctx, x, ()| Ok(x.minor()));
        methods.add_method("invert", |ctx, x, ()| Ok(x.invert()));

        methods.add_method("membermul", |ctx, x, y| Ok(x.member_mul(y)));
    }
}

pub fn mat3(ctx: Context, _: ()) -> rlua::Result<Table> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (a, b, c, d, e, f, g, h, i)| Ok(Mat3::new(a, b, c, d, e, f, g, h, i))
    )?)?;

    module.set("I", I3)?;
    module.set("O", O3)?;

    Ok(module)
}
