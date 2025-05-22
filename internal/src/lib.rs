use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod has_fields;
mod project;

#[proc_macro]
pub fn p(input: TokenStream) -> TokenStream {
    project::expand(parse_macro_input!(input as _))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_derive(HasFields, attributes(fields, pin))]
pub fn derive_has_fields(item: TokenStream) -> TokenStream {
    has_fields::derive(parse_macro_input!(item as DeriveInput))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
