use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::{Expr, Member, Result, Token, parse::Parse, spanned::Spanned};

pub struct Input {
    at: Token![@],
    mutability: Option<Token![mut]>,
    base: Expr,
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
            base: input.step(|cursor| {
                let mut rest = *cursor;
                let mut dash = None;
                let mut toks = vec![];
                while let Some((tt, next)) = rest.token_tree() {
                    match &tt {
                        TokenTree::Punct(punct) if punct.as_char() == '-' => {
                            dash = Some(rest);
                            rest = next;
                            toks.push(tt);
                        }
                        TokenTree::Punct(punct) if dash.is_some() && punct.as_char() == '>' => {
                            if let Some(dash) = dash {
                                toks.pop(); // remove previously pushed `-` that's part of the arrow
                                return Ok((syn::parse2(toks.into_iter().collect())?, dash));
                            } else {
                                unreachable!()
                            }
                        }
                        _ => {
                            toks.push(tt);
                            dash = None;
                            rest = next
                        }
                    }
                }
                Err(cursor.error("no `->` was found."))
            })?,
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
    let project = match &mutability {
        Some(_) => quote!(project_mut),
        None => quote!(project),
    };
    Ok(quote_spanned! {span=>
        match (
            &#mutability #base.#field,
            #compat::RawProjected::access(&#base.___projection_checker_raw),
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
