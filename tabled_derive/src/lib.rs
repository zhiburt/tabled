extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::*;
use std::str;
use syn::{
    parse_macro_input, token, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields,
    Ident, Index, Lit, Meta, NestedMeta, Type, Variant,
};

#[proc_macro_derive(Tabled, attributes(header, field))]
pub fn tabled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_tabled(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_tabled(ast: &DeriveInput) -> TokenStream {
    let info = collect_info(ast).unwrap();
    let fields = info.values;
    let headers = info.headers;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics Tabled for #name #ty_generics #where_clause {
            fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
                #fields
            }

            fn headers() -> Vec<std::borrow::Cow<'static, str>> {
                #headers
            }
        }
    };

    expanded
}

fn collect_info(ast: &DeriveInput) -> Result<Impl, String> {
    match &ast.data {
        Data::Struct(data) => collect_info_struct(data),
        Data::Enum(data) => collect_info_enum(data),
        Data::Union(_) => Err("Union type isn't supported".to_owned()),
    }
}

fn collect_info_struct(ast: &DataStruct) -> Result<Impl, String> {
    info_from_fields(&ast.fields, field_var_name, "")
}

// todo: refactoring. instead of using a lambda + prefix
// we could just not emit `self.` `_x` inside
// So the calle would prefix it on its own
fn info_from_fields(
    fields: &Fields,
    field_name: impl Fn(usize, &Field) -> TokenStream,
    header_prefix: &str,
) -> Result<Impl, String> {
    let fields = fields.into_iter().enumerate().map(|(i, field)| {
        let attributes = Attributes::parse(&field.attrs);
        (i, field, attributes)
    });

    let mut headers = Vec::new();
    let mut values = Vec::new();

    for (i, field, attributes) in fields {
        if attributes.is_ignored() {
            continue;
        }

        let header = field_headers(field, i, &attributes, header_prefix);

        headers.push(header);

        let field_name = field_name(i, field);
        let value = get_field_fields(field_name, &attributes);

        values.push(value);
    }

    let headers = quote!({
        let mut out = Vec::new();
        #(out.extend(#headers);)*
        out
    });

    let values = quote!({
        let mut out = Vec::new();
        #(out.extend(#values);)*
        out
    });

    Ok(Impl { headers, values })
}

fn field_headers(
    field: &Field,
    index: usize,
    attributes: &Attributes,
    prefix: &str,
) -> TokenStream {
    if attributes.inline {
        return get_type_headers(&field.ty, &attributes.inline_prefix, "");
    }

    let header_name = field_header_name(field, attributes, index);
    if !prefix.is_empty() {
        quote!(vec![format!("{}{}", #prefix, #header_name).into()])
    } else {
        quote!(vec![String::from(#header_name).into()])
    }
}

fn collect_info_enum(ast: &DataEnum) -> Result<Impl, String> {
    let mut variants = Vec::new();
    for variant in &ast.variants {
        let attributes = Attributes::parse(&variant.attrs);
        if attributes.is_ignored() {
            continue;
        }

        let info = info_from_variant(variant, &attributes)?;
        variants.push((variant, info));
    }

    let headers_list = variants.iter().map(|(_, i)| &i.headers).collect::<Vec<_>>();
    let headers = quote! {
        vec![
            #(#headers_list,)*
        ]
        .concat()
    };

    let values = values_for_enum(variants);

    Ok(Impl { headers, values })
}

fn info_from_variant(variant: &Variant, attributes: &Attributes) -> Result<Impl, String> {
    if attributes.inline {
        return info_from_fields(&variant.fields, variant_var_name, &attributes.inline_prefix);
    }

    let variant_name = variant_name(variant, attributes);
    let value = "+";

    // we need exactly string because of it must be inlined as string
    let headers = quote! {vec![#variant_name.into()]};
    // we need exactly string because of it must be inlined as string
    let values = quote! {vec![#value.into()]};

    Ok(Impl { headers, values })
}

struct Impl {
    headers: TokenStream,
    values: TokenStream,
}

fn get_type_headers(field_type: &Type, inline_prefix: &str, prefix: &str) -> TokenStream {
    if prefix.is_empty() && inline_prefix.is_empty() {
        quote! { <#field_type as Tabled>::headers() }
    } else {
        quote! {
            <#field_type as Tabled>::headers().into_iter()
                .map(|header| format!("{}{}{}", #prefix, #inline_prefix, header).into())
                .collect::<Vec<_>>()
        }
    }
}

fn get_field_fields(field: TokenStream, attr: &Attributes) -> TokenStream {
    if attr.inline {
        return quote! { #field.fields() };
    }

    if let Some(func) = &attr.display_with {
        let func_call = use_function_for(field, func);
        return quote!(vec![#func_call.into()]);
    }

    quote!(vec![format!("{}", #field).into()])
}

fn use_function_for(field: TokenStream, function: &str) -> TokenStream {
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

fn field_var_name(index: usize, field: &Field) -> TokenStream {
    let f = field.ident.as_ref().map_or_else(
        || Index::from(index).to_token_stream(),
        |i| i.to_token_stream(),
    );
    quote!(self.#f)
}

fn variant_var_name(index: usize, field: &Field) -> TokenStream {
    match &field.ident {
        Some(indent) => indent.to_token_stream(),
        None => Ident::new(
            format!("x_{}", index).as_str(),
            proc_macro2::Span::call_site(),
        )
        .to_token_stream(),
    }
}

fn values_for_enum(variants: Vec<(&Variant, Impl)>) -> TokenStream {
    let branches = variants.iter().map(|(variant, _)| match_variant(variant));

    let headers = variants
        .iter()
        .map(|(_, info)| &info.headers)
        .collect::<Vec<_>>();

    let fields = variants
        .iter()
        .map(|(_, info)| &info.values)
        .collect::<Vec<_>>();

    let mut stream = TokenStream::new();
    for (i, (branch, fields)) in branches.into_iter().zip(fields).enumerate() {
        let branch = quote! {
            Self::#branch => {
                let offset = offsets[#i];
                let fields: Vec<std::borrow::Cow<str>> = #fields;

                for (i, field) in fields.into_iter().enumerate() {
                    out_vec[i+offset] = field;
                }
            },
        };

        stream.append_all(branch);
    }

    quote! {
        // To be able to insert variant fields in proper places we do this MAGIC with offset.
        //
        // We check headers output as it's static and has an information
        // about length of each field header if it was inlined.
        //
        // It's a bit strange trick but I haven't found any better
        // how to calculate a size and offset.
        let headers: Vec<Vec<std::borrow::Cow<_>>> = vec![#(#headers,)*];
        let lengths = headers.iter().map(|values| values.len()).collect::<Vec<_>>();
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

        let mut out_vec: Vec<std::borrow::Cow<'_, str>> = std::iter::repeat("".into()).take(size).collect();

        #[allow(unused_variables)]
        match &self {
            #stream
            _ => return vec![], // variant is hidden so we return an empty vector
        };

        out_vec
    }
}

fn variant_idents(v: &Variant) -> Vec<TokenStream> {
    v.fields
        .iter()
        .enumerate()
        // we intentionally not ignore these fields to be able to build a pattern correctly
        // .filter(|(_, field)| !Attr::parse(&field.attrs).is_ignored())
        .map(|(index, field)| variant_var_name(index, field))
        .collect::<Vec<_>>()
}

fn match_variant(v: &Variant) -> TokenStream {
    let mut token = TokenStream::new();
    token.append_all(v.ident.to_token_stream());

    let fields = variant_idents(v);

    match &v.fields {
        Fields::Named(_) => {
            token::Brace::default().surround(&mut token, |s| {
                s.append_separated(fields, quote! {,});
            });
        }
        Fields::Unnamed(_) => {
            token::Paren::default().surround(&mut token, |s| {
                s.append_separated(fields, quote! {,});
            });
        }
        Fields::Unit => {}
    };

    token
}

fn variant_name(variant: &Variant, attributes: &Attributes) -> String {
    attributes
        .name
        .clone()
        .unwrap_or_else(|| variant.ident.to_string())
}

fn field_header_name(f: &Field, attr: &Attributes, index: usize) -> String {
    match &attr.name {
        Some(name) => name.to_string(),
        None => match f.ident.as_ref() {
            Some(name) => name.to_string(),
            None => format!("{}", index),
        },
    }
}

// It would be cool to create a library for a parsing attributes
#[derive(Debug)]
struct Attributes {
    hidden: bool,
    inline: bool,
    inline_prefix: String,
    name: Option<String>,
    display_with: Option<String>,
}

impl Attributes {
    fn parse(attrs: &[Attribute]) -> Self {
        let is_ignored = attrs_has_ignore_sign(attrs);
        let should_be_inlined = should_be_inlined(attrs);
        let inline_prefix = look_for_inline_prefix(attrs);
        let display_with = check_display_with_func(attrs);
        let override_header_name = override_header_name(attrs);

        Self {
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
