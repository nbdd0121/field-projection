use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Result, Token, parse::Parse};

use crate::IdentOrSelf;

pub struct Input {
    kind: ProjKind,
    name: IdentOrSelf,
}

enum ProjKind {
    Shared,
    Mut(Token![mut]),
    #[expect(dead_code)]
    Move(Token![move]),
}

impl Parse for ProjKind {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        if input.peek(Token![mut]) {
            input.parse().map(Self::Mut)
        } else if input.peek(Token![move]) {
            input.parse().map(Self::Move)
        } else {
            Ok(Self::Shared)
        }
    }
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let res = Self {
            kind: input.parse()?,
            name: input.parse()?,
        };
        if !input.is_empty() {
            return Err(input.error("unexpected tokens"));
        }
        Ok(res)
    }
}

pub fn expand(
    Input {
        kind,
        name: IdentOrSelf(name),
    }: Input,
) -> TokenStream {
    let projections = format_ident!("___projections_for_{name}");
    let (start, mut_, action) = match &kind {
        ProjKind::Move(_) => (quote!(__start_proj_move), quote!(mut), quote!()),
        ProjKind::Mut(mut_) => (quote!(__start_proj_mut), quote!(#mut_), quote!(&#mut_)),
        ProjKind::Shared => (quote!(__start_proj), quote!(), quote!(&)),
    };
    quote! {
        let #mut_ #projections = ::field_projection::compat::#start(#action #name);
    }
}
