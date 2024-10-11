use tan::{
    context::Context,
    error::Error,
    expr::{expr_clone, format_value, Expr},
    util::{
        args::{unpack_map_arg, unpack_map_mut_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

// #todo implement some of those functions: https://www.programiz.com/python-programming/methods/mapionary

pub fn map_eq(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo support multiple arguments.

    let a = unpack_map_arg(args, 0, "a")?;
    let b = unpack_map_arg(args, 1, "b")?;

    Ok(Expr::Bool(*a == *b))
}

// #insight use `contains-key` so that `contains` refers to the value, consistent with other collections.
// #todo consider other names: has, has-key, contains-key, includes, etc.
// #todo consider appending a `?`
pub fn map_contains_key(args: &[Expr]) -> Result<Expr, Error> {
    let [map, key] = args else {
        return Err(Error::invalid_arguments(
            "requires `this` and `key` argument",
            None,
        ));
    };

    let Some(items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    // #todo support non-string/symbol keys
    // #todo support string keys also.

    // #todo temp solution!
    let key = format_value(key);

    // #idea instead convert key to string? or hash?

    Ok(Expr::Bool(items.contains_key(&key)))
}

// #todo version that returns a new sequence
// #todo also consider set, put
// #todo item or element? -> I think for collections item is better.
pub fn map_put(args: &[Expr]) -> Result<Expr, Error> {
    let [map, key, value] = args else {
        return Err(Error::invalid_arguments(
            "requires `this`, `key`, and `value` arguments",
            None,
        ));
    };

    let Some(mut items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    // #todo support non-string/symbol keys
    // #todo support string keys also.

    // let Expr::KeySymbol(key) = key.unpack() else {
    //     return Err(Error::invalid_arguments(
    //         "`key` argument should be a KeySymbol",
    //         key.range(),
    //     ));
    // };

    // #todo temp solution!
    let key = format_value(key);

    // #idea instead convert key to string? or hash?

    items.insert(key.clone(), value.unpack().clone()); // #todo hmm this clone!

    // #todo what to return?
    Ok(Expr::None)
}

// #todo how is this related with HTTP PATCH?
// #todo alternative names: `merge`, `patch`, `extend` (from Rust)
// #todo I think `extend` is better, more descriptive.
// #todo have draining and non-draining versions (drain other.) (consuming is better than draining)
// #todo have mutating and non-mutating versions.
pub fn map_update_mut(args: &[Expr]) -> Result<Expr, Error> {
    let [this, other] = args else {
        return Err(Error::invalid_arguments(
            "requires `this` and `other` argument",
            None,
        ));
    };

    let Some(mut this_items) = this.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Map",
            this.range(),
        ));
    };

    let Some(other_items) = other.as_map() else {
        return Err(Error::invalid_arguments(
            "`other` argument should be a Map",
            other.range(),
        ));
    };

    // #todo expensive clone
    // let it = other_items.clone().into_iter();
    // this_items.extend(it);

    // #todo still expensive
    for (key, value) in other_items.iter() {
        this_items.insert(key.clone(), value.clone());
    }

    // #todo what to return?
    // Ok(this.clone()) // #todo this is expensive, just use Rc/Arc everywhere.
    Ok(Expr::None)
}

// #todo could be replaced with `some-or` or Maybe functions.
// #todo temp method until we have Maybe
// #todo (map :key <default>) could accept a default value.
// #todo this should be a special form, not evaluate the default value if not needed (short-circuit).
// #todo consider making default optional.
pub fn map_get_or(args: &[Expr]) -> Result<Expr, Error> {
    // #todo rename `default_value` to `fallback`, more descriptive.
    let [map, key, default_value] = args else {
        return Err(Error::invalid_arguments(
            "requires `this` and `key` argument",
            None,
        ));
    };

    let Some(items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    // #todo support non-string/symbol keys
    // #todo support string keys also.

    // let Expr::KeySymbol(key) = key.unpack() else {
    //     return Err(Error::invalid_arguments(
    //         "`key` argument should be a KeySymbol",
    //         key.range(),
    //     ));
    // };

    // #todo temp solution!
    let key = format_value(key);

    // #idea instead convert key to string? or hash?

    let value = items.get(&key);

    // #todo can we remove the clones?

    if let Some(value) = value {
        Ok(expr_clone(value))
    } else {
        Ok(expr_clone(default_value))
    }
}

// #todo Also consider the name `delete` (or even `yank`)?
pub fn map_remove(args: &[Expr]) -> Result<Expr, Error> {
    let mut map = unpack_map_mut_arg(args, 0, "map")?;
    let key = unpack_stringable_arg(args, 1, "key")?;

    // #todo Should return None if nothing removed!
    // #todo Should this return the value? -> Yes make maximally useful!
    let value = map.remove(key);

    // #insight Returning the value is cheap.
    Ok(value.unwrap_or(Expr::None))
}

// #todo consider name `keys-of` to avoid clash with variable keys? -> get-keys
// #todo document the above in a decision file
// #todo keys is problematic if it's in the prelude!
pub fn map_get_keys(args: &[Expr]) -> Result<Expr, Error> {
    let [map] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    let Some(items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    let keys: Vec<_> = items.keys().map(Expr::string).collect();

    Ok(Expr::array(keys))
}

pub fn map_get_values(args: &[Expr]) -> Result<Expr, Error> {
    let [map] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    let Some(items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    let keys: Vec<_> = items.values().map(expr_clone).collect();

    Ok(Expr::array(keys))
}

// #todo consider other names, e.g. `items`.
// #todo introduce entries/get-entries for other collections/containers, even Array/List.
pub fn map_get_entries(args: &[Expr]) -> Result<Expr, Error> {
    let [map] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    let Some(items) = map.as_map_mut() else {
        return Err(Error::invalid_arguments(
            "`map` argument should be a Map",
            map.range(),
        ));
    };

    // #todo why does map return k as String?
    // #todo wow, this is incredibly inefficient.
    // #todo #hack temp fix we add the a `:` prefix to generate keys
    let entries: Vec<_> = items
        .iter()
        .map(|(k, v)| Expr::array(vec![Expr::KeySymbol(k.clone()), expr_clone(v)]))
        .collect();

    Ok(Expr::array(entries))
}

pub fn setup_lib_map(context: &mut Context) {
    let module = require_module("prelude", context);

    // #todo add something like `get-or-init`` or `update-with-default` or `get-and-update`

    module.insert_invocable("=$$Map$$Map", Expr::foreign_func(&map_eq));

    // #todo add type qualifiers!
    module.insert_invocable("contains-key?", Expr::foreign_func(&map_contains_key));
    // #todo #deprecate Remove contains-key when all call-sites are updated.
    module.insert_invocable("contains-key", Expr::foreign_func(&map_contains_key));
    module.insert_invocable("put", Expr::foreign_func(&map_put));
    module.insert_invocable("put$$Map", Expr::foreign_func(&map_put));
    module.insert_invocable("update!", Expr::foreign_func(&map_update_mut));
    module.insert_invocable("get-or", Expr::foreign_func(&map_get_or));

    // #(Func [(Map T) Hashable] T)
    module.insert_invocable("remove", Expr::foreign_func(&map_remove));

    // #todo Remove older get-* functions {
    module.insert_invocable("get-keys", Expr::foreign_func(&map_get_keys));
    module.insert_invocable("get-values", Expr::foreign_func(&map_get_values));
    module.insert_invocable("get-entries", Expr::foreign_func(&map_get_entries));
    // }
    module.insert_invocable("keys-of", Expr::foreign_func(&map_get_keys));
    module.insert_invocable("values-of", Expr::foreign_func(&map_get_values));
    module.insert_invocable("entries-of", Expr::foreign_func(&map_get_entries));
}
