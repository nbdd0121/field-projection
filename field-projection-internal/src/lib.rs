//! Internal implementation details of crate `field_projection`, **do not use**.

use proc_macro::TokenStream;
use syn::Error;

mod field;

#[proc_macro_derive(Field)]
pub fn field(input: TokenStream) -> TokenStream {
    field::field(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
