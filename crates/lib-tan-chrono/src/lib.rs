use chrono::import_lib_chrono;
use tan::context::Context;

mod chrono;

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_chrono(context);
}
