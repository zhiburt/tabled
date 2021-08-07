extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use std::str;
use syn::{
    parse_macro_input, token, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields,
    Ident, Index, Lit, Meta, NestedMeta, Type, Variant,
};

#[proc_macro_derive(Tabled, attributes(header, field))]
pub fn tabled(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_tabled(&input)
}

fn impl_tabled(ast: &DeriveInput) -> TokenStream {
    let attributes = data_attributes(&ast.data);
    let headers = get_headers(&ast.data, &attributes);
    let fields = get_fields(&ast.data, &attributes);

    let name = &ast.ident;
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

fn get_headers(d: &Data, attrs: &[Attr]) -> proc_macro2::TokenStream {
    let headers = match d {
        Data::Struct(st) => get_st_headers(st, attrs),
        Data::Enum(e) => get_enum_headers(e, attrs).concat(),
        Data::Union(_) => todo!("it's not clear how to handle union type"),
    };

    quote!({
        let v: Vec<Vec<String>> = vec![
            #(#headers,)*
        ];

        v.concat()
    })
}

fn data_attributes(d: &Data) -> Vec<Attr> {
    let attrs_of_fields: Vec<&[Attribute]> = match d {
        Data::Struct(st) => st.fields.iter().map(|f| f.attrs.as_slice()).collect(),
        Data::Enum(e) => e.variants.iter().map(|v| v.attrs.as_slice()).collect(),
        Data::Union(_) => todo!("it's not clear how to handle union type"),
    };

    attrs_of_fields
        .into_iter()
        .map(|attrs| Attr::parse(attrs))
        .collect()
}

fn get_st_headers(st: &DataStruct, attrs: &[Attr]) -> Vec<proc_macro2::TokenStream> {
    st.fields
        .iter()
        .enumerate()
        .filter(|(i, _)| !attrs[*i].is_ignored())
        .map(|(i, f)| field_headers(f, &attrs[i], i, ""))
        .collect()
}

fn field_headers(
    field: &Field,
    attr: &Attr,
    index: usize,
    prefix: &str,
) -> proc_macro2::TokenStream {
    if attr.inline {
        inline_header(&field.ty, attr, prefix)
    } else {
        let header = field_header_name(field, attr, index);
        if !prefix.is_empty() {
            quote!(vec![format!("{}{}", #prefix, #header)])
        } else {
            quote!(vec![String::from(#header)])
        }
    }
}

fn inline_header(t: &Type, attr: &Attr, prefix: &str) -> proc_macro2::TokenStream {
    let inline_prefix = &attr.inline_prefix;
    if inline_prefix.is_empty() && prefix.is_empty() {
        quote! {
            <#t as Tabled>::headers()
        }
    } else {
        quote! {
            <#t as Tabled>::headers().into_iter()
                .map(|header| format!("{}{}{}", #prefix, #inline_prefix, header))
                .collect::<Vec<_>>()
        }
    }
}

fn get_enum_headers(e: &DataEnum, attrs: &[Attr]) -> Vec<Vec<proc_macro2::TokenStream>> {
    e.variants
        .iter()
        .enumerate()
        .filter(|(i, _)| !attrs[*i].is_ignored())
        .map(|(i, v)| variant_headers(v, &attrs[i]))
        .collect::<Vec<_>>()
}

fn variant_headers(variant: &Variant, attr: &Attr) -> Vec<proc_macro2::TokenStream> {
    if attr.inline {
        let prefix = &attr.inline_prefix;

        let mut calls = Vec::new();
        for (index, field) in variant.fields.iter().enumerate() {
            let field_attr = Attr::parse(&field.attrs);
            let call = field_headers(field, &field_attr, index, prefix);
            calls.push(call);
        }

        calls
    } else {
        let header = attr
            .name
            .clone()
            .unwrap_or_else(|| variant.ident.to_string());
        vec![quote!(vec![String::from(#header)])]
    }
}

fn get_fields(d: &Data, attrs: &[Attr]) -> proc_macro2::TokenStream {
    match d {
        Data::Struct(st) => {
            let fields = get_st_fields(st, attrs);
            quote! {
                {
                    let v: Vec<Vec<String>> = vec![
                        #(#fields,)*
                    ];

                    v.concat()
                }
            }
        }
        Data::Enum(e) => get_enum_fields(e, attrs),
        Data::Union(_) => todo!("it's not clear how to handle union type"),
    }
}

fn get_st_fields(st: &DataStruct, attrs: &[Attr]) -> Vec<proc_macro2::TokenStream> {
    let mut v = Vec::new();
    for (i, field) in st.fields.iter().enumerate() {
        if attrs[i].is_ignored() {
            continue;
        }

        let field_var = field_var_name(field, i);
        let fields = get_field_fields(field_var, &attrs[i]);

        v.push(fields);
    }

    v
}

fn get_field_fields(field: proc_macro2::TokenStream, attr: &Attr) -> proc_macro2::TokenStream {
    if attr.inline {
        return quote! { #field.fields() };
    }

    if let Some(func) = &attr.display_with {
        let func_call = use_function_for(field, func);
        return quote!(vec![#func_call]);
    }

    quote!(vec![format!("{}", #field)])
}

fn use_function_for(field: proc_macro2::TokenStream, function: &str) -> proc_macro2::TokenStream {
    let path: syn::Result<syn::ExprPath> = syn::parse_str(function);
    match path {
        Ok(path) => {
            quote! { #path(&#field) }
        }
        _ => {
            let function = Ident::new(function, proc_macro2::Span::call_site());
            quote! { #function(&#field) }
        }
    }
}

fn field_var_name(field: &Field, index: usize) -> proc_macro2::TokenStream {
    field.ident.as_ref().map_or_else(
        || {
            let mut s = quote!(self.);
            s.extend(Index::from(index).to_token_stream());
            s
        },
        |f| quote!(self.#f),
    )
}

fn get_enum_fields(e: &DataEnum, attrs: &[Attr]) -> proc_macro2::TokenStream {
    let fields = e
        .variants
        .iter()
        .enumerate()
        .filter(|(i, _)| !attrs[*i].is_ignored())
        .map(|(i, v)| variant_fields(v, &attrs[i]))
        .collect::<Vec<_>>();

    let branches = e
        .variants
        .iter()
        .enumerate()
        .filter(|(i, _)| !attrs[*i].is_ignored())
        .map(|(i, v)| variant_match_branches(v, &attrs[i]))
        .collect::<Vec<_>>();

    assert_eq!(branches.len(), fields.len());

    let headers = get_enum_headers(e, attrs)
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

fn variant_fields(v: &Variant, attr: &Attr) -> Vec<proc_macro2::TokenStream> {
    if !attr.inline {
        return vec![quote!(vec!["+".to_string()])];
    }

    let branch_idents = variant_idents(v);
    if branch_idents.is_empty() {
        return vec![quote!(vec!["+".to_string()])];
    }

    branch_idents
        .into_iter()
        .zip(v.fields.iter().map(|field| Attr::parse(&field.attrs)))
        .map(|(ident, attr)| get_field_fields(ident.to_token_stream(), &attr))
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
                let tmp_var = Ident::new(
                    format!("x_{}", index).as_str(),
                    proc_macro2::Span::call_site(),
                );

                tmp_var
            }
        })
        .collect::<Vec<_>>()
}

fn variant_match_branches(v: &Variant, attr: &Attr) -> proc_macro2::TokenStream {
    let mut token = proc_macro2::TokenStream::new();
    token.append_all(v.ident.to_token_stream());

    let fields = if attr.inline {
        let fields = variant_idents(v);
        quote! (#(#fields, )*)
    } else {
        quote!(..)
    };

    match &v.fields {
        Fields::Named(_) => {
            token::Brace::default().surround(&mut token, |s| {
                s.append_all(fields);
            });
        }
        Fields::Unnamed(_) => {
            token::Paren::default().surround(&mut token, |s| {
                s.append_all(fields);
            });
        }
        Fields::Unit => {}
    };

    token
}

fn field_header_name(f: &Field, attr: &Attr, index: usize) -> String {
    match &attr.name {
        Some(name) => name.to_string(),
        None => match f.ident.as_ref() {
            Some(name) => name.to_string(),
            None => format!("{}", index),
        },
    }
}

fn override_header_name(attrs: &[Attribute]) -> Option<String> {
    find_name_attribute(attrs, "header", "name", look_up_nested_meta_str)
        .or_else(|| find_name_attribute(attrs, "header", "name", look_up_nested_meta_flag_str))
}

fn check_display_with_func(attrs: &[Attribute]) -> Option<String> {
    find_name_attribute(attrs, "field", "display_with", look_up_nested_meta_str)
}

fn should_be_inlined(attrs: &[Attribute]) -> bool {
    let inline_attr = find_name_attribute(attrs, "header", "inline", look_up_nested_meta_bool)
        .or_else(|| find_name_attribute(attrs, "field", "inline", look_up_nested_meta_bool))
        .or_else(|| {
            find_name_attribute(attrs, "header", "inline", look_up_nested_flag_str_in_attr)
                .map(|_| true)
        })
        .or_else(|| {
            find_name_attribute(attrs, "field", "inline", look_up_nested_flag_str_in_attr)
                .map(|_| true)
        });
    inline_attr == Some(true)
}

fn look_for_inline_prefix(attrs: &[Attribute]) -> String {
    find_name_attribute(attrs, "header", "inline", look_up_nested_flag_str_in_attr)
        .or_else(|| find_name_attribute(attrs, "field", "inline", look_up_nested_flag_str_in_attr))
        .unwrap_or_else(|| "".to_owned())
}

fn attrs_has_ignore_sign(attrs: &[Attribute]) -> bool {
    let is_ignored = find_name_attribute(attrs, "header", "hidden", look_up_nested_meta_bool);
    is_ignored == Some(true)
}

fn parse_name_attribute<R, F>(attr: &Attribute, method: &str, name: &str, lookup: F) -> Option<R>
where
    F: Fn(&NestedMeta, &str) -> Result<Option<R>, String>,
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
    F: Fn(&NestedMeta, &str) -> Result<Option<R>, String>,
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

fn look_up_nested_meta_str(meta: &NestedMeta, name: &str) -> Result<Option<String>, String> {
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

fn look_up_nested_meta_flag_str(meta: &NestedMeta, _: &str) -> Result<Option<String>, String> {
    match meta {
        NestedMeta::Lit(lit) => check_str_literal(lit),
        _ => Ok(None),
    }
}

fn check_str_literal(lit: &Lit) -> Result<Option<String>, String> {
    match lit {
        Lit::Str(value) => Ok(Some(value.value())),
        Lit::ByteStr(value) => str::from_utf8(&value.value())
            .map(|s| s.to_owned())
            .map(Some)
            .map_err(|_| "Expected a valid UTF-8 string for a field".to_owned()),
        _ => Ok(None),
    }
}

fn look_up_nested_meta_bool(meta: &NestedMeta, name: &str) -> Result<Option<bool>, String> {
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
    meta: &NestedMeta,
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
    F: Fn(&NestedMeta, &str) -> Result<Option<R>, String> + Clone,
{
    attributes
        .iter()
        .find_map(|attr| parse_name_attribute(attr, method, name, lookup.clone()))
}

// It would be cool to create a library for a parsing attributes
#[derive(Debug)]
struct Attr {
    hidden: bool,
    inline: bool,
    inline_prefix: String,
    name: Option<String>,
    display_with: Option<String>,
}

impl Attr {
    fn parse(attrs: &[Attribute]) -> Self {
        let is_ignored = attrs_has_ignore_sign(attrs);
        let should_be_inlined = should_be_inlined(attrs);
        let inline_prefix = look_for_inline_prefix(attrs);
        let display_with = check_display_with_func(attrs);
        let override_header_name = override_header_name(attrs);

        Attr {
            display_with,
            hidden: is_ignored,
            inline: should_be_inlined,
            inline_prefix,
            name: override_header_name,
        }
    }

    fn is_ignored(&self) -> bool {
        self.hidden
    }
}
