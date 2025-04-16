extern "C" {
    fn __add(a: i32, b: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    unsafe { __add(a, b) }
}
