//! Internal implementation details of crate `field_projection`, **do not use**.

use proc_macro::TokenStream;
use syn::Error;

mod field;
mod pin;

#[proc_macro_derive(Field)]
pub fn field(input: TokenStream) -> TokenStream {
    field::field(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(PinField, attributes(pin))]
pub fn pin_field(input: TokenStream) -> TokenStream {
    pin::pin_field(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
