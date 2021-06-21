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
use proc_macro2::Ident;
use quote::*;
use std::str;
use syn::{parse_macro_input, Attribute, DeriveInput, Field, Lit, Meta, NestedMeta, Variant};

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
                #headers
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_headers(d: &syn::Data) -> proc_macro2::TokenStream {
    let headers = match d {
        syn::Data::Struct(st) => get_st_headers(st),
        syn::Data::Enum(e) => get_enum_headers(e).concat(),
        syn::Data::Union(_) => todo!("it's not clear how to handle union type"),
    };

    quote!({
        let v: Vec<Vec<String>> = vec![
            #(#headers,)*
        ];

        v.concat()
    })
}

fn get_st_headers(st: &syn::DataStruct) -> Vec<proc_macro2::TokenStream> {
    get_fields_headers(st.fields.iter())
}

fn get_fields_headers<'a>(
    fields: impl Iterator<Item = &'a Field>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .enumerate()
        .filter(|(_, f)| !is_ignored_field(f))
        .map(|(i, f)| field_headers(f, i))
        .collect()
}

fn field_headers(field: &syn::Field, index: usize) -> proc_macro2::TokenStream {
    if should_be_inlined(&field.attrs) {
        inline_header(&field.ty, &field.attrs)
    } else {
        let header = field_name(field, index);
        quote!(vec![String::from(#header)])
    }
}

fn inline_header(t: &syn::Type, attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let inline_prefix = look_for_inline_prefix(attrs);
    if inline_prefix.is_empty() {
        quote! {
            <#t as Tabled>::headers()
        }
    } else {
        quote! {
            <#t as Tabled>::headers().into_iter()
                .map(|header| format!("{}{}", #inline_prefix, header))
                .collect::<Vec<_>>()
        }
    }
}

fn get_enum_headers(e: &syn::DataEnum) -> Vec<Vec<proc_macro2::TokenStream>> {
    e.variants
        .iter()
        .filter(|v| !is_ignored_variant(v))
        .map(variant_headers)
        .collect::<Vec<_>>()
}

fn variant_headers(variant: &Variant) -> Vec<proc_macro2::TokenStream> {
    if should_be_inlined(&variant.attrs) {
        let mut calls = Vec::new();
        for (index, field) in variant.fields.iter().enumerate() {
            let call = field_headers(field, index);
            calls.push(call);
        }

        calls
    } else {
        let header = variant_name(variant);
        vec![quote!(vec![String::from(#header)])]
    }
}

fn get_fields(d: &syn::Data) -> proc_macro2::TokenStream {
    match d {
        syn::Data::Struct(st) => {
            let fields = get_st_fields(st);
            quote! {
                {
                    let v: Vec<Vec<String>> = vec![
                        #(#fields,)*
                    ];

                    v.concat()
                }
            }
        }
        syn::Data::Enum(e) => get_enum_fields(e),
        syn::Data::Union(_) => todo!("it's not clear how to handle union type"),
    }
}

fn get_st_fields(st: &syn::DataStruct) -> Vec<proc_macro2::TokenStream> {
    let mut v = Vec::new();
    for (i, field) in st.fields.iter().enumerate() {
        if is_ignored_field(field) {
            continue;
        }

        let fields = fields_of_field(field, i);

        v.push(fields);
    }

    v
}

fn fields_of_field(field: &Field, index: usize) -> proc_macro2::TokenStream {
    let field_name = field_field(field, index);
    get_field_fields(field_name, &field.attrs)
}

fn get_field_fields(
    field: proc_macro2::TokenStream,
    attrs: &[syn::Attribute],
) -> proc_macro2::TokenStream {
    let mut field_value = field;
    let is_inline = should_be_inlined(&attrs);
    if is_inline {
        let value = quote! { #field_value.fields() };
        return value;
    }

    let func = check_display_with_func(&attrs);
    if let Some(func) = func {
        field_value = use_function_for(field_value, &func);
    }

    field_value = quote!(vec![format!("{}", #field_value)]);

    field_value
}

fn use_function_for(field: proc_macro2::TokenStream, function: &str) -> proc_macro2::TokenStream {
    let function = syn::Ident::new(function, proc_macro2::Span::call_site());
    quote! { #function(&#field) }
}

fn field_field(field: &Field, index: usize) -> proc_macro2::TokenStream {
    field.ident.as_ref().map_or_else(
        || {
            let mut s = quote!(self.);
            s.extend(syn::Index::from(index).to_token_stream());
            s
        },
        |f| quote!(self.#f),
    )
}

fn get_enum_fields(e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut fields = Vec::new();

    let variants = e.variants.iter().filter(|v| !is_ignored_variant(v));
    for v in variants {
        let variant_fields = variant_fields(v);
        fields.push(variant_fields);
    }

    let branches = e
        .variants
        .iter()
        .filter(|v| !is_ignored_variant(v))
        .map(variant_match_branches)
        .collect::<Vec<_>>();

    assert_eq!(branches.len(), fields.len());

    let headers = get_enum_headers(e)
        .into_iter()
        .map(|headers| {
            quote! {
                vec![
                    #(#headers,)*
                ]
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let mut stream = proc_macro2::TokenStream::new();
    for (i, (branch, fields)) in branches.into_iter().zip(fields).enumerate() {
        let branch = quote! {
            Self::#branch => {
                // It's a bit strange trick but I haven't found any better
                // how to calculate a size and offset
                let headers: Vec<Vec<Vec<String>>> = vec![
                    #(#headers,)*
                ];
                let lengths = headers.iter().map(|values| values.iter().map(|values| values.len()).sum::<usize>()).collect::<Vec<_>>();
                let size = lengths.iter().sum::<usize>();
                let offsets: Vec<usize> = lengths.iter().fold(Vec::new(), |mut acc, len| {
                    // offset of 1 element is 0
                    if acc.is_empty() {
                        acc.push(0);
                    }

                    let privious_len: usize = acc.last().map(|l| *l).unwrap_or(0);
                    acc.push(privious_len + len);
                    acc
                });
                let offset = offsets[#i];

                let mut v: Vec<String> = std::iter::repeat(String::new()).take(size).collect();

                let fields: Vec<Vec<String>> = vec![
                    #(#fields),*
                ];
                let fields = fields.concat();

                for (i, field) in fields.into_iter().enumerate() {
                    v[i+offset] = field;
                }

                v
            },
        };

        stream.append_all(branch);
    }

    quote! {
        #[allow(unused_variables)]
        match &self {
            #stream
            _ => vec![],
        }
    }
}

fn variant_fields(v: &Variant) -> Vec<proc_macro2::TokenStream> {
    if !should_be_inlined(&v.attrs) {
        return vec![quote!(vec!["+".to_string()])];
    }

    let branch_idents = variant_idents(v);
    if branch_idents.is_empty() {
        return vec![quote!(vec!["+".to_string()])];
    }

    branch_idents
        .into_iter()
        .map(|ident| get_field_fields(ident.to_token_stream(), &v.attrs))
        .collect()
}

fn variant_idents(v: &Variant) -> Vec<Ident> {
    v.fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            if let Some(ident) = field.ident.as_ref() {
                ident.clone()
            } else {
                let tmp_var = syn::Ident::new(
                    format!("x_{}", index).as_str(),
                    proc_macro2::Span::call_site(),
                );

                tmp_var
            }
        })
        .collect::<Vec<_>>()
}

fn variant_match_branches(v: &Variant) -> proc_macro2::TokenStream {
    let mut token = proc_macro2::TokenStream::new();
    token.append_all(v.ident.to_token_stream());

    let fields = if should_be_inlined(&v.attrs) {
        let fields = variant_idents(v);
        quote! (#(#fields, )*)
    } else {
        quote!(..)
    };

    match &v.fields {
        syn::Fields::Named(_) => {
            syn::token::Brace::default().surround(&mut token, |s| {
                s.append_all(fields);
            });
        }
        syn::Fields::Unnamed(_) => {
            syn::token::Paren::default().surround(&mut token, |s| {
                s.append_all(fields);
            });
        }
        syn::Fields::Unit => {}
    };

    token
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

fn check_display_with_func(attrs: &[syn::Attribute]) -> Option<String> {
    find_name_attribute(attrs, "field", "display_with", look_up_nested_meta_str)
}

fn variant_name(v: &syn::Variant) -> String {
    find_name_attribute(&v.attrs, "header", "name", look_up_nested_meta_str)
        .or_else(|| find_name_attribute(&v.attrs, "header", "name", look_up_nested_meta_flag_str))
        .unwrap_or_else(|| v.ident.to_string())
}

fn should_be_inlined(attrs: &[syn::Attribute]) -> bool {
    let inline_attr = find_name_attribute(&attrs, "header", "inline", look_up_nested_meta_bool)
        .or_else(|| find_name_attribute(&attrs, "field", "inline", look_up_nested_meta_bool))
        .or_else(|| {
            find_name_attribute(&attrs, "header", "inline", look_up_nested_flag_str_in_attr)
                .map(|_| true)
        });
    inline_attr == Some(true)
}

fn look_for_inline_prefix(attrs: &[syn::Attribute]) -> String {
    find_name_attribute(&attrs, "header", "inline", look_up_nested_flag_str_in_attr)
        .or_else(|| find_name_attribute(&attrs, "field", "inline", look_up_nested_flag_str_in_attr))
        .unwrap_or_else(|| "".to_owned())
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
        let val = parse_name_attribute_nested(meta_list.nested.iter(), method, name, lookup);
        if val.is_some() {
            return val;
        }
    }

    None
}

fn parse_name_attribute_nested<'a, R, F>(
    nested: impl Iterator<Item = &'a NestedMeta>,
    method: &str,
    name: &str,
    lookup: F,
) -> Option<R>
where
    F: Fn(&syn::NestedMeta, &str) -> Result<Option<R>, String>,
{
    for nested_meta in nested {
        let val = lookup(nested_meta, name).unwrap_or_else(
            |e| panic!("{error} macros {macro} field {name}", error=e, macro=method, name=name),
        );

        if val.is_some() {
            return val;
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

fn look_up_nested_flag_str_in_attr(
    meta: &syn::NestedMeta,
    name: &str,
) -> Result<Option<String>, String> {
    match meta {
        NestedMeta::Meta(Meta::List(list)) => {
            parse_name_attribute_nested(list.nested.iter(), "", name, look_up_nested_meta_flag_str)
                .ok_or_else(|| "An attribute doesn't have expected value".to_string())
                .map(Some)
        }
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
