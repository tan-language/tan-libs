use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_stringable_arg, module_util::require_module},
};
use urlencoding::encode;

// #todo Implement encode-uri-component, decode-uri-component

// #todo also add support for from_urlencoded crate.

// #todo Consider using quote or escape instead of encode.
// #todo C#: encode-data-string, encode-uri-string, Go: queryEscape, pathEscape.
// #todo uri/escape-string, uri/escape-component-string
pub fn encode_uri_component(args: &[Expr]) -> Result<Expr, Error> {
    let string = unpack_stringable_arg(args, 0, "string")?;
    let encoded = encode(string);
    Ok(Expr::string(encoded))
}

// #todo find a good name for this.
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    // #todo consider other paths?
    let module = require_module("codec/uri", context);

    module.insert(
        "encode-uri-component",
        Expr::foreign_func(&encode_uri_component),
    );
}
