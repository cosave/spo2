use proc_macro::TokenStream;
use quote::quote;
use spo2_core::FfiStruct;
use syn::{parse2, ItemStruct};

#[proc_macro_attribute]
pub fn ffi_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = FfiStruct::try_from(proc_macro2::TokenStream::from(input.clone()))
        .expect("Unable to parse as Struct");
    if input_struct.is_copy_safe() {
        input
    } else {
        quote! {
             compile_error!("Only copy type structs are supported");
        }
        .into()
    }
}

#[proc_macro_attribute]
pub fn ffi_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct =
        parse2::<ItemStruct>(input.clone().into()).expect(&format!("Unable to parse as Struct"));
    let input_struct = FfiStruct::new(input_struct);
    if input_struct.is_copy_safe() {
        input
    } else {
        quote! {
             compile_error!("Only copy type structs are supported");
        }
        .into()
    }
}

#[cfg(test)]
mod tests {}
