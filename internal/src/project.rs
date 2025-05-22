use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{Member, Result, Token, parse::Parse, spanned::Spanned};

use crate::{IdentOrSelf, start::ProjKind};

pub struct Input {
    at: Token![@],
    move_: Option<Token![move]>,
    mut_: Option<Token![mut]>,
    base: IdentOrSelf,
    arrow: Token![->],
    field: Member,
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.at.to_tokens(tokens);
        self.move_.to_tokens(tokens);
        self.mut_.to_tokens(tokens);
        self.base.to_tokens(tokens);
        self.arrow.to_tokens(tokens);
        self.field.to_tokens(tokens);
    }
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let res = Self {
            at: input.parse()?,
            move_: input.parse()?,
            mut_: input.parse()?,
            base: input.parse()?,
            arrow: input.parse()?,
            field: input.parse()?,
        };
        if !input.is_empty() {
            return Err(input.error("unexpected tokens"));
        }
        Ok(res)
    }
}

pub fn expand(input: Input) -> Result<TokenStream> {
    let span = input.span();
    let Input {
        move_,
        mut_,
        base: IdentOrSelf(base),
        field,
        ..
    } = input;
    let raw_ = format_ident!("___projections_raw_ptr_for_{base}");
    let base = format_ident!("___projections_checker_for_{base}");
    let compat = quote!(::field_projection::compat);
    let (action, deref, project) = match &(move_, mut_) {
        (Some(_), Some(_)) => (quote!(), quote!(), quote!(project_move_mut)),
        (Some(_), None) => (quote!(), quote!(), quote!(project_move)),
        (None, Some(mut_)) => (quote!(&#mut_), quote!(*), quote!(project_mut)),
        (None, None) => (quote!(&), quote!(*), quote!(project)),
    };
    Ok(quote_spanned! {span=>
        match (
            #action #base.#field,
            #raw_
        ) {
            (this, raw) => {
                #compat::ProjectedField::safety_check(& #deref this).check();
                #[allow(unused_unsafe)]
                let res = unsafe {
                    #compat::ProjectedField::#project(
                        this,
                        raw,
                    )
                };
                res
            }
        }
    })
}
