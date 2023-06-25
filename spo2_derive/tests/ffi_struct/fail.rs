use std::ffi::c_char;

use spo2_derive::ffi_struct;

#[ffi_struct]
struct CPointerStruct {
    field1: *mut c_char,
    field2: f64,
    field3: bool,
}

pub fn main() {}
