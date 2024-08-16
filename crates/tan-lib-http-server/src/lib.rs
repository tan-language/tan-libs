use tan::context::Context;

use http_server::import_lib_http_server;

pub mod http_server;

// #todo network/smtp

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_http_server(context);
}
