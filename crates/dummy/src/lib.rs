use std::sync::Arc;

use rand::Rng;

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_int_arg, module_util::require_module},
};

fn random_int(args: &[Expr], _context: &mut Context) -> Result<Expr, Error> {
    let end = unpack_int_arg(args, 0, "end")?;

    let mut rng = rand::thread_rng();

    Ok(Expr::Int(rng.gen_range(0..end)))
}

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    let module = require_module("dummy", context);
    module.insert("random2", Expr::ForeignFunc(Arc::new(random_int)));
}
