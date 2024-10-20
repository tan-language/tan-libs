use std::collections::HashSet;

use tan::{
    context::Context,
    error::Error,
    expr::{expr_clone, Expr},
    util::module_util::require_module,
};

// #insight there isn alternative implementation in `set-alt.rs`.

pub fn set_new(_args: &[Expr]) -> Result<Expr, Error> {
    // #todo Expr has interior mutability, not the best for a Set key.
    let set: HashSet<Expr> = HashSet::new();
    Ok(Expr::set(set))
}

pub fn set_put(args: &[Expr]) -> Result<Expr, Error> {
    let [this, value] = args else {
        return Err(Error::invalid_arguments(
            "requires `this` and `value` arguments",
            None,
        ));
    };

    let Some(mut items) = this.as_set_mut() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Set",
            this.range(),
        ));
    };

    // #insight don't put annotated values in the Set.

    // #todo hmmm this clone!
    items.insert(expr_clone(value.unpack()));

    // #todo what should we return here?
    Ok(Expr::None)
}

// #todo set_values is a _weird_ name!
// #todo consider other names, e.g. items?
pub fn set_values(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    let Some(items) = this.as_set_mut() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Set",
            this.range(),
        ));
    };

    let values: Vec<_> = items.iter().map(expr_clone).collect();
    Ok(Expr::array(values))
}

pub fn import_lib_set(context: &mut Context) {
    // #tod op consider other paths, e.g. data/set, collections/set, collection/set, etc?
    let module = require_module("set", context);

    module.insert_invocable("Set", Expr::foreign_func(&set_new));

    // #todo #hack temp fix!
    // #todo really need to improve signature matching and e.g. support put$$Set$$Expr or put$$Set$$Any
    module.insert_invocable("put$$Set$$Int", Expr::foreign_func(&set_put));
    module.insert_invocable("put$$Set$$Float", Expr::foreign_func(&set_put));
    module.insert_invocable("put$$Set$$String", Expr::foreign_func(&set_put));
    // #todo investigate why this is needed!
    // #todo better solution: use Expr::Method or Expr::Multi for foreign functions and functions.
    module.insert_invocable("values-of", Expr::foreign_func(&set_values));
    module.insert_invocable("values-of$$Set", Expr::foreign_func(&set_values));
}
