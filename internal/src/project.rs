use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{Member, Result, Token, parse::Parse, spanned::Spanned};

use crate::IdentOrSelf;

pub struct Input {
    at: Token![@],
    mut_: Option<Token![mut]>,
    base: IdentOrSelf,
    arrow: Token![->],
    field: Member,
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.at.to_tokens(tokens);
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
        mut_,
        base: IdentOrSelf(base),
        field,
        ..
    } = input;
    let base = format_ident!("___projections_for_{base}");
    let compat = quote!(::field_projection::compat);
    let (project, access) = match &mut_ {
        Some(_) => (quote!(project_mut), quote!(access_mut)),
        None => (quote!(project), quote!(access)),
    };
    Ok(quote_spanned! {span=>
        match (
            &#mut_ #base.#field,
            #compat::RawProjectionAccess::#access(&#mut_ #base.___projection_checker_raw),
        ) {
            (this, raw) => {
                #compat::ProjectedField::safety_check(&*this).check();
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
