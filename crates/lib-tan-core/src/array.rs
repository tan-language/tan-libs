use tan::{
    context::Context,
    error::Error,
    eval::insert_symbol_binding,
    expr::{expr_clone, Expr},
    util::{args::unpack_array_arg, module_util::require_module},
};

// #todo Consider using a magic name.
// #todo Consider using a map, and not binding in the context!
pub fn array_destructure_bind(args: &[Expr], context: &mut Context) -> Result<Expr, Error> {
    let value = unpack_array_arg(args, 0, "array")?;
    // #insight The pattern is also an array.
    let pattern = unpack_array_arg(args, 1, "pattern")?;

    // #todo temp, NASTY code.

    // #todo check if the item count matches, report mismatches.
    for (i, name) in pattern.iter().enumerate() {
        let Some(sym) = name.as_symbol() else {
            return Err(Error::invalid_arguments(
                "malformed destructuring bind, array pattern should contain symbols",
                name.range(),
            ));
        };
        if sym == "_" {
            continue;
        }

        // #insight '...' is called `ellipsis`.

        // #todo consider `..._` for ignoring?
        if sym == "..." {
            break;
        }

        if sym.starts_with("...") {
            insert_symbol_binding(
                &sym[3..],
                &pattern[i].range(),
                Expr::array(&value[i..]),
                context,
            )?;
        } else {
            insert_symbol_binding(
                sym,
                &name.range(),
                expr_clone(value.get(i).unwrap()),
                context,
            )?;
        }
    }

    Ok(Expr::None)
}

pub fn import_lib_array(context: &mut Context) {
    // #todo should put in `array` module and then into `prelude`.
    let module = require_module("prelude", context);

    // #todo Consider a magic name.
    // #todo Consider adding `!` suffix.
    module.insert_invocable(
        "destructure-bind$$Array$$Array",
        Expr::foreign_func_mut_context(&array_destructure_bind),
    );
}
