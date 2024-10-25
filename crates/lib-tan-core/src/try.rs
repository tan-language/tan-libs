// #ref https://github.com/bruxisma/outcome

// #todo Consider Outcome = { Try | Maybe } instead of Try = { Outcome | Maybe }
// #todo Consider using Maybe for Try or vice-versa.
// #todo Consider (Ok ...) | Err (err), (Some ...) | ()
// #todo Even consider T | (Err err), T | ()

// #todo Support error accumulation, e.g. in form validation, like Haskell's Validation.

// #todo Maybe we don't need try operator and just a monadic (do ...) works?
// #todo Even in that case it would be nice to make clear that something is fallible.
