use ::chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono::import_lib_chrono;
use tan::context::Context;

pub mod chrono;

pub fn datetime_from_date(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
}

// #todo Find a good name for this: considere import_*, link_*
#[no_mangle]
pub fn install_foreign_dyn_lib(context: &mut Context) {
    import_lib_chrono(context);
}
