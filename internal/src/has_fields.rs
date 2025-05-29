use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Generics, Ident, Result,
    Token,
    parse::{Parse, Parser},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
};

pub fn derive(
    DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    }: DeriveInput,
) -> Result<TokenStream> {
    let core_ = quote!(::field_projection);
    let compat = quote!(::field_projection::compat);
    let field_name = format_ident!("__FIELD_NAME");
    let field_info = format_ident!("FieldInfo");

    let fields_attrs: Vec<_> = attrs
        .iter()
        .filter_map(|a| a.meta.require_list().ok())
        .filter_map(|l| l.path.is_ident("fields").then_some(l.tokens.clone()))
        .map(|toks| {
            Parser::parse2(Punctuated::<FieldAttr, Token![,]>::parse_terminated, toks)
                .map(|p| p.into_iter().collect::<Vec<_>>())
        })
        .collect::<Result<Vec<Vec<FieldAttr>>>>()?
        .into_iter()
        .flatten()
        .collect();
    let emit_pinned_impls = fields_attrs.contains(&FieldAttr::WithPinned);

    let mut info_generics = generics.clone();
    info_generics.lt_token.get_or_insert_default();
    info_generics.gt_token.get_or_insert_default();
    // TODO: proper index
    info_generics
        .params
        .insert(0, parse_quote!(const #field_name: ::core::primitive::u64));
    let (impl_gen, ty_gen, whr) = generics.split_for_impl();
    let (_, info_ty_gen, _) = info_generics.split_for_impl();
    let Data::Struct(DataStruct { fields, .. }) = &data else {
        return Err(Error::new(
            match &data {
                Data::Enum(e) => e.enum_token.span,
                Data::Union(u) => u.union_token.span,
                Data::Struct(_) => unreachable!(),
            },
            "can only derive `HasFields` for structs.",
        ));
    };
    let Fields::Named(fields) = fields else {
        return Err(Error::new(
            fields.span(),
            "can only derive `HasFields` for structs with named fields.",
        ));
    };
    if let Some(span) = attrs
        .iter()
        .filter_map(|a| {
            let l = a.meta.require_list().ok()?;
            (l.path.is_ident("repr")
                && l.tokens.clone().into_iter().any(|tt| match tt {
                    TokenTree::Ident(i) => i == "packed",
                    _ => false,
                }))
            .then_some(a.span())
        })
        .next()
    {
        return Err(Error::new(
            span,
            "cannot derive `HasFields` for packed structs.",
        ));
    }
    let field_impls = fields.named.iter().map(
        |Field {
             attrs,
             ident: field,
             ty,
             ..
         }| {
            let info_ty_gen = info_ty_gen
                .to_token_stream()
                .into_iter()
                .map(|mut tok| {
                    match &mut tok {
                        TokenTree::Ident(i) if i == &field_name => {
                            tok = TokenTree::Group(Group::new(
                                Delimiter::Brace,
                                quote!(#compat::hash_field_name(::core::stringify!(#field))),
                            ));
                        }
                        _ => {}
                    }
                    tok
                })
                .collect::<TokenStream>();
            let pinned_impl = if emit_pinned_impls {
                let lt = quote!('__a);
                let r = format_ident!("__r");
                let (projected, body) = match attrs.iter().any(|a| {
                    a.meta
                        .require_path_only()
                        .ok()
                        .map(|p| p.is_ident("pin"))
                        .unwrap_or(false)
                }) {
                    true => (
                        quote!(::core::pin::Pin<&#lt mut #ty>),
                        quote!(unsafe { ::core::pin::Pin::new_unchecked(#r) }),
                    ),
                    false => (quote!(&#lt mut #ty), quote!(#r)),
                };
                quote! {
                    unsafe impl #impl_gen #core_::marker::PinnableField
                    for #field_info #info_ty_gen
                        #whr
                    {
                        type Projected<#lt> = #projected
                            where #ty: #lt;

                        unsafe fn from_pinned_ref<#lt>(#r: &#lt mut #ty) -> Self::Projected<#lt> {
                            #body
                        }
                    }
                }
            } else {
                quote!()
            };
            quote! {
                unsafe impl #impl_gen #core_::marker::UnalignedField
                for #field_info #info_ty_gen
                    #whr
                {
                    type Base = #ident #ty_gen;
                    type Type = #ty;
                    const OFFSET: usize = ::core::mem::offset_of!(#ident #ty_gen, #field);
                }
                // SAFETY: we checked that the struct is not `repr(packed)`.
                unsafe impl #impl_gen #core_::marker::Field
                for #field_info #info_ty_gen
                    #whr
                {
                }

                #pinned_impl
            }
        },
    );
    let checker = checker(&ident, &generics, &compat, &core_, fields);
    Ok(quote! {
        const _: () = {
            #vis struct #field_info #info_generics {
                _phantom: ::core::marker::PhantomData<#ident #ty_gen>,
            }

            unsafe impl #impl_gen #compat::HasFields for #ident #ty_gen
                #whr
            {
                type FieldInfo<
                    const #field_name: ::core::primitive::u64,
                > = #field_info #info_ty_gen;
            }

            #(#field_impls)*

            #checker
        };
    })
}

fn checker(
    ident: &Ident,
    generics: &Generics,
    compat: &TokenStream,
    core_: &TokenStream,
    fields: &FieldsNamed,
) -> TokenStream {
    let proj = format_ident!("__Proj");
    let mut generics = generics.clone();
    generics.lt_token.get_or_insert_default();
    generics.gt_token.get_or_insert_default();
    let (_, ident_ty_gen, whr) = generics.split_for_impl();
    let mut generics = generics.clone();
    // TODO: proper index
    generics.params.insert(
        0,
        parse_quote!(#proj: #compat::ProjectableExt<Inner = #ident #ident_ty_gen>),
    );
    let (checker_impl_gen, checker_ty_gen, _) = generics.split_for_impl();
    let fields = fields.named.iter().map(
        |Field {
             vis,
             ident: name,
             ..
         }| {
            quote! {
                #vis #name: #compat::ProjectedField<#proj, #core_::marker::field_of!(#ident #ident_ty_gen, #name)>
            }
        },
    );
    quote! {
        pub struct Checker #generics {
            #(#fields,)*
        }

        unsafe impl #checker_impl_gen #compat::ProjectionChecker for Checker #checker_ty_gen {
            type Proj = #proj;
        }

        unsafe impl #checker_impl_gen #compat::CheckedProject<#proj> for #ident #ident_ty_gen
            #whr
        {
            type Checker = Checker #checker_ty_gen;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FieldAttr {
    WithPinned,
}

impl Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "with_pinned" {
            Ok(Self::WithPinned)
        } else {
            Err(Error::new(ident.span(), "unknown field attribute"))
        }
    }
}
