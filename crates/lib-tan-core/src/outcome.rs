// #ref https://github.com/bruxisma/outcome

// #todo Consider Outcome = { Try | Maybe } instead of Try = { Outcome | Maybe }
// #todo Consider using Maybe for Try or vice-versa.
// #todo Consider (Ok ...) | Err (err), (Some ...) | ()
// #todo Even consider T | (Err err), T | ()

// #todo Support error accumulation, e.g. in form validation, like Haskell's Validation.

// #todo Maybe we don't need try operator and just a monadic (do ...) works?
// #todo Even in that case it would be nice to make clear that something is fallible.

// #insight Use the Outcome type instead of (Or T Error) to enforce handling of the error case.
// #ref More benefits: https://gemini.google.com/app/44afb9ab819a24fb

// (try (func x y)) <- (?func x y)
// (unwrap (func x y)) <- (!func x y)
// (unwrap outcome) <- !outcome
// (is-ok? outcome)
// (is-err? outcome)
// (if (let (Ok x) outcome) ... (else ...))

fn outcome_new(rgs: &[Expr]) -> Result<Expr, Error> {
    let reason = unpack_stringable_arg(args, 0, "reason")?;
    Ok(Expr::error(reason))
}
