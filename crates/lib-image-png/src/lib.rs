use tan::context::Context;

use png::import_lib_image_png;

pub mod png;

// #todo consider other names, e.g. `raster`, `graphics`, ...

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_image_png(context);
}
