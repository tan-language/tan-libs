// #todo Consider moving to /string.

use fs::import_lib_fs;
use tan::context::Context;

pub mod fs;

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_fs(context);
}
