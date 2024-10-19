use ::chrono::{DateTime, NaiveDate, TimeZone, Utc};
use croner::Cron;
use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, Expr},
    util::{
        args::{unpack_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

// #todo Is cron a POSIX thing, to be organized under posix?

// #todo Extract somewhere.
// #insight Originally used from /chrono, was causing issues.
pub fn datetime_from_date(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
}

// #todo Duplicated in /chrono.
// #todo Should throw errors.
// #todo As un-optimized as it gets.
pub fn rust_date_from_tan_date(tan_date: &Expr) -> NaiveDate {
    let map = tan_date.as_map().unwrap();
    let s = format!("{}-{}-{}", map["year"], map["month"], map["day"]);
    let format_string = "%Y-%m-%d";
    // NaiveDate::from_ymd_opt(2024, 1, 18).unwrap();
    NaiveDate::parse_from_str(&s, format_string).unwrap()
}

// #todo #perf #IMPORTANT Use ForeignStruct.

pub fn cron_new(args: &[Expr]) -> Result<Expr, Error> {
    let pattern = unpack_stringable_arg(args, 0, "pattern")?;

    // #todo can we keep an opaque pointer to an actual Rust Cron instead?

    let rx = Expr::string(pattern);

    Ok(annotate_type(rx, "Cron"))
}

pub fn cron_is_matching(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Consider naming the argument both `this` _and_ `pattern`.
    let pattern = unpack_stringable_arg(args, 0, "pattern")?;
    let tan_date = unpack_arg(args, 1, "date")?;
    let rust_date = rust_date_from_tan_date(tan_date);

    // #todo Proper error reporting here!
    // #todo Support options like dom-and-dow, pass as dict.
    let Ok(cron) = Cron::new(pattern).with_dom_and_dow().parse() else {
        return Err(Error::invalid_arguments(
            &format!("invalid cron pattern: {pattern}"),
            // #todo Add correct range.
            // this.range(),
            None,
        ));
    };

    let datetime = datetime_from_date(rust_date);

    let Ok(is_matching) = cron.is_time_matching(&datetime) else {
        return Err(Error::invalid_arguments(
            &format!("invalid cron pattern: {pattern}"),
            // #todo Add correct range.
            // this.range(),
            None,
        ));
    };

    Ok(Expr::Bool(is_matching))
}

pub fn import_lib_cron(context: &mut Context) {
    // #todo Find a better module-path, something related with pattern.
    let module = require_module("cron", context);

    // (let cp (Cron "15 10 * * 1"))
    // (is-matching? cp date)

    module.insert_invocable("Cron", Expr::foreign_func(&cron_new));

    module.insert_invocable("is-matching?", Expr::foreign_func(&cron_is_matching));
}
