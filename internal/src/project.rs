use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::{Ident, Member, Result, Token, parse::Parse, spanned::Spanned};

pub struct Input {
    at: Token![@],
    mutability: Option<Token![mut]>,
    base: Ident,
    arrow: Token![->],
    field: Member,
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.at.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.base.to_tokens(tokens);
        self.arrow.to_tokens(tokens);
        self.field.to_tokens(tokens);
    }
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let res = Self {
            at: input.parse()?,
            mutability: input.parse()?,
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
        mutability,
        base,
        field,
        ..
    } = input;
    let compat = quote!(::field_projection::compat);
    let (project, access) = match &mutability {
        Some(_) => (quote!(project_mut), quote!(access_mut)),
        None => (quote!(project), quote!(access)),
    };
    Ok(quote_spanned! {span=>
        match (
            &#mutability #base.#field,
            #compat::RawProjected::#access(&#mutability #base.___projection_checker_raw),
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
