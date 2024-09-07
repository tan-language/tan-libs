// #todo Consider moving to /string.

use tan::context::Context;
use text::import_lib_text;

pub mod text;

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_text(context);
}
