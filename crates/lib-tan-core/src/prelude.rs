use tan::context::Context;
use tan::error::Error;
use tan::expr::Expr;
use tan::util::module_util::require_module;

use crate::array::import_lib_array;

use super::arithmetic::setup_lib_arithmetic;
use super::bool::setup_lib_bool;
use super::buffer::setup_lib_buffer;
use super::char::setup_lib_char;
use super::cmp::setup_lib_cmp;
use super::eq::setup_lib_eq;
use super::error::setup_lib_error;
use super::float::setup_lib_float;
use super::int::setup_lib_int;
use super::io::setup_lib_io;
use super::lang::setup_lib_lang;
use super::map::setup_lib_map;
use super::maybe::setup_lib_maybe;
use super::range::setup_lib_range;
use super::seq::setup_lib_seq;
use super::string::setup_lib_string;
use super::u8::setup_lib_u8;

// #todo temporarily here, move to String?
/// Formats an expression into a string.
pub fn expr_to_string(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo improve default formatting
    Ok(Expr::string(this.to_string()))
}

// #todo instead of evaluating in prelude maybe it's better to use the functions from the actual modules?
pub fn import_lib_prelude(context: &mut Context) {
    // #todo maybe context.require_module(path) instead?

    setup_lib_arithmetic(context);
    setup_lib_bool(context);
    setup_lib_eq(context);
    setup_lib_cmp(context);
    setup_lib_io(context);
    setup_lib_string(context);
    setup_lib_seq(context);
    import_lib_array(context);
    setup_lib_map(context);
    setup_lib_lang(context);
    setup_lib_range(context);
    setup_lib_buffer(context);
    setup_lib_char(context);
    setup_lib_u8(context);
    setup_lib_float(context);
    setup_lib_int(context);
    setup_lib_maybe(context);
    setup_lib_error(context);

    // #todo I think we can remove this hack after the latest code updates.
    // #todo Move this to lang.rs
    // #todo #temp #hack See how it's done in lib-tan-chrono.
    let module = require_module("prelude", context);
    module.insert_invocable(
        "to-string",
        Expr::foreign_func(&expr_to_string), // #todo #temp
    );
}
