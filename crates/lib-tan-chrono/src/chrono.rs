use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};

use tan::{
    context::Context,
    error::Error,
    expr::{annotate_type, Expr},
    scope::Scope,
    util::{
        args::{unpack_arg, unpack_int_arg, unpack_map_arg},
        module_util::require_module,
    },
};

// #todo support rfc-399 and rfc-2822

// #link https://datatracker.ietf.org/doc/html/rfc3339
// #link https://ijmacd.github.io/rfc3339-iso8601/

// #todo what is a better name for DateTime.
// #todo does it make sense to support a Date? maybe we could only support a 'DateTime'
// #todo consider another namespace?
// #todo consider adding some chrono types to the prelude.
// #todo register the`Date` and `Duration` types.

// #insight `Duration` is similar to `Time`, i.e. time is a 'duration' from 0000-00-00, explore this.
// #todo Range instead of Duration?

pub fn tan_date_time_from_rust_date_time(rust_date_time: NaiveDateTime) -> Expr {
    // #todo month0, day0 is an interesting idea.
    let mut map = HashMap::new();
    // #todo add helpers to initialize Expr::Int
    map.insert("year".to_string(), Expr::Int(rust_date_time.year() as i64));
    map.insert(
        "month".to_string(),
        Expr::Int((rust_date_time.month0() + 1) as i64),
    );
    map.insert(
        "day".to_string(),
        Expr::Int((rust_date_time.day0() + 1) as i64),
    );

    map.insert("hour".to_string(), Expr::Int(rust_date_time.hour() as i64));
    map.insert(
        "minute".to_string(),
        Expr::Int(rust_date_time.minute() as i64),
    );
    map.insert(
        "second".to_string(),
        Expr::Int(rust_date_time.second() as i64),
    );

    // #todo support annotation with multiple types/traits, e.g. both Date + Map.

    let expr = Expr::map(map);

    annotate_type(expr, "Date-Time")
}

pub fn chrono_date_time_now(_args: &[Expr]) -> Result<Expr, Error> {
    let date_time = Utc::now().naive_utc();
    Ok(tan_date_time_from_rust_date_time(date_time))
}

// #todo add unit test
pub fn chrono_date_time(args: &[Expr]) -> Result<Expr, Error> {
    if args.is_empty() {
        chrono_date_time_now(args)
    } else {
        // #todo Find good name here!
        let expr = unpack_arg(args, 0, "time")?;

        if let Expr::Int(timestamp) = expr {
            // #todo Better add explicit function for this, e.g. (Date-Time-from-unix-timestamp ...).
            let Some(date_time) = DateTime::from_timestamp(*timestamp, 0) else {
                return Err(Error::invalid_arguments("invalid timestamp", None));
            };
            Ok(tan_date_time_from_rust_date_time(date_time.naive_utc()))
        } else {
            Err(Error::invalid_arguments("invalid argument", None))
        }
    }
}

pub fn chrono_date_time_to_string(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo check dyn_type.

    let Some(map) = this.as_map() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Date-Time",
            this.range(),
        ));
    };

    // #todo error checking!

    let Some(year) = map["year"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(month) = map["month"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(day) = map["day"].as_int() else {
        return Err(Error::invalid_arguments("invalid Dat-Time", this.range()));
    };

    let Some(hour) = map["hour"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(minute) = map["minute"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(second) = map["second"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    // #todo have separate function for to-rfc-399
    let str = format!(
        "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    );

    Ok(Expr::string(str))
}

// #todo differentiate to_string from to_rfc399
pub fn chrono_date_time_to_rfc399(args: &[Expr]) -> Result<Expr, Error> {
    chrono_date_time_to_string(args)
}

pub fn tan_date_from_components(year: i64, month: i64, day: i64) -> Expr {
    // #todo month0, day0 is an interesting idea.
    let mut map = HashMap::new();
    // #todo add helpers to initialize Expr::Int
    map.insert("year".to_string(), Expr::Int(year));
    map.insert("month".to_string(), Expr::Int(month));
    map.insert("day".to_string(), Expr::Int(day));

    // #todo support annotation with multiple types/traits, e.g. both Date + Map.

    let expr = Expr::map(map);

    annotate_type(expr, "Date")
}

pub fn tan_date_from_rust_date(rust_date: NaiveDate) -> Expr {
    tan_date_from_components(
        rust_date.year() as i64,
        (rust_date.month0() + 1) as i64,
        (rust_date.day0() + 1) as i64,
    )
    // // #todo month0, day0 is an interesting idea.
    // let mut map = HashMap::new();
    // // #todo add helpers to initialize Expr::Int
    // map.insert("year".to_string(), Expr::Int(rust_date.year() as i64));
    // map.insert(
    //     "month".to_string(),
    //     Expr::Int((rust_date.month0() + 1) as i64),
    // );
    // map.insert("day".to_string(), Expr::Int((rust_date.day0() + 1) as i64));

    // // #todo support annotation with multiple types/traits, e.g. both Date + Map.

    // let expr = Expr::map(map);

    // annotate_type(expr, "Date")
}

// #todo Should throw errors.
// #todo as un-optimized as it gets.
pub fn rust_date_from_tan_date(tan_date: &Expr) -> NaiveDate {
    let map = tan_date.as_map().unwrap();
    let s = format!("{}-{}-{}", map["year"], map["month"], map["day"]);
    let format_string = "%Y-%m-%d";
    // NaiveDate::from_ymd_opt(2024, 1, 18).unwrap();
    NaiveDate::parse_from_str(&s, format_string).unwrap()
}

// #insight i64s used to match Expr::Int()

// // #ai
// /// Returns true if the input is a leap year.
// /// Leap year logic: divisible by 4 but not divisible by 100 unless also divisible by 400
// fn is_leap_year(year: i64) -> bool {
//     (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
// }

// // #ai
// fn days_in_month(month: i64, year: i64) -> i64 {
//     match month {
//         1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
//         4 | 6 | 9 | 11 => 30,
//         2 => {
//             if is_leap_year(year) {
//                 29
//             } else {
//                 28
//             }
//         }
//         _ => panic!("Invalid month"),
//     }
// }

pub fn chrono_date_now(_args: &[Expr]) -> Result<Expr, Error> {
    let date = Utc::now().naive_utc().date();
    Ok(tan_date_from_rust_date(date))
}

// #todo support construction from [year month day]

// #todo support optional format string.
pub fn chrono_date_from_string(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `str` argument", None));
    };

    let Expr::String(s) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`str` argument should be a String",
            this.range(),
        ));
    };

    // #todo make customizable.
    let format_string = "%Y-%m-%d";

    let Ok(date) = NaiveDate::parse_from_str(s, format_string) else {
        return Err(Error::invalid_arguments("invalid date string", None));
    };

    Ok(tan_date_from_rust_date(date))
}

pub fn chrono_date(args: &[Expr]) -> Result<Expr, Error> {
    if args.is_empty() {
        chrono_date_now(args)
    } else {
        chrono_date_from_string(args)
    }
}

pub fn chrono_date_from_components(args: &[Expr]) -> Result<Expr, Error> {
    let year = unpack_int_arg(args, 0, "year")?;
    let month = unpack_int_arg(args, 1, "month")?;
    let day = unpack_int_arg(args, 2, "day")?;
    Ok(tan_date_from_components(year, month, day))
}

// #todo Rename to day-of-week?
// #insight Returns weekday in 0..=6
// #insight In compliance to the ISO-8601 standard, the first day of the week is Monday.
// Mon = 0,
// Tue = 1,
// Wed = 2,
// Thu = 3,
// Fri = 4,
// Sat = 5,
// Sun = 6,
pub fn chrono_date_day_of_week(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Check that this is a valid Date.
    let tan_date = unpack_arg(args, 0, "date")?;
    let rust_date = rust_date_from_tan_date(tan_date);
    Ok(Expr::Int(rust_date.weekday().num_days_from_monday() as i64))
}

pub fn chrono_date_from_map(args: &[Expr]) -> Result<Expr, Error> {
    let map = unpack_map_arg(args, 0, "components")?;
    let expr = Expr::map(map.clone());
    Ok(annotate_type(expr, "Date"))
}

pub fn chrono_date_to_string(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo check dyn_type.

    let Some(map) = this.as_map() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Date-Time",
            this.range(),
        ));
    };

    // #todo error checking!

    let Some(year) = map["year"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(month) = map["month"].as_int() else {
        return Err(Error::invalid_arguments("invalid Date-Time", this.range()));
    };

    let Some(day) = map["day"].as_int() else {
        return Err(Error::invalid_arguments("invalid Dat-Time", this.range()));
    };

    // #todo have separate function for to-rfc-399
    let str = format!("{}-{:02}-{:02}", year, month, day);

    Ok(Expr::string(str))
}

// #todo Implement me!
// pub fn chrono_date_to_rfc399(args: &[Expr]) -> Result<Expr, Error> {
//     let [this] = args else {
//         return Err(Error::invalid_arguments("requires `this` argument", None));
//     };
//
//     // #todo check dyn_type.
//
//     let Some(map) = this.as_map() else {
//         return Err(Error::invalid_arguments(
//             "`this` argument should be a Date",
//             this.range(),
//         ));
//     };
//
//     // #todo error checking!
//
//     let Some(year) = map["year"].as_int() else {
//         return Err(Error::invalid_arguments("invalid Date", this.range()));
//     };
//
//     let Some(month) = map["month"].as_int() else {
//         return Err(Error::invalid_arguments("invalid Date", this.range()));
//     };
//
//     let Some(day) = map["day"].as_int() else {
//         return Err(Error::invalid_arguments("invalid Date", this.range()));
//     };
//
//     let str = format!("{}-{:02}-{:02}T00:00:00", year, month, day);
//
//     Ok(Expr::string(str))
// }

// https://docs.rs/chrono/latest/chrono/format/strftime/
pub fn chrono_date_format(args: &[Expr]) -> Result<Expr, Error> {
    let [spec, date] = args else {
        return Err(Error::invalid_arguments(
            "requires `spec` and `date` arguments",
            None,
        ));
    };

    let rust_date = rust_date_from_tan_date(date);

    let Some(fmt) = spec.as_stringable() else {
        return Err(Error::invalid_arguments(
            "`format-spec` argument should be a Stringable",
            spec.range(),
        ));
    };

    let output = rust_date.format(fmt);

    Ok(Expr::string(output.to_string()))
}

// #todo Add Date-Time add helpers.

// #todo Also implement Duration and (+ Date Duration).

// #todo Consider changing the order of arguments?
pub fn chrono_date_add_days(args: &[Expr]) -> Result<Expr, Error> {
    let [this, days] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo check dyn_type.

    let Some(days) = days.as_int() else {
        return Err(Error::invalid_arguments(
            "`days` argument should be an Int",
            this.range(),
        ));
    };

    let rust_date = rust_date_from_tan_date(this);

    let new_rust_date = rust_date + Duration::days(days);

    Ok(tan_date_from_rust_date(new_rust_date))
}

// #todo replace with + Duration
pub fn chrono_date_add_weeks(args: &[Expr]) -> Result<Expr, Error> {
    let [this, weeks] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo check dyn_type.

    let Some(weeks) = weeks.as_int() else {
        return Err(Error::invalid_arguments(
            "`weeks` argument should be an Int",
            this.range(),
        ));
    };

    let rust_date = rust_date_from_tan_date(this);

    let new_rust_date = rust_date + Duration::weeks(weeks);

    Ok(tan_date_from_rust_date(new_rust_date))
}

pub fn import_lib_chrono(context: &mut Context) {
    let module = require_module("chrono", context);

    module.insert_invocable("Date-Time", Expr::foreign_func(&chrono_date_time));
    // #todo consider `to-rfc-399-string` or `format-rfc-399`
    module.insert_invocable(
        "to-rfc-399",
        Expr::foreign_func(&chrono_date_time_to_rfc399),
    );
    module.insert_invocable(
        "to-rfc-399$$Date-Time",
        Expr::foreign_func(&chrono_date_time_to_rfc399),
    );
    // #todo Cannot differentiate from the Date-Time version, maybe put in different module-path?
    // module.insert_invocable(
    //     "to-rfc-399$$Date",
    //     Expr::foreign_func(&chrono_date_to_rfc399)),
    // );
    // #todo consider (String date-time)
    // #insight #hack this is added in prelude! NASTY hack
    // module.insert_invocable(
    //     "to-string$$Date-Time",
    //     Expr::foreign_func(&chrono_date_time_to_string)),
    // );

    module.insert_invocable("Date", Expr::foreign_func(&chrono_date));
    module.insert_invocable("Date$$Map", Expr::foreign_func(&chrono_date_from_map));
    module.insert_invocable(
        "Date$$Int$$Int$$Int",
        Expr::foreign_func(&chrono_date_from_components),
    );

    // #todo #deprecate Remove one of these aliases.
    module.insert_invocable("day-of-week", Expr::foreign_func(&chrono_date_day_of_week));
    module.insert_invocable("weekday-of", Expr::foreign_func(&chrono_date_day_of_week));

    // #todo implement with duration and `+`.
    module.insert_invocable("add-days", Expr::foreign_func(&chrono_date_add_days));
    module.insert_invocable("add-weeks", Expr::foreign_func(&chrono_date_add_weeks));
    // #insight spec comes first for more 'natural' currying.
    // #todo maybe just pass optional parameters to to-string?
    // #todo what would be a better name? stringf, strfmt? format is just too generic to reserve.
    // #todo just make this (String date)?
    module.insert_invocable("format-string", Expr::foreign_func(&chrono_date_format));
    module.insert_invocable(
        "format-string$$Date",
        Expr::foreign_func(&chrono_date_format),
    );
    // #todo How to do this?
    // module.insert_invocable(
    //     "String$$Date",
    //     Expr::foreign_func(&chrono_date_format)),
    // );
    // #todo add more functions

    // #todo #hack Think of a better implementation.
    // #todo This is an ULTRA-HACK!

    let prelude_chrono_scope = Arc::new(Scope::default());

    // #todo Just use (Str date)!

    prelude_chrono_scope.insert(
        "to-string$$Date",
        Expr::foreign_func(&chrono_date_to_string),
    );

    prelude_chrono_scope.insert(
        "to-string$$Date-Time",
        Expr::foreign_func(&chrono_date_time_to_string),
    );

    // #todo Seems to be called _multiple_ times, investigate!
    // #todo Put a println and run the tests to investigate.
    context.import_scope_into_prelude(prelude_chrono_scope.clone());
}
