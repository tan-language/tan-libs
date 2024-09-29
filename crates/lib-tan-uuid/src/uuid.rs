use tan::{context::Context, error::Error, expr::Expr, util::module_util::require_module};
use uuid::Uuid;

// #todo Extract as a separate lib-tan-* crate.
// #todo UUID object with related methods.
// #todo UUID is a 128bit number, a buffer of 16 bytes.

pub fn uuid_new_v4(_args: &[Expr]) -> Result<Expr, Error> {
    let id = Uuid::new_v4();

    // #todo for the moment we use the UUID as a string alias, should be a buffer of 16 bytes?

    Ok(Expr::string(id))
}

pub fn import_lib_uuid(context: &mut Context) {
    let module = require_module("uuid", context);

    // #todo better name? construct-v4-uuid, or just v4-uuid.
    module.insert_invocable("make-v4-uuid", Expr::foreign_func(&uuid_new_v4));
}

// #todo Add unit-tests (Tan).
