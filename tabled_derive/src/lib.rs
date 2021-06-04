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
use syn::{
    parse_macro_input, Attribute, DeriveInput, Field, Lit,
    Meta, NestedMeta,
};

#[proc_macro_derive(Tabled, attributes(header))]
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
        .filter(|(_, f)| {
            let is_ignored = find_bool_attribute(&f.attrs, "header", "hidden");
            is_ignored != Some(true)
        })
        .map(|(i, f)| {
            let override_name = find_name_attribute(&f.attrs, "header", "name");
            match override_name {
                Some(name) => name,
                None => f
                    .ident
                    .as_ref()
                    .map_or_else(|| format!("{}", i), |f| f.to_string()),
            }
        })
        .collect()
}

fn get_enum_headers(e: &syn::DataEnum) -> Vec<String> {
    e.variants
        .iter()
        .filter(|v| {
            let is_ignored = find_bool_attribute(&v.attrs, "header", "hidden");
            is_ignored != Some(true)
        })
        .map(|v| {
            let override_name = find_name_attribute(&v.attrs, "header", "name");
            match override_name {
                Some(name) => vec![name],
                None => {
                    let variant = v.ident.to_string();
                    vec![format!("{}", variant)]
                }
            }
        })
        .collect::<Vec<Vec<_>>>()
        .concat()
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
    st.fields
        .iter()
        .filter(|f| {
            let is_ignored = find_bool_attribute(&f.attrs, "header", "hidden");
            is_ignored != Some(true)
        })
        .map(|f| f.ident.as_ref())
        .enumerate()
        .map(|(i, f)| {
            f.map_or_else(
                || {
                    let mut s = quote!(self.);
                    s.extend(syn::Index::from(i).to_token_stream());
                    s
                },
                |f| quote!(self.#f),
            )
        })
        .collect()
}

fn get_enum_fields(e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut fields_per_variant = Vec::new();
    let mut variant_field_shift = Vec::new();
    let mut variant_fields_len = Vec::new();
    let mut count_fields = 0;
    let variants = e.variants.iter().filter(|v| {
        let is_ignored = find_bool_attribute(&v.attrs, "header", "hidden");
        is_ignored != Some(true)
    });
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
        .filter(|v| {
            let is_ignored = find_bool_attribute(&v.attrs, "header", "hidden");
            is_ignored != Some(true)
        })
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

fn parse_name_attribute(attr: &Attribute, method: &str, name: &str) -> Option<String> {
    if attr.path.is_ident(method) {
        let meta = &attr.parse_meta();
        match meta {
            Ok(Meta::List(meta_list)) => {
                if meta_list.path.is_ident(method) {
                    for nested_meta in &meta_list.nested {
                        match nested_meta {
                            NestedMeta::Lit(Lit::Str(value)) => return Some(value.value()),
                            NestedMeta::Lit(Lit::ByteStr(value)) => return Some(
                                std::str::from_utf8(&value.value())
                                    .expect(&format!("Expected a valid UTF-8 string for a macro {macro} field {name}", macro=method, name=name))
                                    .to_owned(),
                            ),
                            NestedMeta::Meta(Meta::NameValue(value)) => {
                                if value.path.is_ident(name) {
                                    match &value.lit {
                                        Lit::Str(value) => return Some(value.value()),
                                        Lit::ByteStr(value) => return Some(
                                            std::str::from_utf8(&value.value())
                                                .expect(&format!("Expected a valid UTF-8 string for a macro {macro} field {name}", macro=method, name=name))
                                                .to_owned(),
                                        ),
                                        _ => panic!("Parameter {name} for macro {macro} should be String", name=name, macro=method)

                                    }
                                }
                            }
                            _ => {
                                
                            }
                        }
                    }
                }

                None
            }
            _ => None,
        }
    } else {
        None
    }
}

fn find_name_attribute(attributes: &[Attribute], method: &str, name: &str) -> Option<String> {
    attributes
        .iter()
        .find_map(|attr| parse_name_attribute(attr, method, name))
}

fn parse_bool_attribute(attr: &Attribute, method: &str, name: &str) -> Option<bool> {
    if attr.path.is_ident(method) {
        let meta = &attr.parse_meta();
        match meta {
            Ok(Meta::List(meta_list)) => {
                for nested_meta in &meta_list.nested {
                    match nested_meta {
                        NestedMeta::Meta(Meta::Path(path)) => if path.is_ident(name) { return Some(true) } else { return None },
                        NestedMeta::Meta(Meta::NameValue(value)) => {
                            if value.path.is_ident(name) {
                                match &value.lit {
                                    Lit::Bool(value) => return Some(value.value()),
                                    Lit::Verbatim(literal) => panic!("{:?}", literal),
                                    _ => panic!("Parameter {name} for macro {macro} should be a bool value", name=name, macro=method)
                                }
                            }
                        }
                        _ => {
                            
                        }
                    }
                }

                None
            }
            _ => None,
        }
    } else {
        None
    }
}

fn find_bool_attribute(attributes: &[Attribute], method: &str, name: &str) -> Option<bool> {
    attributes
        .iter()
        .find_map(|attr| parse_bool_attribute(attr, method, name))
}
