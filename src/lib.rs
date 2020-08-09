#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Enumerate, attributes(enumerate))]
pub fn derive_enumerate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let e = match &ast.data {
        syn::Data::Enum(e) => e,
        syn::Data::Struct(syn::DataStruct { struct_token, .. }) => {
            struct_token
                .span()
                .unwrap()
                .error("Expected `enum` found `struct`.")
                .emit();
            return TokenStream::new();
        }
        syn::Data::Union(syn::DataUnion { union_token, .. }) => {
            union_token
                .span()
                .unwrap()
                .error("Expected `enum` found `union`.")
                .emit();
            return TokenStream::new();
        }
    };

    let variants: Result<Vec<_>, _> = e.variants.iter().map(process_variant).collect();

    if variants.is_err() {
        return TokenStream::new();
    }
    let variants = variants.unwrap().into_iter().filter_map(|v| v);

    (quote! {
        impl #name {
            fn enumerate() -> impl Iterator<Item = &'static #name> {
                const VARS: &[#name] = &[#( #name::#variants ),*];
                VARS.iter()
            }
        }
    })
    .into()
}

fn process_variant(v: &syn::Variant) -> Result<Option<&syn::Ident>, ()> {
    let syn::Variant { ident, attrs, .. } = v;
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "enumerate" {
            let mut tokens = attr.tokens.clone().into_iter();
            if let Some(TokenTree::Group(ref group)) = tokens.next() {
                let mut inner = group.stream().into_iter();
                if let Some(TokenTree::Ident(ident)) = inner.next() {
                    if ident == "skip" && inner.next().is_none() {
                        return Ok(None);
                    }
                }
                let stream = group.stream();
                stream
                    .span()
                    .unwrap()
                    .error(format!("Unexpected attribute: `{}`", stream))
                    .emit();
                return Err(());
            }
        }
    }
    if v.fields != syn::Fields::Unit {
        v.span()
            .unwrap()
            .error("Cannot enumerate enum with tuple or struct variant.")
            .help("Try skipping the variant with the `#[enumerate(skip)]` attribute.")
            .emit();
        Err(())
    } else {
        Ok(Some(ident))
    }
}
