#![allow(unused)]
mod ffi_struct;

#[test]
fn test_ffi_struct_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ffi_struct/pass.rs");
    t.compile_fail("tests/ffi_struct/fail.rs");
}
