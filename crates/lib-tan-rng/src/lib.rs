use rng::import_lib_rng;
use tan::context::Context;

pub mod rng;

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_rng(context);
}
