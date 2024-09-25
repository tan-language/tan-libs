// #insight random is _not_ part of math! (are you sure?)

// #todo move out of standard library into the common library

// #todo Investigate the design of C++
// static std::uniform_real_distribution<double> distribution(0.0, 1.0);
// static std::mt19937 generator;
// return distribution(generator);

// #todo random_int, random_float
// #todo should take a range trait.
// #todo support random-number-generator 'object', with more specialization
// #todo fast, crypto-secure, very-random, per-thread, etc versions.
// #todo but also a couple of helper functions.
// #todo better module name: stochastic, rnd, rng? `rng` is interesting, nah 'rng' is not great.

// #todo use OnceLock to cache the RNG

// #todo better (?) api:
// (use random)
// (let n (random/int 5))
// (let n (random/float 5))
// (let n (random/num 5)) ; generic
//
// (use [RNG] /random)
// (let rng (RNG))
// (let n (gen-float rng 5.0))

// #todo add support for seeding.

// #todo Extract as standalone library?

use rand::Rng;

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_float_arg, module_util::require_module},
};

/// (random 100) returns a random integer in the range 0..100
pub fn random_int(args: &[Expr]) -> Result<Expr, Error> {
    if let Some(end) = args.first() {
        let Some(end) = end.as_int() else {
            return Err(Error::invalid_arguments(
                "expected Int argument",
                end.range(),
            ));
        };

        let mut rng = rand::thread_rng();

        Ok(Expr::Int(rng.gen_range(0..end)))
    } else {
        Err(Error::invalid_arguments(
            "expected at least one argument",
            None,
        ))
    }
}

// #todo Should make random generic, and also work with type-inference.
// #todo Generics could also reuse the `/` notation, e.g. (random/float 100.0)

/// (random-float) returns a random integer in the range 0.0..1.0
/// (random-float 100.0) returns a random integer in the range 0.0..100.0
/// (random-float 20.0 100.0) returns a random integer in the range 20.0..100.0
pub fn random_float(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Also support a range argument!

    let mut rng = rand::thread_rng();

    match args.len() {
        0 => Ok(Expr::Float(rng.gen())),
        1 => {
            let end = unpack_float_arg(args, 0, "end")?;
            Ok(Expr::Float(rng.gen_range(0.0..end)))
        }
        2 => {
            let start = unpack_float_arg(args, 0, "start")?;
            let end = unpack_float_arg(args, 1, "end")?;
            Ok(Expr::Float(rng.gen_range(start..end)))
        }
        _ => Err(Error::invalid_arguments(
            "expected at least one argument to random-float",
            None,
        )),
    }
}

pub fn import_lib_rng(context: &mut Context) {
    // #todo What is a good path? should avoid math?
    // #todo The module path `rand` is better.
    let module = require_module("rng", context);

    // #todo better name?
    module.insert("random", Expr::foreign_func(&random_int));
    // #todo better name?
    module.insert("random-float", Expr::foreign_func(&random_float));
}

// #todo add unit tests.
