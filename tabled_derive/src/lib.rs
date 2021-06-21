// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use std::str;
use syn::{parse_macro_input, Attribute, DeriveInput, Field, Lit, Meta, NestedMeta};

#[proc_macro_derive(Tabled, attributes(header, field))]
pub fn tabled(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_tabled(&input)
}

fn impl_tabled(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let headers = get_headers(&ast.data);
    let fields = get_fields(&ast.data);

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Tabled for #name #ty_generics #where_clause {
            fn fields(&self) -> Vec<String> {
                #fields
            }

            fn headers() -> Vec<String> {
                vec![#(String::from(#headers),)*]
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_headers(d: &syn::Data) -> Vec<String> {
    match d {
        syn::Data::Struct(st) => get_st_headers(st),
        syn::Data::Enum(e) => get_enum_headers(e),
        syn::Data::Union(_) => todo!("it's not clear how to handle union type"),
    }
}

fn get_st_headers(st: &syn::DataStruct) -> Vec<String> {
    get_fields_headers(st.fields.iter())
}

fn get_fields_headers<'a>(fields: impl Iterator<Item = &'a Field>) -> Vec<String> {
    fields
        .enumerate()
        .filter(|(_, f)| !is_ignored_field(f))
        .map(|(i, f)| field_name(f, i))
        .collect()
}

fn get_enum_headers(e: &syn::DataEnum) -> Vec<String> {
    e.variants
        .iter()
        .filter(|v| !is_ignored_variant(v))
        .map(variant_name)
        .collect()
}

fn get_fields(d: &syn::Data) -> proc_macro2::TokenStream {
    match d {
        syn::Data::Struct(st) => {
            let fields = get_st_fields(st);
            quote! { vec![#(format!("{}", #fields),)*] }
        }
        syn::Data::Enum(e) => get_enum_fields(e),
        syn::Data::Union(_) => todo!(),
    }
}

fn get_st_fields(st: &syn::DataStruct) -> Vec<proc_macro2::TokenStream> {
    let mut v = Vec::new();
    for (i, field) in st.fields.iter().enumerate() {
        let is_ignored =
            find_name_attribute(&field.attrs, "header", "hidden", look_up_nested_meta_bool);
        if is_ignored == Some(true) {
            continue;
        }

        let mut value = field.ident.as_ref().map_or_else(
            || {
                let mut s = quote!(self.);
                s.extend(syn::Index::from(i).to_token_stream());
                s
            },
            |f| quote!(self.#f),
        );

        let with_function = find_name_attribute(
            &field.attrs,
            "field",
            "display_with",
            look_up_nested_meta_str,
        );
        if let Some(function) = with_function {
            let function = syn::Ident::new(&function, proc_macro2::Span::call_site());
            value = quote! { #function(&#value) };
        }

        v.push(value);
    }

    v
}

fn get_enum_fields(e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut fields_per_variant = Vec::new();
    let mut variant_field_shift = Vec::new();
    let mut variant_fields_len = Vec::new();
    let mut count_fields = 0;
    let variants = e.variants.iter().filter(|v| !is_ignored_variant(v));
    for _ in variants {
        let fields = vec![quote! { "+".to_string() }];

        variant_field_shift.push(count_fields);
        variant_fields_len.push(fields.len());
        count_fields += fields.len();
        fields_per_variant.push(fields);
    }

    let variants = e
        .variants
        .iter()
        .filter(|v| !is_ignored_variant(v))
        .map(|v| {
            let mut token = proc_macro2::TokenStream::new();
            token.append_all(v.ident.to_token_stream());

            match &v.fields {
                syn::Fields::Named(fields) => {
                    let parameters = fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref())
                        .flatten()
                        .map(|f| {
                            quote! { #f,}
                        })
                        .collect::<Vec<_>>();

                    syn::token::Brace::default().surround(&mut token, |s| {
                        s.append_all(parameters);
                    });
                }
                syn::Fields::Unnamed(_) => {
                    // TODO: "a tuple based struct doesn't implemented; here supposed to be a generated Ident for a tuple"
                    syn::token::Paren::default().surround(&mut token, |s| {
                        s.append_all(quote! {_});
                    });
                }
                syn::Fields::Unit => {}
            };

            token
        })
        .collect::<Vec<_>>();

    quote! {
        let size = #count_fields;
        let mut v: Vec<String> = std::iter::repeat(String::new()).take(size).collect();
        #[allow(unused_variables)]
        match &self {
            #(Self::#variants => {
                let fields = vec![#(#fields_per_variant.to_string()),*];

                for i in #variant_field_shift..#variant_field_shift+#variant_fields_len {
                    v[i] = fields[i-#variant_field_shift].clone();
                }

                v
            },)*
            _ => vec![],
        }
    }
}

fn field_name(f: &syn::Field, index: usize) -> String {
    let override_name = find_name_attribute(&f.attrs, "header", "name", look_up_nested_meta_str)
        .or_else(|| find_name_attribute(&f.attrs, "header", "name", look_up_nested_meta_flag_str));
    match override_name {
        Some(name) => name,
        None => match f.ident.as_ref() {
            Some(name) => name.to_string(),
            None => format!("{}", index),
        },
    }
}

fn variant_name(v: &syn::Variant) -> String {
    find_name_attribute(&v.attrs, "header", "name", look_up_nested_meta_str)
        .or_else(|| find_name_attribute(&v.attrs, "header", "name", look_up_nested_meta_flag_str))
        .unwrap_or_else(|| v.ident.to_string())
}

fn is_ignored_field(f: &syn::Field) -> bool {
    attrs_has_ignore_sign(&f.attrs)
}

fn is_ignored_variant(f: &syn::Variant) -> bool {
    attrs_has_ignore_sign(&f.attrs)
}

fn attrs_has_ignore_sign(attrs: &[syn::Attribute]) -> bool {
    let is_ignored = find_name_attribute(&attrs, "header", "hidden", look_up_nested_meta_bool);
    is_ignored == Some(true)
}

fn parse_name_attribute<R, F>(attr: &Attribute, method: &str, name: &str, lookup: F) -> Option<R>
where
    F: Fn(&syn::NestedMeta, &str) -> Result<Option<R>, String>,
{
    if !attr.path.is_ident(method) {
        return None;
    }

    let meta = attr.parse_meta().ok()?;

    if let Meta::List(meta_list) = meta {
        for nested_meta in &meta_list.nested {
            let val = lookup(nested_meta, name).unwrap_or_else(
                |e| panic!("{error} macros {macro} field {name}", error=e, macro=method, name=name),
            );

            if val.is_some() {
                return val;
            }
        }
    }

    None
}

fn look_up_nested_meta_str(meta: &syn::NestedMeta, name: &str) -> Result<Option<String>, String> {
    match meta {
        NestedMeta::Meta(Meta::NameValue(value)) => {
            if value.path.is_ident(name) {
                check_str_literal(&value.lit)
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

fn look_up_nested_meta_flag_str(meta: &syn::NestedMeta, _: &str) -> Result<Option<String>, String> {
    match meta {
        NestedMeta::Lit(lit) => check_str_literal(lit),
        _ => Ok(None),
    }
}

fn check_str_literal(lit: &syn::Lit) -> Result<Option<String>, String> {
    match lit {
        Lit::Str(value) => Ok(Some(value.value())),
        Lit::ByteStr(value) => str::from_utf8(&value.value())
            .map(|s| s.to_owned())
            .map(Some)
            .map_err(|_| "Expected a valid UTF-8 string for a field".to_owned()),
        _ => Ok(None),
    }
}

fn look_up_nested_meta_bool(meta: &syn::NestedMeta, name: &str) -> Result<Option<bool>, String> {
    match meta {
        NestedMeta::Meta(Meta::Path(path)) if path.is_ident(name) => Ok(Some(true)),
        NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident(name) => match &value.lit {
            Lit::Bool(value) => Ok(Some(value.value())),
            _ => Err("A parameter should be a bool value".to_string()),
        },
        _ => Ok(None),
    }
}

fn find_name_attribute<R, F>(
    attributes: &[Attribute],
    method: &str,
    name: &str,
    lookup: F,
) -> Option<R>
where
    F: Fn(&syn::NestedMeta, &str) -> Result<Option<R>, String> + Clone,
{
    attributes
        .iter()
        .find_map(|attr| parse_name_attribute(attr, method, name, lookup.clone()))
}
