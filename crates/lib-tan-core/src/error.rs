use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        args::{unpack_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

// #todo error wrap.
// #todo error pretty-print / format-pretty
// #todo error variant (don't use the word `kind` reserved for type-system)

fn error_new(args: &[Expr]) -> Result<Expr, Error> {
    let reason = unpack_stringable_arg(args, 0, "reason")?;
    Ok(Expr::error(reason))
}

fn is_error(args: &[Expr]) -> Result<Expr, Error> {
    let arg = unpack_arg(args, 0, "value")?;
    if let Expr::Error(_) = arg {
        Ok(Expr::Bool(true))
    } else {
        Ok(Expr::Bool(false))
    }
}

pub fn setup_lib_error(context: &mut Context) {
    // #todo put in 'error' / 'err' or path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    // #todo Consider `Err`.
    module.insert_invocable("Error", Expr::foreign_func(&error_new));
    // #todo Consider just `error?`.
    module.insert_invocable("is-error?", Expr::foreign_func(&is_error));
}
