use http_client::import_lib_http_client;
use tan::context::Context;

pub mod http_client;

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_http_client(context);
}
