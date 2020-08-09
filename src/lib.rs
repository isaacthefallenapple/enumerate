#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use std::collections::HashMap;
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

    let mut standard_enumerator = Vec::new();
    let mut enumerators = HashMap::<_, Vec<_>>::new();

    let mut started = None;
    for variant in e.variants.iter().map(process_variant) {
        if let Err(e) = variant {
            e.emit();
            return TokenStream::new();
        }
        let variant = variant.unwrap();
        match variant {
            Variant(_, Attr::Skipped) => continue,
            Variant(ident, Attr::Default) => {
                standard_enumerator.push(ident);
            }
            Variant(ident, Attr::Start(enumerator)) => {
                started.replace(enumerator.clone());
                enumerators.entry(enumerator).or_default().push(ident);
            }
            Variant(ident, Attr::None) => {
                started
                    .as_ref()
                    .and_then(|enumerator| enumerators.get_mut(enumerator))
                    .unwrap_or(&mut standard_enumerator)
                    .push(ident);
            }
            Variant(ident, Attr::Single(enumerator)) => {
                enumerators.entry(enumerator).or_default().push(ident);
            }
        }
    }

    let enumerator_names = enumerators
        .keys()
        .map(|ident| format_ident!("enumerate_{}", ident));
    let enumerator_variants = enumerators.values();

    (quote! {
        impl #name {
            fn enumerate() -> impl Iterator<Item = &'static #name> {
                const VARS: &[#name] = &[#( #name::#standard_enumerator ),*];
                VARS.iter()
            }

            #(
                fn #enumerator_names() -> impl Iterator<Item = &'static #name> {
                    const VARS: &[#name] = &[#( #name::#enumerator_variants ),*];
                    VARS.iter()
                }
            )*
        }
    })
    .into()
}

fn process_variant(v: &syn::Variant) -> Result<Variant, proc_macro::Diagnostic> {
    let syn::Variant { ident, attrs, .. } = v;

    let var = match parse_attr(attrs) {
        Err(err) => return Err(err),
        Ok(attr) => Variant(ident.clone(), attr),
    };

    if var.1 != Attr::Skipped && v.fields != syn::Fields::Unit {
        Err(v
            .span()
            .unwrap()
            .error("Cannot enumerate enum with tuple or struct variant.")
            .help("Try skipping the variant with the `#[enumerate(skip)]` attribute."))
    } else {
        Ok(var)
    }
}

struct Variant(syn::Ident, Attr);

#[derive(PartialEq)]
enum Attr {
    None,
    Start(syn::Ident),
    Single(syn::Ident),
    Default,
    Skipped,
}

impl syn::parse::Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let err = input.error(format!("Invalid argument: `{}`.", input));
        let ident: syn::Ident = input.parse()?;

        if input.is_empty() {
            let attr = if ident == "skip" {
                Attr::Skipped
            } else if ident == "default" {
                Attr::Default
            } else {
                Attr::Single(ident)
            };
            return Ok(attr);
        } else if ident == "start" {
            let _: syn::Token![=] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            if input.is_empty() {
                return Ok(Attr::Start(ident));
            }
        }
        Err(err)
    }
}

fn parse_attr(attrs: &[syn::Attribute]) -> Result<Attr, proc_macro::Diagnostic> {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "enumerate" {
            let mut stream = attr.tokens.clone().into_iter();
            if let Some(TokenTree::Group(group)) = stream.next() {
                return syn::parse2(group.stream()).map_err(|err| {
                    group
                        .stream()
                        .span()
                        .unwrap()
                        .error(err.to_string())
                        .note("Expected `skip`, valid identifier, or `start = <ident>`.")
                });
            }
        }
    }
    Ok(Attr::None)
}
