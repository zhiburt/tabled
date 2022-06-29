extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::*;
use std::{collections::HashMap, str};
use syn::{
    parse_macro_input, token, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields,
    Ident, Index, Lit, Meta, MetaNameValue, NestedMeta, Type, Variant,
};

#[proc_macro_derive(Tabled, attributes(tabled))]
pub fn tabled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_tabled(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_tabled(ast: &DeriveInput) -> TokenStream {
    let attrs = ObjectAttributes::parse(&ast.attrs);

    let length = get_tabled_length(ast).unwrap();
    let info = collect_info(ast, attrs).unwrap();
    let fields = info.values;
    let headers = info.headers;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics Tabled for #name #ty_generics #where_clause {
            const LENGTH: usize = #length;

            fn fields(&self) -> Vec<String> {
                #fields
            }

            fn headers() -> Vec<String> {
                #headers
            }
        }
    };

    expanded
}

fn get_tabled_length(ast: &DeriveInput) -> Result<TokenStream, String> {
    match &ast.data {
        Data::Struct(data) => Ok(get_fields_length(&data.fields)),
        Data::Enum(data) => Ok(get_enum_length(data)),
        Data::Union(_) => Err("Union type isn't supported".to_owned()),
    }
}

fn get_fields_length(fields: &Fields) -> TokenStream {
    let size_components = fields
        .iter()
        .map(|field| {
            let attributes = Attributes::parse(&field.attrs);
            (field, attributes)
        })
        .filter(|(_, attr)| !attr.is_ignored())
        .map(|(field, attr)| {
            if !attr.inline {
                quote!({ 1 })
            } else {
                let field_type = &field.ty;
                quote!({<#field_type as Tabled>::LENGTH})
            }
        });

    let size_components = std::iter::once(quote!(0)).chain(size_components);

    let mut stream = TokenStream::new();
    stream.append_separated(size_components, syn::token::Add::default());

    stream
}

fn get_enum_length(enum_ast: &DataEnum) -> TokenStream {
    let variant_sizes = get_enum_variant_length(enum_ast);
    let variant_sizes = std::iter::once(quote!(0)).chain(variant_sizes);

    let mut stream = TokenStream::new();
    stream.append_separated(variant_sizes, syn::token::Add::default());

    stream
}

fn get_enum_variant_length(enum_ast: &DataEnum) -> impl Iterator<Item = TokenStream> + '_ {
    enum_ast
        .variants
        .iter()
        .map(|variant| {
            let attributes = Attributes::parse(&variant.attrs);
            (variant, attributes)
        })
        .filter(|(_, attr)| !attr.is_ignored())
        .map(|(variant, attr)| {
            if !attr.inline {
                quote!(1)
            } else {
                get_fields_length(&variant.fields)
            }
        })
}

fn collect_info(ast: &DeriveInput, attrs: ObjectAttributes) -> Result<Impl, String> {
    match &ast.data {
        Data::Struct(data) => collect_info_struct(data, attrs),
        Data::Enum(data) => collect_info_enum(data, attrs),
        Data::Union(_) => Err("Union type isn't supported".to_owned()),
    }
}

fn collect_info_struct(ast: &DataStruct, attrs: ObjectAttributes) -> Result<Impl, String> {
    info_from_fields(&ast.fields, &attrs, field_var_name, "")
}

// todo: refactoring. instead of using a lambda + prefix
// we could just not emit `self.` `_x` inside
// So the called would prefix it on its own
fn info_from_fields(
    fields: &Fields,
    attrs: &ObjectAttributes,
    field_name: impl Fn(usize, &Field) -> TokenStream,
    header_prefix: &str,
) -> Result<Impl, String> {
    let count_fields = fields.len();

    let fields = fields.into_iter().enumerate().map(|(i, field)| {
        let mut attributes = Attributes::parse(&field.attrs);
        merge_attributes(&mut attributes, attrs);

        (i, field, attributes)
    });

    let mut headers = Vec::new();
    let mut values = Vec::new();
    let mut reorder = HashMap::new();

    for (i, field, attributes) in fields {
        if attributes.is_ignored() {
            continue;
        }

        if let Some(order) = attributes.order {
            if order >= count_fields {
                panic!("An order index '{}' is out of fields scope", order);
            }

            reorder.insert(order, i);
        }

        let header = field_headers(field, i, &attributes, header_prefix);

        headers.push(header);

        let field_name = field_name(i, field);
        let value = get_field_fields(field_name, &attributes);

        values.push(value);
    }

    if !reorder.is_empty() {
        values = reorder_fields(&reorder, values);
        headers = reorder_fields(&reorder, headers);
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

fn reorder_fields<T: Clone>(order: &HashMap<usize, usize>, elements: Vec<T>) -> Vec<T> {
    let mut out: Vec<Option<T>> = Vec::with_capacity(elements.len());
    out.resize(elements.len(), None);

    for (pos, index) in order {
        let value = elements[*index].clone();
        out[*pos] = Some(value);
    }

    let mut j = 0;
    for el in out.iter_mut() {
        if el.is_some() {
            continue;
        }

        while order.values().any(|&pos| j == pos) {
            j += 1;
        }

        let v = elements[j].clone();
        *el = Some(v);

        j += 1;
    }

    out.into_iter().flatten().collect()
}

fn field_headers(
    field: &Field,
    index: usize,
    attributes: &Attributes,
    prefix: &str,
) -> TokenStream {
    if attributes.inline {
        let prefix = attributes
            .inline_prefix
            .as_ref()
            .map_or_else(|| "", |s| s.as_str());
        return get_type_headers(&field.ty, prefix, "");
    }

    let header_name = field_header_name(field, attributes, index);
    if !prefix.is_empty() {
        quote!(vec![format!("{}{}", #prefix, #header_name)])
    } else {
        quote!(vec![String::from(#header_name)])
    }
}

fn collect_info_enum(ast: &DataEnum, attrs: ObjectAttributes) -> Result<Impl, String> {
    let mut headers_list = Vec::new();
    let mut variants = Vec::new();
    for variant in &ast.variants {
        let mut attributes = Attributes::parse(&variant.attrs);
        merge_attributes(&mut attributes, &attrs);
        if attributes.is_ignored() {
            continue;
        }

        let info = info_from_variant(variant, &attributes, &attrs)?;
        variants.push((variant, info.values));
        headers_list.push(info.headers);
    }

    let variant_sizes = get_enum_variant_length(ast);
    let values = values_for_enum(variant_sizes, variants);

    let headers = quote! {
        vec![
            #(#headers_list,)*
        ]
        .concat()
    };

    Ok(Impl { headers, values })
}

fn info_from_variant(
    variant: &Variant,
    attributes: &Attributes,
    attrs: &ObjectAttributes,
) -> Result<Impl, String> {
    if attributes.inline {
        let prefix = attributes
            .inline_prefix
            .as_ref()
            .map_or_else(|| "", |s| s.as_str());
        return info_from_fields(&variant.fields, attrs, variant_var_name, prefix);
    }

    let variant_name = variant_name(variant, attributes);
    let value = "+";

    // we need exactly string because of it must be inlined as string
    let headers = quote! {vec![#variant_name.to_string()]};
    // we need exactly string because of it must be inlined as string
    let values = quote! {vec![#value.to_string()]};

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
                .map(|header| format!("{}{}{}", #prefix, #inline_prefix, header))
                .collect::<Vec<_>>()
        }
    }
}

fn get_field_fields(field: TokenStream, attr: &Attributes) -> TokenStream {
    if attr.inline {
        return quote! { #field.fields() };
    }

    if let Some(func) = &attr.display_with {
        let func_call = match attr.display_with_use_self {
            true => use_function_with_self(func),
            false => use_function_for(field, func),
        };

        return quote!(vec![#func_call]);
    }

    quote!(vec![format!("{}", #field)])
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

fn use_function_with_self(function: &str) -> TokenStream {
    let path: syn::Result<syn::ExprPath> = syn::parse_str(function);
    match path {
        Ok(path) => {
            quote! { #path(&self) }
        }
        _ => {
            let function = Ident::new(function, proc_macro2::Span::call_site());
            quote! { #function(&self) }
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

fn values_for_enum(
    variant_sizes: impl Iterator<Item = TokenStream>,
    variants: Vec<(&Variant, TokenStream)>,
) -> TokenStream {
    let branches = variants.iter().map(|(variant, _)| match_variant(variant));

    let fields = variants
        .iter()
        .map(|(_, values)| values)
        .collect::<Vec<_>>();

    let mut stream = TokenStream::new();
    for (i, (branch, fields)) in branches.into_iter().zip(fields).enumerate() {
        let branch = quote! {
            Self::#branch => {
                let offset = offsets[#i];
                let fields: Vec<String> = #fields;

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
        let mut offsets: &mut [usize] = &mut [0, #(#variant_sizes,)*];
        for i in 1 .. offsets.len() {
            offsets[i] += offsets[i-1]
        }

        let size = <Self as Tabled>::LENGTH;
        let mut out_vec: Vec<String> = vec![String::new(); size];

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
        .rename
        .clone()
        .or_else(|| {
            attributes
                .rename_all
                .as_ref()
                .map(|case| case.cast(variant.ident.to_string()))
        })
        .unwrap_or_else(|| variant.ident.to_string())
}

fn field_header_name(f: &Field, attr: &Attributes, index: usize) -> String {
    if let Some(name) = &attr.rename {
        return name.to_string();
    }

    match &f.ident {
        Some(name) => {
            let name = name.to_string();
            match &attr.rename_all {
                Some(case) => case.cast(name),
                None => name,
            }
        }
        None => index.to_string(),
    }
}

// todo: make String a &static str
#[derive(Debug)]
struct Attributes {
    is_ignored: bool,
    inline: bool,
    inline_prefix: Option<String>,
    rename: Option<String>,
    rename_all: Option<CasingStyle>,
    display_with: Option<String>,
    display_with_use_self: bool,
    order: Option<usize>,
}

impl Attributes {
    fn parse(attrs: &[Attribute]) -> Self {
        let is_ignored = attrs_has_ignore_sign(attrs);
        let should_be_inlined = should_be_inlined(attrs);
        let inline_prefix = look_for_inline_prefix(attrs);
        let rename_attr = override_header_name(attrs);
        let rename_all_attr = lookup_rename_all_attr(attrs);
        let order = override_header_order(attrs);
        let (display_with, display_with_use_self) =
            check_display_with_func(attrs).map_or((None, false), |(a, b)| (Some(a), b));

        Self {
            display_with,
            display_with_use_self,
            is_ignored,
            order,
            inline_prefix,
            inline: should_be_inlined,
            rename: rename_attr,
            rename_all: rename_all_attr,
        }
    }

    fn is_ignored(&self) -> bool {
        self.is_ignored
    }
}

fn override_header_name(attrs: &[Attribute]) -> Option<String> {
    find_name_attribute(attrs, "tabled", "rename", look_up_nested_meta_str)
}

fn lookup_rename_all_attr(attrs: &[Attribute]) -> Option<CasingStyle> {
    find_name_attribute(attrs, "tabled", "rename_all", look_up_nested_meta_casing)
}

fn override_header_order(attrs: &[Attribute]) -> Option<usize> {
    find_name_attribute(attrs, "tabled", "order", look_up_nested_meta_usize)
}

fn check_display_with_func(attrs: &[Attribute]) -> Option<(String, bool)> {
    find_name_attribute_(attrs, |attr| {
        Ok(parse_display_with_attribute(attr, "tabled"))
    })
    .unwrap()
}

fn should_be_inlined(attrs: &[Attribute]) -> bool {
    let inline_attr = find_name_attribute(attrs, "tabled", "inline", look_up_nested_meta_bool)
        .or_else(|| {
            find_name_attribute(attrs, "tabled", "inline", look_up_nested_flag_str_in_attr)
                .map(|_| true)
        });
    inline_attr == Some(true)
}

fn look_for_inline_prefix(attrs: &[Attribute]) -> Option<String> {
    find_name_attribute(attrs, "tabled", "inline", look_up_nested_flag_str_in_attr)
}

fn attrs_has_ignore_sign(attrs: &[Attribute]) -> bool {
    let is_ignored = find_name_attribute(attrs, "tabled", "skip", look_up_nested_meta_bool);
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
        NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident(name) => {
            check_str_literal(&value.lit)
        }
        _ => Ok(None),
    }
}

fn look_up_nested_meta_casing(
    meta: &NestedMeta,
    name: &str,
) -> Result<Option<CasingStyle>, String> {
    match meta {
        NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident(name) => {
            check_casing_literal(&value.lit)
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

fn check_casing_literal(lit: &Lit) -> Result<Option<CasingStyle>, String> {
    match lit {
        Lit::Str(value) => Ok(Some(CasingStyle::from_lit(value.clone()))),
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
        NestedMeta::Meta(Meta::List(list)) if list.path.is_ident(name) => {
            parse_name_attribute_nested(list.nested.iter(), "", name, look_up_nested_meta_flag_str)
                .ok_or_else(|| "An attribute doesn't have expected value".to_string())
                .map(Some)
        }
        _ => Ok(None),
    }
}

fn look_up_nested_meta_usize(meta: &NestedMeta, name: &str) -> Result<Option<usize>, String> {
    match meta {
        NestedMeta::Meta(Meta::NameValue(value)) => {
            if value.path.is_ident(name) {
                check_usize_literal(&value.lit)
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

fn check_usize_literal(lit: &Lit) -> Result<Option<usize>, String> {
    match lit {
        Lit::Int(value) => {
            let value = value
                .base10_parse::<usize>()
                .map_err(|e| format!("Failed to parse int {:?}; {}", value.to_string(), e))?;
            Ok(Some(value))
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

fn find_name_attribute_<R, F>(attributes: &[Attribute], lookup: F) -> Result<Option<R>, String>
where
    F: Fn(&Attribute) -> Result<Option<R>, String>,
{
    for attr in attributes {
        let result = (lookup)(attr)?;
        if result.is_some() {
            return Ok(result);
        }
    }

    Ok(None)
}

fn parse_display_with_attribute(attr: &Attribute, attr_name: &str) -> Option<(String, bool)> {
    if !attr.path.is_ident(attr_name) {
        return None;
    }

    let meta = attr.parse_meta().ok()?;

    if let Meta::List(list) = meta {
        for meta in list.nested {
            let val = parse_display_with_attribute_obj(&meta, "display_with");
            match val {
                Ok(Some(value)) => return Some(value),
                Err(err) => {
                    panic!("{error} macros {macro} field {name}", error=err, macro=attr_name, name="display_with")
                }
                _ => {}
            }
        }
    }

    None
}

fn parse_display_with_attribute_obj(
    meta: &NestedMeta,
    attr_name: &str,
) -> Result<Option<(String, bool)>, String> {
    match meta {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. }))
            if path.is_ident(attr_name) =>
        {
            let value = take_str_literal(lit)?;
            Ok(Some((value, false)))
        }
        NestedMeta::Meta(Meta::List(list)) if list.path.is_ident(attr_name) => {
            let mut name = None;
            let mut use_self = false;
            for meta in &list.nested {
                match meta {
                    NestedMeta::Meta(Meta::Path(path)) if path.is_ident("args") => {
                        use_self = true;
                    }
                    NestedMeta::Lit(lit) if name.is_none() => {
                        name = Some(take_str_literal(lit)?);
                    }
                    _ => {}
                }
            }

            match name {
                Some(name) => Ok(Some((name, use_self))),
                None => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

fn take_str_literal(lit: &Lit) -> Result<String, String> {
    match lit {
        Lit::Str(value) => Ok(value.value()),
        Lit::ByteStr(value) => str::from_utf8(&value.value())
            .map(|s| s.to_owned())
            .map_err(|_| "Expected a valid UTF-8 string for a field".to_owned()),
        _ => Err("Expected a string but got something else".to_owned()),
    }
}

struct ObjectAttributes {
    rename_all: Option<CasingStyle>,
}

impl ObjectAttributes {
    fn parse(attrs: &[Attribute]) -> Self {
        let rename_all = lookup_rename_all_attr(attrs);
        Self { rename_all }
    }
}

fn merge_attributes(attr: &mut Attributes, global_attr: &ObjectAttributes) {
    if attr.rename_all.is_none() {
        attr.rename_all = global_attr.rename_all;
    }
}

/// Defines the casing for the attributes long representation.
#[derive(Copy, Clone, Debug, PartialEq)]
enum CasingStyle {
    /// Indicate word boundaries with uppercase letter, excluding the first word.
    Camel,
    /// Keep all letters lowercase and indicate word boundaries with hyphens.
    Kebab,
    /// Indicate word boundaries with uppercase letter, including the first word.
    Pascal,
    /// Keep all letters uppercase and indicate word boundaries with underscores.
    ScreamingSnake,
    /// Keep all letters lowercase and indicate word boundaries with underscores.
    Snake,
    /// Keep all letters lowercase and remove word boundaries.
    Lower,
    /// Keep all letters uppercase and remove word boundaries.
    Upper,
    /// Use the original attribute name defined in the code.
    Verbatim,
}

impl CasingStyle {
    fn from_lit(name: syn::LitStr) -> Self {
        use self::CasingStyle::*;
        use heck::ToUpperCamelCase;

        let normalized = name.value().to_upper_camel_case().to_lowercase();

        match normalized.as_ref() {
            "camel" | "camelcase" => Camel,
            "kebab" | "kebabcase" => Kebab,
            "pascal" | "pascalcase" => Pascal,
            "screamingsnake" | "screamingsnakecase" => ScreamingSnake,
            "snake" | "snakecase" => Snake,
            "lower" | "lowercase" => Lower,
            "upper" | "uppercase" => Upper,
            "verbatim" | "verbatimcase" => Verbatim,
            _ => panic!("unsupported casing: `{:?}`; supperted values are ['camelCase', 'kebab-case', 'PascalCase', 'SCREAMING_SNAKE_CASE', 'snake_case', 'lowercase', 'UPPERCASE', 'verbatim']", name.value()),
        }
    }

    fn cast(&self, s: String) -> String {
        use CasingStyle::*;

        match self {
            Pascal => heck::ToUpperCamelCase::to_upper_camel_case(s.as_str()),
            Camel => heck::ToLowerCamelCase::to_lower_camel_case(s.as_str()),
            Kebab => heck::ToKebabCase::to_kebab_case(s.as_str()),
            Snake => heck::ToSnakeCase::to_snake_case(s.as_str()),
            ScreamingSnake => heck::ToShoutySnakeCase::to_shouty_snake_case(s.as_str()),
            Lower => heck::ToSnakeCase::to_snake_case(s.as_str()).replace('_', ""),
            Upper => heck::ToShoutySnakeCase::to_shouty_snake_case(s.as_str()).replace('_', ""),
            Verbatim => s,
        }
    }
}
