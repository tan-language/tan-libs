use comrak::{markdown_to_html, Options};

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_stringable_arg, module_util::require_module},
};

// #todo rename to `text/common-mark`?
// #todo find a better name for this module.
// #todo this should be extracted to a separate crate, and/or a dynamic library.

// #insight some Github Flavored Markdown extensions are supported.

pub fn html_from_common_mark(args: &[Expr]) -> Result<Expr, Error> {
    let markup = unpack_stringable_arg(args, 0, "markup")?;

    // #todo consider renaming :unsafe to :allow-html?
    // Accepts an optional ..options parameter
    // :unsafe
    let options = args.get(1);

    // #see here are the Comrak options: https://docs.rs/comrak/latest/comrak/struct.ExtensionOptions.html
    // #todo consider allowing granular setting of options? probably no.
    // #todo cache the generation of options.

    let mut comrak_options = Options::default();
    comrak_options.extension.strikethrough = true;
    comrak_options.extension.table = true;
    comrak_options.extension.autolink = true;
    comrak_options.extension.tasklist = true;
    comrak_options.extension.superscript = true;
    comrak_options.extension.footnotes = true;
    // #todo extract shortcodes support to separate module, e.g. `text/shortcodes`
    // options.extension.shortcodes = true; // #insight needs feature `shortcodes`
    if options.is_some() {
        let Some(options) = options.unwrap().as_map() else {
            return Err(Error::invalid_arguments(
                "options argument should be a map",
                None,
            ));
        };

        if options.contains_key("unsafe") {
            // #todo actually check that :unsafe is true
            // #todo add unit test
            // #insight this allows 'raw' html
            comrak_options.render.unsafe_ = true;
        }
    }

    let html = markdown_to_html(markup, &comrak_options);

    Ok(Expr::String(html))
}

pub fn import_lib_text_cmark(context: &mut Context) {
    // #todo find a good name/path for this library.
    let module = require_module("text/cmark", context);

    // (let html cmark/to-html expr)
    // (let html cmark/expr-to-html expr)
    // (let html cmark/expr->html expr)
    // (let html cmark/to-html markup)
    // (let html cmark/to-html markup {:unsafe true})
    module.insert_invocable("to-html", Expr::foreign_func(&html_from_common_mark));
}
