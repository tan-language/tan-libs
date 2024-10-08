// #todo text/css
// #todo Rename to `css-x`, `cssx`? or `scss`? or `sexp-css`?
// #todo Conside `css-expr` name: https://docs.racket-lang.org/css-expr/
// #todo Consider naming this a 'dialect' or 'dsl' or 'language' instead of text?

use tan::{
    context::Context,
    error::Error,
    expr::{format_value, Expr},
    util::{module_util::require_module, try_lock_read},
};

// #todo #fixme key-symbol conversion skips the `:` chars.
// #todo maybe move to expr.rs as an alternative `as_stringable`?
fn try_string_from_expr(expr: &Expr) -> Option<String> {
    let expr = expr.unpack();

    match expr {
        Expr::Symbol(s) => Some(s.clone()),
        Expr::KeySymbol(s) => Some(format!(":{s}")),
        Expr::String(s) => Some(s.clone()),
        _ => None,
    }
}

// #todo is this evaluating something?
// #todo consider always adding the trailing `;`?

fn render_css_expr(expr: &Expr) -> Result<Expr, Error> {
    let expr = expr.unpack();

    // #todo try to unquote!

    match expr {
        Expr::List(terms) => {
            if let Some(op) = terms.first() {
                let Some(sym) = try_string_from_expr(op) else {
                    // #todo we could return the argument position here and enrich the error upstream.
                    // #todo hmm, the error is too precise here, do we really need the annotations?
                    return Err(Error::invalid_arguments(
                        &format!("{op} is not a Symbol"),
                        op.range(),
                    ));
                };

                let mut i = 1;

                // #todo escape body/children

                let mut body: Vec<String> = Vec::new();

                while i < terms.len() {
                    let prop = render_css_expr(&terms[i])?; // #todo no render needed
                    let mut declaration = format_value(&prop);
                    i += 1;
                    if i < terms.len() {
                        declaration.push_str(": ");
                        let value = render_css_expr(&terms[i])?; // #todo no render needed
                        declaration.push_str(&format_value(&value));
                    }
                    i += 1;
                    body.push(declaration);
                }

                if body.is_empty() {
                    // #todo add exception for <script> tag.
                    Ok(Expr::string(format!("{sym} {{}}")))
                } else {
                    Ok(Expr::string(format!("{sym} {{ {} }}", body.join("; "))))
                }
            } else {
                // #todo offer context, e.g. in which function we are.
                Err(Error::invalid_arguments(
                    "empty expression, remove",
                    expr.range(),
                )) // #todo
            }
        }
        // #todo write a unit test for this.
        Expr::Array(rules) => {
            let mut body: Vec<String> = Vec::new();
            // #todo #hack ultra hackish way to emulate unquote-explode in CSS-Expr
            let rules = try_lock_read(rules, None)?;
            let is_explode = if let Some(flag) = rules[0].as_string() {
                flag == "..."
            } else {
                false
            };
            if is_explode {
                for expr in rules.iter().skip(1) {
                    body.push(format_value(expr));
                }
                Ok(Expr::string(body.join(";")))
            } else {
                for expr in rules.iter() {
                    let value = render_css_expr(expr)?;
                    body.push(format_value(value));
                }
                // #todo consider \n\n as separator.
                Ok(Expr::string(body.join("\n")))
            }
        }
        Expr::Map(..) => {
            // #todo remove duplication with List above.
            // #todo what is the coorect type for this?
            // let items: &HashMap<String, Expr> = items.borrow();
            // #todo #hack temp solution.
            let items = expr.as_map().unwrap();
            let mut body: Vec<String> = Vec::new();
            for (key, value) in items.iter() {
                body.push(format!("{key}: {}", format_value(value)));
            }
            Ok(Expr::string(body.join("; ")))
        }
        _ => Ok(Expr::string(format_value(expr))),
    }
}

// #todo name `css_from_css_expr` ?
pub fn css_expr_to_css(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(expr) = args.first() {
        render_css_expr(expr)
    } else {
        Err(Error::invalid_arguments(
            "expected at least one argument",
            None,
        ))
    }
}

// #todo consider naming the library just `css`?
// #todo consider setup_mod_css or setup_module_css
pub fn import_lib_css_expr(context: &mut Context) {
    // #todo another name than dialect? (language, lang, flavor, dsl)
    // (use dialect/css-expr) (use dialect/css) (use dialect/html)
    let module = require_module("dialect/css-expr", context);

    // (let css (css-expr/to-css expr))
    module.insert_invocable("to-css", Expr::foreign_func(&css_expr_to_css));
}
