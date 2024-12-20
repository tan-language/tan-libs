use rust_decimal_macros::dec;

use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, Expr},
    util::{
        args::{unpack_float_arg, unpack_int_arg},
        module_util::require_module,
    },
};

// #todo Split into multiple files, int, float, time, date, etc...

// #todo rename rust implementations to {type}_{func}.

// #insight
// Named `arithmetic` as those operators can apply to non-numbers, e.g. Time, Date

// #todo use AsRef, to avoid Annotated!
// #todo use macros to generate specializations for generic versions.
// #todo deduct from type if the function can affect the env or have any other side-effects.

// #todo ranges for arguments is too detailed, most probably we do not have the ranges!
// #todo support invalid_arguments without range.

// #fixme (+ 1.2 1.4 1.5) does not work, falls back to Int (more than 2 arguments)

// #todo autogen with a macro!
pub fn add_int(args: &[Expr]) -> Result<Expr, Error> {
    let mut xs = Vec::new();

    for arg in args {
        let Some(n) = arg.as_int() else {
            // #todo we could return the argument position here and enrich the error upstream.
            // #todo hmm, the error is too precise here, do we really need the annotations?
            return Err(Error::invalid_arguments(
                &format!("{arg} is not an Int"),
                arg.range(),
            ));
        };
        xs.push(n);
    }

    let sum = add_int_impl(xs);

    Ok(Expr::Int(sum))
}

// #insight example of splitting wrapper from impl.
fn add_int_impl(xs: Vec<i64>) -> i64 {
    xs.iter().sum()
}

pub fn add_float(args: &[Expr]) -> Result<Expr, Error> {
    let mut sum = 0.0;

    for arg in args {
        let Some(n) = arg.as_float() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Float"),
                arg.range(),
            ));
        };
        sum += n;
    }

    Ok(Expr::Float(sum))
}

// #todo Move to dec.rs
pub fn add_dec(args: &[Expr]) -> Result<Expr, Error> {
    let mut sum = dec!(0.0);

    for arg in args {
        let Some(n) = arg.as_decimal() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Dec"),
                arg.range(),
            ));
        };
        sum += n;
    }

    Ok(Expr::Dec(sum))
}

// #todo keep separate, optimized version with just 2 arguments!
// #todo should support variable args.
// #todo should return the error without range and range should be added by caller.
// pub fn sub_int(args: &[Expr]) -> Result<Expr, Error> {
//     // #todo support multiple arguments.
//     let [a, b] = args else {
//         return Err(Error::invalid_arguments(
//             "- requires at least two arguments",
//             None,
//         ));
//     };

//     let Some(a) = a.as_int() else {
//         return Err(Error::invalid_arguments(
//             &format!("{a} is not an Int"),
//             a.range(),
//         ));
//     };
//
//     let Some(b) = b.as_int() else {
//         return Err(Error::invalid_arguments(
//             &format!("{b} is not an Int"),
//             b.range(),
//         ));
//     };
//
//     Ok(Expr::Int(a - b))
// }

pub fn sub_int(args: &[Expr]) -> Result<Expr, Error> {
    // #todo what if there is a single argument?

    let mut diff = unpack_int_arg(args, 0, "n")?;

    for arg in args.iter().skip(1) {
        let Some(n) = arg.as_int() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not an Int"),
                arg.range(),
            ));
        };
        diff -= n;
    }

    Ok(Expr::Int(diff))
}

pub fn neg_int(args: &[Expr]) -> Result<Expr, Error> {
    // #todo What about 0?
    let n = unpack_int_arg(args, 0, "n")?;
    Ok(Expr::Int(-n))
}

pub fn sub_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "- requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_float() else {
        return Err(Error::invalid_arguments(
            &format!("{a} is not a Float"),
            a.range(),
        ));
    };

    let Some(b) = b.as_float() else {
        return Err(Error::invalid_arguments(
            &format!("{b} is not a Float"),
            b.range(),
        ));
    };

    Ok(Expr::Float(a - b))
}

pub fn neg_float(args: &[Expr]) -> Result<Expr, Error> {
    let n = unpack_float_arg(args, 0, "n")?;
    Ok(Expr::Float(-n))
}

pub fn mul_int(args: &[Expr]) -> Result<Expr, Error> {
    // #todo optimize!
    let mut product = 1;

    for arg in args {
        let Some(n) = arg.as_int() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not an Int"),
                arg.range(),
            ));
        };
        product *= n;
    }

    Ok(Expr::Int(product))
}

pub fn mul_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo optimize!
    let mut product = 1.0;

    for arg in args {
        let Some(n) = arg.as_float() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Float"),
                arg.range(),
            ));
        };
        product *= n;
    }

    Ok(Expr::Float(product))
}

pub fn div_int(args: &[Expr]) -> Result<Expr, Error> {
    // #todo optimize!
    let mut quotient = unpack_int_arg(args, 0, "n")?;

    for arg in args.iter().skip(1) {
        let Some(n) = arg.as_int() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not an Int"),
                arg.range(),
            ));
        };

        quotient /= n;
    }

    Ok(Expr::Int(quotient))
}

// #todo support int/float.
pub fn div_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo optimize!
    let mut quotient = f64::NAN;

    // #todo check for divide by zero! even statically check!
    // #todo actually, divide by zero should return Infinity, not panic!!

    for arg in args {
        let Some(n) = arg.as_float() else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Float"),
                arg.range(),
            ));
        };

        if quotient.is_nan() {
            quotient = n;
        } else {
            quotient /= n;
        }
    }

    Ok(Expr::Float(quotient))
}

pub fn mod_int(args: &[Expr]) -> Result<Expr, Error> {
    // #todo what are good variable names.
    let x = unpack_int_arg(args, 0, "x")?;
    let y = unpack_int_arg(args, 1, "y")?;

    Ok(Expr::Int(x % y))
}

pub fn mod_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo what are good variable names.
    let x = unpack_float_arg(args, 0, "x")?;
    let y = unpack_float_arg(args, 1, "y")?;

    Ok(Expr::Float(x % y))
}

// #todo should be associated with `Ordering` and `Comparable`.
pub fn int_compare(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "requires at least two arguments",
            None,
        ));
    };

    let Some(a) = a.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("{a} is not an Int"),
            a.range(),
        ));
    };

    let Some(b) = b.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("{b} is not an Int"),
            b.range(),
        ));
    };

    // #todo temp hack until Tan has enums?
    let ordering = match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    Ok(Expr::Int(ordering))
}

pub fn setup_lib_arithmetic(context: &mut Context) {
    let module = require_module("prelude", context);

    // #todo notice the use of annotate_type.

    // #todo forget the mangling, implement with a dispatcher function, multi-function.
    module.insert_invocable("+", annotate_type(Expr::foreign_func(&add_int), "Int"));
    module.insert_invocable(
        "+$$Int$$Int",
        annotate_type(Expr::foreign_func(&add_int), "Int"),
    );
    // #todo #temp hack to support multiple args.
    module.insert_invocable(
        "+$$Int$$Int$$Int",
        annotate_type(Expr::foreign_func(&add_int), "Int"),
    );
    module.insert_invocable(
        "+$$Int$$Int$$Int$$Int",
        annotate_type(Expr::foreign_func(&add_int), "Int"),
    );
    module.insert_invocable(
        "+$$Float$$Float",
        // #todo add the proper type: (Func Float Float Float)
        // #todo even better: (Func (Many Float) Float)
        annotate_type(Expr::foreign_func(&add_float), "Float"),
    );
    // #todo #temp hack to support multiple args.
    module.insert_invocable(
        "+$$Float$$Float$$Float",
        // #todo add the proper type: (Func Float Float Float)
        // #todo even better: (Func (Many Float) Float)
        annotate_type(Expr::foreign_func(&add_float), "Float"),
    );
    module.insert_invocable(
        "+$$Float$$Float$$Float$$Float",
        // #todo add the proper type: (Func Float Float Float)
        // #todo even better: (Func (Many Float) Float)
        annotate_type(Expr::foreign_func(&add_float), "Float"),
    );
    module.insert_invocable(
        "+$$Dec$$Dec",
        // #todo add the proper type: (Func Dec Dec Dec)
        // #todo even better: (Func (Many Dec) Dec)
        annotate_type(Expr::foreign_func(&add_dec), "Dec"),
    );
    module.insert_invocable("-", Expr::foreign_func(&sub_int));
    module.insert_invocable("-$$Int", annotate_type(Expr::foreign_func(&neg_int), "Int"));
    module.insert_invocable(
        "-$$Int$$Int",
        annotate_type(Expr::foreign_func(&sub_int), "Int"),
    );
    // #todo #hack implement a version of module.insert that automatically adds a few methods/overloads.
    module.insert_invocable(
        "-$$Int$$Int$$Int",
        annotate_type(Expr::foreign_func(&sub_int), "Int"),
    );
    module.insert_invocable(
        "-$$Int$$Int$$Int$$Int",
        annotate_type(Expr::foreign_func(&sub_int), "Int"),
    );
    module.insert_invocable(
        "-$$Float",
        annotate_type(Expr::foreign_func(&neg_float), "Float"),
    );
    module.insert_invocable(
        "-$$Float$$Float",
        annotate_type(Expr::foreign_func(&sub_float), "Float"),
    );
    module.insert_invocable("*", Expr::foreign_func(&mul_int));
    module.insert_invocable(
        "*$$Int$$Int",
        annotate_type(Expr::foreign_func(&mul_int), "Int"),
    );
    // #todo #temp hack to support multiple args.
    module.insert_invocable(
        "*$$Int$$Int$$Int",
        annotate_type(Expr::foreign_func(&mul_int), "Int"),
    );
    module.insert_invocable(
        "*$$Float$$Float",
        // #todo add the proper type: (Func Float Float Float)
        // #todo even better: (Func (Many Float) Float)
        annotate_type(Expr::foreign_func(&mul_float), "Float"),
    );
    module.insert_invocable(
        "*$$Float$$Float$$Float",
        // #todo add the proper type: (Func Float Float Float)
        // #todo even better: (Func (Many Float) Float)
        annotate_type(Expr::foreign_func(&mul_float), "Float"),
    );
    module.insert_invocable("/", annotate_type(Expr::foreign_func(&div_float), "Float"));
    module.insert_invocable(
        "/$$Int$$Int",
        annotate_type(Expr::foreign_func(&div_int), "Int"),
    );
    // #todo ultra-hack
    module.insert_invocable(
        "/$$Float$$Float",
        annotate_type(Expr::foreign_func(&div_float), "Float"),
    );
    // #todo ultra-hack
    module.insert_invocable(
        "/$$Float$$Float$$Float",
        annotate_type(Expr::foreign_func(&div_float), "Float"),
    );
    // #todo Add support for float exponentiation.
    // #todo shouldn't be required.
    module.insert_invocable("%", annotate_type(Expr::foreign_func(&mod_int), "Int"));
    module.insert_invocable(
        "%$$Int$$Int",
        annotate_type(Expr::foreign_func(&mod_int), "Int"),
    );
    module.insert_invocable(
        "%$$Float$$Float",
        annotate_type(Expr::foreign_func(&mod_float), "Float"),
    );
}
