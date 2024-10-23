use core::f64;

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{
        args::{unpack_bool_arg, unpack_float_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

// #todo Don't include functions like floor/ceil/round in prelude.
// #todo Consider the names floor-of, ceil-of, round-of.

// #todo Implement with Tan.
pub fn float_from_int(args: &[Expr]) -> Result<Expr, Error> {
    let [value] = args else {
        return Err(Error::invalid_arguments("requires `value` argument", None));
    };

    // #todo create a helper.
    let Some(value) = value.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("value=`{value}` is not Int"),
            value.range(),
        ));
    };

    Ok(Expr::Float(value as f64))
}

// #todo Implement with Tan.
pub fn float_from_bool(args: &[Expr]) -> Result<Expr, Error> {
    let value = unpack_bool_arg(args, 0, "value")?;

    Ok(Expr::Float(if value { 1.0 } else { 0.0 }))
}

// #todo Consider (Float/from-string ...)
pub fn float_from_string(args: &[Expr]) -> Result<Expr, Error> {
    let string = unpack_stringable_arg(args, 0, "string")?;
    let Ok(value) = string.parse::<f64>() else {
        return Err(Error::invalid_arguments(
            &format!("string=`{string}` is not a valid Float number"),
            args[0].range(),
        ));
    };
    Ok(Expr::Float(value))
}

// #todo Introduce Float/+Infinity, Float/-Infinity.

// #todo Consider skipping the prelude for min?
// #todo What could be another name instead of min? `min-of`? `minimum`?
pub fn float_min(args: &[Expr]) -> Result<Expr, Error> {
    let mut min = f64::MAX;

    for arg in args {
        let Some(n) = arg.as_float() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Float"),
                arg.range(),
            ));
        };
        if n < min {
            min = n;
        }
    }

    Ok(Expr::Float(min))
}

pub fn float_max(args: &[Expr]) -> Result<Expr, Error> {
    let mut max = f64::MIN;

    for arg in args {
        let Some(n) = arg.as_float() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Float"),
                arg.range(),
            ));
        };
        if n > max {
            max = n;
        }
    }

    Ok(Expr::Float(max))
}

// #todo Implement in Tan.
pub fn float_abs(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(n.abs()))
}

// #todo Introduce multiple rounding functions.
// #todo Should the rounding functions also handle floor/ceil?

pub fn float_floor(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(n.floor()))
}

pub fn float_ceil(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(n.ceil()))
}

// #todo Support multiple rounding modes.
// #todo What would be a non-ambiguous number?
pub fn float_round(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    // #todo Consider round_ties_even.
    Ok(Expr::Float(n.round()))
}

pub fn float_sqrt(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(n.sqrt()))
}

pub fn float_sin(args: &[Expr]) -> Result<Expr, Error> {
    let Some(n) = args.first() else {
        return Err(Error::invalid_arguments("missing argument", None));
    };

    let Some(n) = n.as_float() else {
        return Err(Error::invalid_arguments(
            "expected Float argument",
            n.range(),
        ));
    };

    Ok(Expr::Float(n.sin()))
}

pub fn float_cos(args: &[Expr]) -> Result<Expr, Error> {
    let Some(n) = args.first() else {
        return Err(Error::invalid_arguments("missing argument", None));
    };

    let Some(n) = n.as_float() else {
        return Err(Error::invalid_arguments(
            "expected Float argument",
            n.range(),
        ));
    };

    Ok(Expr::Float(n.cos()))
}

// #todo Avoid confusion with ...Tan.
pub fn float_tan(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(n.tan()))
}

// #todo support variable args?
pub fn float_powi(args: &[Expr]) -> Result<Expr, Error> {
    let [n, e] = args else {
        return Err(Error::invalid_arguments(
            "- requires at least two arguments",
            None,
        ));
    };

    // #todo version of as_float that automatically throws an Error?
    let Some(n) = n.as_float() else {
        return Err(Error::invalid_arguments(
            &format!("{n} is not a Float"),
            n.range(),
        ));
    };

    let Some(e) = e.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("{e} is not an Int"),
            e.range(),
        ));
    };

    Ok(Expr::Float(n.powi(e as i32)))
}

// #todo Introduce clamp
// #todo Also introduce clamp in Range and/or Interval.

pub fn setup_lib_float(context: &mut Context) {
    // #todo put in 'float' path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    // #todo consider to-float instead?

    // #todo #think having so many overloads can cover issues, e.g. use the wrong implicit overload.

    // #todo make `float_new` the default.
    module.insert_invocable("Float", Expr::foreign_func(&float_from_int));
    module.insert_invocable("Float$$Int", Expr::foreign_func(&float_from_int));
    module.insert_invocable("Float$$Bool", Expr::foreign_func(&float_from_bool));
    module.insert_invocable("Float$$String", Expr::foreign_func(&float_from_string));
    module.insert_invocable("min", Expr::foreign_func(&float_min));
    module.insert_invocable(
        "min$$Float$$Float",
        // annotate_type(Expr::foreign_func(&add_float)), "Float"),
        Expr::foreign_func(&float_min),
    );
    module.insert_invocable("max", Expr::foreign_func(&float_max));
    module.insert_invocable(
        "max$$Float$$Float",
        // annotate_type(Expr::foreign_func(&add_float)), "Float"),
        Expr::foreign_func(&float_max),
    );

    module.insert_invocable("abs", Expr::foreign_func(&float_abs));
    module.insert_invocable("abs$$Float", Expr::foreign_func(&float_abs));

    // #todo Kind of annoying that these are non-verbs.

    module.insert_invocable("floor$$Float", Expr::foreign_func(&float_floor));
    module.insert_invocable("ceil$$Float", Expr::foreign_func(&float_ceil));
    module.insert_invocable("round$$Float", Expr::foreign_func(&float_round));

    // #todo Consider sqrt-of or Num/sqrt or math/sqrt.
    // #todo Note that `sqrt` does not follow Tan naming conventions but it's a standard term.
    module.insert_invocable("sqrt", Expr::foreign_func(&float_sqrt));
    module.insert_invocable("sqrt$$Float", Expr::foreign_func(&float_sqrt));

    module.insert_invocable("sin", Expr::foreign_func(&float_sin));
    module.insert_invocable("cos", Expr::foreign_func(&float_cos));
    module.insert_invocable("tan", Expr::foreign_func(&float_tan));
    module.insert_invocable("**", Expr::foreign_func(&float_powi));
    module.insert_invocable("**$$Float$$Int", Expr::foreign_func(&float_powi));

    // Constants.

    // #warning Don't use those yet!
    // #todo Fix Float/max, it self-evaluates, duh!
    // #todo Mark as constant / make immutable?
    // #todo Should we skip `Float/` prefix?
    // #todo Rename to max-value?
    module.insert_invocable("float/max", Expr::Float(f64::MAX));
    module.insert_invocable("float/infinity", Expr::Float(f64::INFINITY));
}
