use css_expr::import_lib_css_expr;
use tan::context::Context;

// #todo Rename to css-x or css-t or t-css
pub mod css_expr;

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_css_expr(context);
}
