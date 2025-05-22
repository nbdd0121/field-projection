use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Ident, Token, parse::Parse, parse_macro_input};

mod has_fields;
mod project;
mod start;

#[proc_macro]
pub fn p(input: TokenStream) -> TokenStream {
    project::expand(parse_macro_input!(input as _))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro]
pub fn start_proj(input: TokenStream) -> TokenStream {
    start::expand(parse_macro_input!(input as _)).into()
}

#[proc_macro_derive(HasFields, attributes(fields, pin))]
pub fn derive_has_fields(item: TokenStream) -> TokenStream {
    has_fields::derive(parse_macro_input!(item as DeriveInput))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

struct IdentOrSelf(Ident);

impl Parse for IdentOrSelf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![self]) {
            let self_: Token![self] = input.parse()?;
            Ok(Self(Ident::new("self", self_.span)))
        } else {
            input.parse().map(Self)
        }
    }
}

impl ToTokens for IdentOrSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
