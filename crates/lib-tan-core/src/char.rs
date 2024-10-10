// #todo maybe as optimization, use special handling in eval?

// #todo create char from Num/Int also
// #todo function to get char int code
// #todo function to join chars into a string

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        args::{unpack_char_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

pub fn char_new(args: &[Expr]) -> Result<Expr, Error> {
    // #todo also support Int as argument.
    let c = unpack_stringable_arg(args, 0, "c")?;

    if c.len() != 1 {
        return Err(Error::invalid_arguments(
            "the string argument should be one char long",
            args[0].range(),
        ));
    }

    let c = c.chars().next().unwrap();

    Ok(Expr::Char(c))
}

pub fn char_is_uppercase(args: &[Expr]) -> Result<Expr, Error> {
    let c = unpack_char_arg(args, 0, "char")?;
    Ok(Expr::Bool(c.is_uppercase()))
}

pub fn char_is_lowercase(args: &[Expr]) -> Result<Expr, Error> {
    let c = unpack_char_arg(args, 0, "char")?;
    Ok(Expr::Bool(c.is_lowercase()))
}

// #todo rename all setup_xxx functions to import_xxx.
pub fn setup_lib_char(context: &mut Context) {
    // #todo put in 'char' path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    module.insert_invocable("Char", Expr::foreign_func(&char_new));

    module.insert_invocable("is-upper-case?", Expr::foreign_func(&char_is_uppercase));
    module.insert_invocable("is-lower-case?", Expr::foreign_func(&char_is_lowercase));
}
