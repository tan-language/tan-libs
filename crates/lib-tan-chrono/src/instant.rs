// #note Not used yet!

// Instant represents a point in time.

use std::{
    ops::Add,
    sync::Arc,
    time::{Duration, Instant},
};

use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, Expr},
    util::{
        args::{unpack_array_arg, unpack_foreign_arg},
        module_util::require_module,
    },
};

// #insight This is instance_new.
pub fn instant_now(_args: &[Expr]) -> Result<Expr, Error> {
    let instant = std::time::Instant::now();
    let expr = Expr::Foreign(Arc::new(instant));
    Ok(annotate_type(expr, "Instant"))
}

// #todo Support i + d _and_ d + i.
pub fn instant_add_duration(args: &[Expr]) -> Result<Expr, Error> {
    let instant = unpack_foreign_arg(args, 0, "instant", "Instant")?;
    let duration = unpack_array_arg(args, 1, "duration")?;
    let Some(instant) = instant.downcast_ref::<Instant>() else {
        return Err(Error::invalid_arguments("invalid Instant", args[0].range()));
    };

    // #todo Do something better here.
    let duration = Duration::new(
        duration[0].as_int().unwrap() as u64,
        duration[1].as_int().unwrap() as u32,
    );

    let new_instant = instant.add(duration);

    let expr = Expr::Foreign(Arc::new(new_instant));
    Ok(annotate_type(expr, "Instant"))
}

pub fn import_lib_chrono_instant(context: &mut Context) {
    let module = require_module("chrono", context);

    // #todo Consider the names `Instant-now`, or even `Now`, or `now`.
    module.insert_invocable("Instant", Expr::foreign_func(&instant_now));

    module.insert_invocable(
        "+$$Instant$$Duration",
        Expr::foreign_func(&instant_add_duration),
    );
}
