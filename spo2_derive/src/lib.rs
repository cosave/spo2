use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn ffi_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = proc_macro2::TokenStream::from(input.clone());

    quote! {
        input
    }
    .into()
}

#[cfg(test)]
mod tests {}
