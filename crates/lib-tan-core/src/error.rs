use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_stringable_arg, module_util::require_module},
};

// #todo error wrap.
// #todo error pretty-print / format-pretty
// #todo error variant (don't use the word `kind` reserved for type-system)

pub fn error_new(args: &[Expr]) -> Result<Expr, Error> {
    let reason = unpack_stringable_arg(args, 0, "reason")?;
    Ok(Expr::error(reason))
}

pub fn setup_lib_error(context: &mut Context) {
    // #todo put in 'error' / 'err' or path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    // #todo consider `Err`.
    module.insert_invocable("Error", Expr::foreign_func(&error_new));
}
