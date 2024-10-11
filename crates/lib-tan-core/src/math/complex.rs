// #todo implement with Tan
// #todo consider putting in Prelude.

// (let z (Complex 1.0 0.3))
// (let r (* z z))

use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, has_type_annotation, Expr},
    util::{expect_lock_read, module_util::require_module},
};

// #todo how to extend to quaternions and octonions.

fn try_complex(expr: &Expr) -> Option<(f64, f64)> {
    if !has_type_annotation(expr, "Complex") {
        return None;
    }

    let Expr::Array(v) = expr.unpack() else {
        return None;
    };

    let v = expect_lock_read(v);

    // #todo use ForeignStruct.
    let re = v[0].as_float().unwrap();
    let im = v[1].as_float().unwrap();

    Some((re, im))
}

#[inline(always)]
fn make_complex(re: impl Into<Expr>, im: impl Into<Expr>) -> Expr {
    let expr = Expr::array(vec![re.into(), im.into()]);
    annotate_type(expr, "Complex")
}

// (Complex)
// (Complex re)
// (Complex re im)
pub fn complex_new(args: &[Expr]) -> Result<Expr, Error> {
    // #todo consider a ForeignStruct.

    let re = args.first().unwrap_or_else(|| &Expr::Float(0.0));
    let im = args.get(1).unwrap_or_else(|| &Expr::Float(0.0));

    // #todo optimize the clones.
    Ok(make_complex(re.clone(), im.clone()))
}

pub fn complex_add(args: &[Expr]) -> Result<Expr, Error> {
    // #todo initialize with first arg, to save one op.

    let mut re_acc = 0.0;
    let mut im_acc = 0.0;

    for arg in args {
        let Some((re, im)) = try_complex(arg) else {
            return Err(Error::invalid_arguments(
                &format!("{arg} is not a Complex"),
                arg.range(),
            ));
        };
        re_acc += re;
        im_acc += im;
    }

    Ok(make_complex(re_acc, im_acc))
}

// (ac - bd) + (ad + bc)i.
// #todo only supports 2 arguments for the moment.
// #todo extract multiplication helper and support multiple arguments.
pub fn complex_mul(args: &[Expr]) -> Result<Expr, Error> {
    let [c, z] = args else {
        return Err(Error::invalid_arguments("requires two arguments", None));
    };

    let Some((a, b)) = try_complex(c) else {
        return Err(Error::invalid_arguments(
            &format!("{c} is not a Complex"),
            c.range(),
        ));
    };

    let Some((c, d)) = try_complex(z) else {
        return Err(Error::invalid_arguments(
            &format!("{z} is not a Complex"),
            c.range(),
        ));
    };

    // complex multiplication formula: (ac - bd) + (ad + bc)i.

    let re = (a * c) - (b * d);
    let im = (a * d) + (b * c);

    Ok(make_complex(re, im))
}

// |z| = √(a² + b²)
pub fn complex_abs(args: &[Expr]) -> Result<Expr, Error> {
    let [c] = args else {
        return Err(Error::invalid_arguments("requires `self` argument", None));
    };

    let Some((a, b)) = try_complex(c) else {
        return Err(Error::invalid_arguments(
            &format!("{c} is not a Complex"),
            c.range(),
        ));
    };

    // complex abs formula: |z| = √(a² + b²)
    // #insight the complex abs is the 'magnitude' of the complex number.

    // #todo detect and optimize pure real (a + 0i) and pure imaginary (0 + bi) cases.

    let magnitude = ((a * a) + (b * b)).sqrt();

    Ok(magnitude.into())
}

pub fn setup_lib_math_complex(context: &mut Context) {
    // #todo skip the `math/` prefix?
    let module = require_module("math/complex", context);

    // #todo make type-paremetric.
    // #todo better name?
    // (let z (Complex 1.0 0.3))
    module.insert_invocable("Complex", Expr::foreign_func(&complex_new));

    module.insert_invocable("+$$Complex$$Complex", Expr::foreign_func(&complex_add));
    module.insert_invocable(
        "+$$Complex$$Complex$$Complex",
        Expr::foreign_func(&complex_add),
    );

    module.insert_invocable("*$$Complex$$Complex", Expr::foreign_func(&complex_mul));

    // #todo move this to arithmetic or something similar.
    module.insert_invocable("abs", Expr::foreign_func(&complex_abs));
    module.insert_invocable("abs$$Complex", Expr::foreign_func(&complex_abs));

    // #todo also consider Complex:one, Complex:zero ~~ (Complex :zero) -> Complex:zero
    // #todo `Complex/one`
    // #todo `Complex/zero`
    // #todo `Complex/re`
    // #todo `Complex/im`
    // #todo `(re c)`, `(re-of c)`, `(get-re c)`
    // #todo `(im c)`, `(im-of c)`, `(get-im c)`
}
