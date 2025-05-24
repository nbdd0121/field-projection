use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, Result, Token, parse::Parse, punctuated::Punctuated};

use crate::IdentOrSelf;

pub struct Input {
    input: Punctuated<Entry, Token![;]>,
}

struct Entry {
    let_: Option<Token![let]>,
    kind: ProjKind,
    name: IdentOrSelf,
    equals: Option<Token![=]>,
    value: Option<Expr>,
}

enum ProjKind {
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
        Punctuated::parse_terminated(input).map(|input| Self { input })
    }
}

impl Parse for Entry {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let let_: Option<Token![let]> = input.parse()?;
        let has_let = let_.is_some();
        let res = Self {
            let_,
            kind: input.parse()?,
            name: input.parse()?,
            equals: has_let.then(|| input.parse()).transpose()?,
            value: has_let.then(|| input.parse()).transpose()?,
        };
        Ok(res)
    }
}

pub fn expand(input: Input) -> TokenStream {
    input.input.into_iter().map(expand_one).collect()
}

fn expand_one(
    Entry {
        let_,
        kind,
        name: IdentOrSelf(name),
        equals,
        value,
    }: Entry,
) -> TokenStream {
    let let_ = let_.unwrap_or_default();
    let equals = equals.unwrap_or_default();
    let projections = format_ident!("___projections_checker_for_{name}");
    let raw_ = format_ident!("___projections_raw_ptr_for_{name}");
    let (start, mut_, action) = match &kind {
        ProjKind::Move(_) => (quote!(__start_proj_move), quote!(), quote!()),
        ProjKind::Mut(mut_) => (quote!(__start_proj_mut), quote!(#mut_), quote!(&#mut_)),
        ProjKind::Shared => (quote!(__start_proj), quote!(), quote!(&)),
    };
    let before = value.map(|value| quote!(#let_ #mut_ #name = #value;));
    let (rest, raw_mut) = match &kind {
        ProjKind::Move(_) => (quote!(let #raw_ = &raw mut #raw_;), quote!(mut)),
        _ => (quote!(), quote!()),
    };
    quote! {
        #before
        #let_ (#mut_ #projections, #raw_mut #raw_) #equals ::field_projection::compat::#start(#action #name);
        #rest
    }
}
