// #todo Maybe as optimization, use special handling in eval?

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        args::{unpack_float_arg, unpack_int_arg},
        module_util::require_module,
    },
};

// #todo #think Intentionally don't provide a u8_from_float to avoid type coercion? Not sure, revisit this.

pub fn u8_new(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support more 'source' types.
    let value = unpack_int_arg(args, 0, "value")?;

    if !(0..256).contains(&value) {
        return Err(Error::invalid_arguments(
            "U8 values should be in 0..256",
            args[0].range(),
        ));
    }

    Ok(Expr::U8(value as u8))
}

// #todo Make this explicit at call site?
pub fn u8_from_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support more 'source' types.
    let value = unpack_float_arg(args, 0, "value")?;

    if !(0.0..256.0).contains(&value) {
        return Err(Error::invalid_arguments(
            "U8 values should be in 0..256",
            args[0].range(),
        ));
    }

    Ok(Expr::U8(value as u8))
}

// #todo rename all setup_xxx functions to import_xxx.
pub fn setup_lib_u8(context: &mut Context) {
    // #todo put in 'u8' path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    module.insert_invocable("U8", Expr::foreign_func(&u8_new));
    module.insert_invocable("U8$$Int", Expr::foreign_func(&u8_new));
    module.insert_invocable("U8$$Float", Expr::foreign_func(&u8_from_float));
}
