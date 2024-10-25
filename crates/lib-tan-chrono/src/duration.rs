// #note Not used yet.

// #todo `Interval` is another good name.
// #ai #ref https://gemini.google.com/app/51f4625581523324

// #insight #ai
// Use Duration when you need to work with a length of time in a general sense.
// Use Interval when you need to represent a specific time span with defined
// start and end points. An interval is like a range? This use of Interval comes
// from joda-time but I am not sure it's correct.

// #todo
// Have predefined durations:
// - Duration/one-day, one-day-duration, one-day, ONE-DAY

// #todo Arithmetic between Date/Date-Time + Duration.
// #todo Implement with Tan?

// (Duration s ns)
// (Duration/days 1)
// (Duration/hours 1)

// #insight 1 second = 1,000,000,000 nano-seconds, 1 nano-second = 10**-9 seconds.

// Duration represents a span of time.

use std::time::Instant;

use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, Expr},
    util::{args::unpack_foreign_arg, module_util::require_module},
};

// (let t (Instant))
// (let duration (Duration-from t))

// Duration could be just an array. [secs, nanos].
// Duration or use a struct or map?

pub fn duration_from_instant(args: &[Expr]) -> Result<Expr, Error> {
    let instant = unpack_foreign_arg(args, 0, "instant", "Instant")?;
    let Some(instant) = instant.downcast_ref::<Instant>() else {
        return Err(Error::invalid_arguments("invalid Instant", args[0].range()));
    };
    let duration = instant.elapsed();
    let secs = Expr::Int(duration.as_secs() as i64);
    let nanos = Expr::Int(duration.as_nanos() as i64);
    // let expr = Expr::Foreign(Arc::new(duration));
    // Ok(annotate_type(expr, "Duration"))
    let expr = Expr::array(vec![secs, nanos]);
    Ok(annotate_type(expr, "Duration"))
}

pub fn import_lib_chrono_duration(context: &mut Context) {
    let module = require_module("chrono", context);

    // #todo Consider (duration-from instant).
    module.insert_invocable(
        "Duration-from$$Instant",
        Expr::foreign_func(&duration_from_instant),
    );
}
