use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_stringable_arg, module_util::require_module},
};

pub fn text_capitalize(args: &[Expr]) -> Result<Expr, Error> {
    let text = unpack_stringable_arg(args, 0, "text")?;

    // #ai
    // #todo Optimize this LM synthesized code:
    let mut chars = text.chars();
    let capitalized = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    };

    Ok(Expr::string(capitalized))
}

pub fn import_lib_text(context: &mut Context) {
    let module = require_module("text", context);

    module.insert("capitalize", Expr::foreign_func(&text_capitalize));
}

// #insight Tan tests are available at `@std/text/`.
