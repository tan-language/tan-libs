use json::import_lib_codec_json;
use tan::context::Context;

pub mod json;

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_codec_json(context);
}
