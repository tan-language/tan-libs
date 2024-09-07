// #todo Consider moving to /string.

use tan::context::Context;
use uuid::import_lib_uuid;

pub mod uuid;

#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_uuid(context);
}
