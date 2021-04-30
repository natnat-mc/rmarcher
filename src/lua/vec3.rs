use crate::structs::{Vec3, X, Y, Z, O};
use rlua::{UserData, UserDataMethods, MetaMethod, Context, Table};

impl UserData for Vec3 {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::Add, |ctx, a: &Self, b: Self| Ok(*a+b));
        methods.add_meta_method(MetaMethod::Sub, |ctx, a: &Self, b: Self| Ok(*a-b));
        methods.add_meta_method(MetaMethod::Mul, |ctx, a: &Self, b: Self| Ok(*a*b));
        methods.add_meta_method(MetaMethod::Pow, |ctx, a: &Self, b: Self| Ok(*a^b));
        methods.add_meta_method(MetaMethod::Unm, |ctx, a: &Self, b: ()| Ok(-*a));
        methods.add_meta_method(MetaMethod::Mul, |ctx, a: &Self, b: f64| Ok(*a*b));
        methods.add_meta_method(MetaMethod::Div, |ctx, a: &Self, b: f64| Ok(*a/b));

        methods.add_method("x", |ctx, x, ()| Ok(x.x()));
        methods.add_method("y", |ctx, x, ()| Ok(x.y()));
        methods.add_method("z", |ctx, x, ()| Ok(x.z()));

        methods.add_method("magnitude", |ctx, x, ()| Ok(x.magnitude()));
        methods.add_method("unit", |ctx, x, ()| Ok(x.unit()));

        methods.add_method("distanceto", |ctx, x, y| Ok(x.distance_to(y)));
        methods.add_method("angleto", |ctx, x, y| Ok(x.angle_to(y)));
        methods.add_method("membermul", |ctx, x, y| Ok(x.member_mul(y)));
    }
}

pub fn vec3<'lua>(ctx: Context<'lua>, _env: Table<'lua>) -> rlua::Result<Table<'lua>> {
    let module = ctx.create_table()?;

    module.set("new", ctx.create_function(
        |ctx, (x, y, z)| Ok(Vec3::new(x, y, z))
    )?)?;

    module.set("X", X)?;
    module.set("Y", Y)?;
    module.set("Z", Z)?;
    module.set("O", O)?;

    Ok(module)
}