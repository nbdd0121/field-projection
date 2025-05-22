use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, Result, Token, parse::Parse};

use crate::IdentOrSelf;

pub struct Input {
    kind: ProjKind,
    name: IdentOrSelf,
}

pub enum ProjKind {
    Shared,
    Mut(Token![mut]),
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

impl ToTokens for ProjKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ProjKind::Shared => {}
            ProjKind::Mut(mut_) => mut_.to_tokens(tokens),
            ProjKind::Move(move_) => move_.to_tokens(tokens),
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
    let projections = format_ident!("___projections_checker_for_{name}");
    let raw_ = format_ident!("___projections_raw_ptr_for_{name}");
    let (start, mut_, action) = match &kind {
        ProjKind::Move(_) => (quote!(__start_proj_move), quote!(), quote!()),
        ProjKind::Mut(mut_) => (quote!(__start_proj_mut), quote!(#mut_), quote!(&#mut_)),
        ProjKind::Shared => (quote!(__start_proj), quote!(), quote!(&)),
    };
    let (rest, raw_mut) = match &kind {
        ProjKind::Move(_) => (quote!(let #raw_ = &raw mut #raw_;), quote!(mut)),
        _ => (quote!(), quote!()),
    };
    quote! {
        let (#mut_ #projections, #raw_mut #raw_) = ::field_projection::compat::#start(#action #name);
        #rest
    }
}
