use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        args::{
            unpack_bool_arg, unpack_float_arg, unpack_int_arg, unpack_stringable_arg, unpack_u8_arg,
        },
        module_util::require_module,
    },
};

// #todo support all types!

// #todo add support for eq_array, eq_map

// #todo Not needed any more, remove it!
// #todo #temp hackish polymorphism helper!
// pub fn eq_polymorphic(args: &[Expr]) -> Result<Expr, Error> {
//     let Some(expr) = args.first() else {
//         return Err(Error::invalid_arguments("malformed equality test", None));
//     };
//     match expr.unpack() {
//         Expr::Int(..) => eq_int(args),
//         Expr::Bool(..) => eq_bool(args),
//         Expr::Float(..) => eq_float(args),
//         Expr::String(..) => eq_string(args),
//         Expr::Symbol(..) | Expr::KeySymbol(..) | Expr::Type(..) => eq_symbol(args),
//         Expr::Array(..) => array_eq(args),
//         _ => Err(Error::invalid_arguments("malformed equality test", None)),
//     }
// }

pub fn eq_int(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.

    // #todo also pass the function name, or at least show the function name upstream.
    let a = unpack_int_arg(args, 0, "a")?;
    let b = unpack_int_arg(args, 1, "b")?;

    Ok(Expr::Bool(a == b))
}

pub fn eq_u8(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.

    // #todo also pass the function name, or at least show the function name upstream.
    let a = unpack_u8_arg(args, 0, "a")?;
    let b = unpack_u8_arg(args, 1, "b")?;

    Ok(Expr::Bool(a == b))
}

pub fn eq_float(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.

    let a = unpack_float_arg(args, 0, "a")?;
    let b = unpack_float_arg(args, 1, "b")?;

    Ok(Expr::Bool(a == b))
}

pub fn eq_bool(args: &[Expr]) -> Result<Expr, Error> {
    // #todo check comments in other eq_* functions.

    // #todo also pass the function name, or at least show the function name upstream.
    let a = unpack_bool_arg(args, 0, "a")?;
    let b = unpack_bool_arg(args, 1, "b")?;

    Ok(Expr::Bool(a == b))
}

pub fn eq_string(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.

    let a = unpack_stringable_arg(args, 0, "a")?;
    let b = unpack_stringable_arg(args, 1, "b")?;

    Ok(Expr::Bool(a == b))
}

// #insight handles both (quoted) Symbol and KeySymbol, they are the same thing anyway. Also handles Type.
pub fn eq_symbol(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "`=` requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_symbolic() else {
        return Err(Error::invalid_arguments(
            &format!("`{a}` is not a Symbol"),
            a.range(),
        ));
    };

    let Some(b) = b.as_symbolic() else {
        return Err(Error::invalid_arguments(
            &format!("`{b}` is not a Symbol"),
            b.range(),
        ));
    };

    Ok(Expr::Bool(a == b))
}

// #todo implement not_eq_* with Tan? can be automatically generic!

pub fn not_eq_int(args: &[Expr]) -> Result<Expr, Error> {
    Ok(Expr::Bool(eq_int(args)?.is_false()))
}

pub fn not_eq_float(args: &[Expr]) -> Result<Expr, Error> {
    Ok(Expr::Bool(eq_float(args)?.is_false()))
}

pub fn not_eq_string(args: &[Expr]) -> Result<Expr, Error> {
    Ok(Expr::Bool(eq_string(args)?.is_false()))
}

// #insight handles both (quoted) Symbol and KeySymbol, they are the same thing anyway.
pub fn not_eq_symbol(args: &[Expr]) -> Result<Expr, Error> {
    // Use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
    // #todo support overloading,
    // #todo make equality a method of Expr?
    // #todo support non-Int types
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "`!=` requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_symbolic() else {
        return Err(Error::invalid_arguments(
            &format!("`{a}` is not a String"),
            a.range(),
        ));
    };

    let Some(b) = b.as_symbolic() else {
        return Err(Error::invalid_arguments(
            &format!("`{b}` is not a Symbol"),
            b.range(),
        ));
    };

    Ok(Expr::Bool(a != b))
}

pub fn int_gt(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "`>` requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("`{a}` is not an Int"),
            a.range(),
        ));
    };

    let Some(b) = b.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("`{b}` is not an Int"),
            b.range(),
        ));
    };

    Ok(Expr::Bool(a > b))
}

pub fn float_gt(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "`>` requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_float() else {
        return Err(Error::invalid_arguments(
            &format!("`{a}` is not a Float"),
            a.range(),
        ));
    };

    let Some(b) = b.as_float() else {
        return Err(Error::invalid_arguments(
            &format!("`{b}` is not a Float"),
            b.range(),
        ));
    };

    Ok(Expr::Bool(a > b))
}

pub fn int_gte(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Support multiple arguments.

    let a = unpack_int_arg(args, 0, "a")?;
    let b = unpack_int_arg(args, 1, "b")?;

    Ok(Expr::Bool(a >= b))
}

pub fn float_gte(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Support multiple arguments.

    let a = unpack_float_arg(args, 0, "a")?;
    let b = unpack_float_arg(args, 1, "b")?;

    Ok(Expr::Bool(a >= b))
}

pub fn int_lt(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "`<` requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("`{a}` is not an Int"),
            a.range(),
        ));
    };

    let Some(b) = b.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("`{b}` is not an Int"),
            b.range(),
        ));
    };

    Ok(Expr::Bool(a < b))
}

pub fn float_lt(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.

    let a = unpack_float_arg(args, 0, "a")?;
    let b = unpack_float_arg(args, 1, "b")?;

    Ok(Expr::Bool(a < b))
}

pub fn int_lte(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Support multiple arguments.

    let a = unpack_int_arg(args, 0, "a")?;
    let b = unpack_int_arg(args, 1, "b")?;

    Ok(Expr::Bool(a <= b))
}

pub fn float_lte(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Support multiple arguments.

    let a = unpack_float_arg(args, 0, "a")?;
    let b = unpack_float_arg(args, 1, "b")?;

    Ok(Expr::Bool(a <= b))
}

// #todo should we have an explicit module for these functions?

pub fn setup_lib_eq(context: &mut Context) {
    let module = require_module("prelude", context);

    // module.insert_invocable("=", Expr::foreign_func(&eq_int));
    module.insert_invocable("=$$Int$$Int", Expr::foreign_func(&eq_int));
    module.insert_invocable("=$$U8$$U8", Expr::foreign_func(&eq_u8));
    module.insert_invocable("=$$Bool$$Bool", Expr::foreign_func(&eq_bool));
    module.insert_invocable("=$$Float$$Float", Expr::foreign_func(&eq_float));
    module.insert_invocable("=$$String$$String", Expr::foreign_func(&eq_string));
    // module.insert_invocable("=$$Symbol$$Symbol", Expr::foreign_func(&eq_symbol)));
    module.insert_invocable("=$$KeySymbol$$KeySymbol", Expr::foreign_func(&eq_symbol));
    // #todo #hack this is nasty!
    module.insert_invocable("=$$Type$$Type", Expr::foreign_func(&eq_symbol));
    module.insert_invocable("=$$Type$$String", Expr::foreign_func(&eq_symbol));
    module.insert_invocable("=$$Type$$KeySymbol", Expr::foreign_func(&eq_symbol));

    module.insert_invocable("!=$$Int$$Int", Expr::foreign_func(&not_eq_int));
    module.insert_invocable("!=$$Float$$Float", Expr::foreign_func(&not_eq_float));
    module.insert_invocable("!=$$String$$String", Expr::foreign_func(&not_eq_string));
    module.insert_invocable("!=$$Symbol$$Symbol", Expr::foreign_func(&not_eq_symbol));
    module.insert_invocable(
        "!=$$KeySymbol$$KeySymbol",
        Expr::foreign_func(&not_eq_symbol),
    );

    module.insert_invocable(">$$Int$$Int", Expr::foreign_func(&int_gt));
    module.insert_invocable(">$$Float$$Float", Expr::foreign_func(&float_gt));
    module.insert_invocable(">=$$Int$$Int", Expr::foreign_func(&int_gte));
    module.insert_invocable(">=$$Float$$Float", Expr::foreign_func(&float_gte));
    module.insert_invocable("<$$Int$$Int", Expr::foreign_func(&int_lt));
    module.insert_invocable("<$$Float$$Float", Expr::foreign_func(&float_lt));
    module.insert_invocable("<=$$Int$$Int", Expr::foreign_func(&int_lte));
    module.insert_invocable("<=$$Float$$Float", Expr::foreign_func(&float_lte));
}
