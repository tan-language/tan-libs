use cmark::import_lib_text_cmark;
use tan::context::Context;

pub mod cmark;

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_text_cmark(context);
}
