use spo2_derive::ffi_struct;

#[ffi_struct]
struct EmptyStruct {}

#[ffi_struct]
struct BaseTypesStruct {
    field1: i32,
    field2: f64,
    field3: bool,
}

pub fn main() {}
