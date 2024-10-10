use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        fs::{get_dirname, get_full_extension},
        module_util::require_module,
    },
};

// #todo consider to associate most functions to the `Path` type.
// #todo support (path :extension)
// #todo support (path :full-extension)
// #todo support (path :filename)
// #todo support (path :directory)
// #todo implement (get-parent ..)

// #todo should it include the final `/`?
/// Returns the directory part of a path.
pub fn path_get_dirname(args: &[Expr]) -> Result<Expr, Error> {
    let [path] = args else {
        return Err(Error::invalid_arguments("requires a `path` argument", None));
    };

    // #todo in the future check for `Path`.
    // #todo return Maybe::None if no directory found.

    let Some(path) = path.as_string() else {
        return Err(Error::invalid_arguments(
            "`path` argument should be a String",
            path.range(),
        ));
    };

    // #todo should return a Maybe.
    let dirname = get_dirname(path).unwrap_or("");

    // #todo should return a `Path` value.

    Ok(Expr::string(dirname))
}

/// Returns the 'full' extension of a path.
pub fn path_get_extension(args: &[Expr]) -> Result<Expr, Error> {
    let [path] = args else {
        return Err(Error::invalid_arguments("requires a `path` argument", None));
    };

    // #todo in the future check for `Path`.
    // #todo return Maybe::None if no extension found.

    let Some(path) = path.as_string() else {
        return Err(Error::invalid_arguments(
            "`path` argument should be a String",
            path.range(),
        ));
    };

    // #todo should return a Maybe.
    let extension = get_full_extension(path).unwrap_or("".to_string());

    // #todo should return a `Path` value.

    Ok(Expr::string(extension))
}

pub fn setup_lib_path(context: &mut Context) {
    // #todo Move under fs/?
    // #insight not everything is fs-related.
    let module = require_module("path", context);

    // #todo think of a better name.
    module.insert_invocable("get-dirname", Expr::foreign_func(&path_get_dirname));

    // #todo think of a better name.
    module.insert_invocable("get-extension", Expr::foreign_func(&path_get_extension));
}
