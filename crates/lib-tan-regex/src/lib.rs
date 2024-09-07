use regex::import_lib_regex;
use tan::context::Context;

pub mod regex;

// #todo find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_regex(context);
}
