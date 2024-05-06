#[no_mangle]
pub extern "C" fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}
