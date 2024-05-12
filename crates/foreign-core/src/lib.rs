use tan::{context::Context, library::uuid::setup_lib_uuid};

pub mod uuid;

pub fn setup_lib(context: &mut Context) {
    setup_lib_uuid(context);
}
