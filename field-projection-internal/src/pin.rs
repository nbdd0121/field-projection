use const_fnv1a_hash::fnv1a_hash_str_64 as field_name_hash;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Error, Fields, GenericParam, Generics, Member,
    Result,
};

pub fn pin_field(input: TokenStream) -> Result<TokenStream> {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse2(input)?;

    // Check this is a struct, and extract inner.
    let data = match data {
        Data::Struct(v) => v,
        Data::Enum(v) => {
            return Err(Error::new(
                v.enum_token.span,
                "#[derive(Field)] cannot be applied to enum",
            ))
        }
        Data::Union(v) => {
            return Err(Error::new(
                v.union_token.span,
                "#[derive(Field)] cannot be applied to union",
            ))
        }
    };

    let fields = match data.fields {
        Fields::Named(v) => v.named,
        Fields::Unnamed(v) => v.unnamed,
        Fields::Unit => Punctuated::new(),
    };

    // Check if `#[pin]` attribute has been used.
    let has_pin: Vec<_> = fields
        .iter()
        .map(|field| field.attrs.iter().any(|a| a.path.is_ident("pin")))
        .collect();

    let field_name: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(v) => Member::Named(v.clone()),
            None => Member::Unnamed(i.into()),
        })
        .collect();

    // Extract generics and where clauses
    let Generics {
        params: generics,
        where_clause,
        ..
    } = generics;
    let generics: Vec<_> = generics
        .into_iter()
        .map(|mut x| {
            match &mut x {
                GenericParam::Lifetime(_) => (),
                GenericParam::Type(t) => t.default = None,
                GenericParam::Const(c) => c.default = None,
            }
            x
        })
        .collect();
    let ty_generics: Vec<_> = generics
        .iter()
        .map(|x| -> &dyn ToTokens {
            match x {
                GenericParam::Lifetime(l) => &l.lifetime,
                GenericParam::Type(t) => &t.ident,
                GenericParam::Const(c) => &c.ident,
            }
        })
        .collect();

    let mixed_site = Span::mixed_site();
    let mut builder = Vec::new();

    for i in 0..field_name.len() {
        let field_name_current = &field_name[i];
        let field_name_str = match field_name_current {
            Member::Named(v) => v.to_string(),
            Member::Unnamed(v) => v.index.to_string(),
        };
        let ty = &fields[i].ty;
        let field_name_hash = field_name_hash(&field_name_str);

        let wrapper_ty = if has_pin[i] {
            quote_spanned!(mixed_site => Pin<&'__field_projection mut __FieldProjection>)
        } else {
            quote_spanned!(mixed_site => &'__field_projection mut __FieldProjection)
        };

        builder.push(quote_spanned! {mixed_site =>
            unsafe impl<
                #(#generics,)*
            > field_projection::PinField for FieldOffset<
                #ident<#(#ty_generics,)*>, #field_name_hash
            > #where_clause
            {
                type PinWrapper<'__field_projection, __FieldProjection: ?Sized + '__field_projection> = #wrapper_ty;
            }
        })
    }

    let gen = quote!(#(#builder)*);
    Ok(gen)
}
