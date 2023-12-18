use spo2_derive::ffi_struct;

pub struct User {
    id: u64,
}

#[no_mangle]
pub extern "C" fn get_user() -> User {
    User { id: 1 }
}
