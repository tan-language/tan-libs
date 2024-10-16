pub mod cron;

use cron::import_lib_cron;
use tan::context::Context;

// #todo Consider implementing cron in Tan.

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_cron(context);
}
