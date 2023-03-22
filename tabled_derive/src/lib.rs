#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

extern crate proc_macro;

mod attributes;
mod casing_style;
mod error;
mod parse;

use proc_macro2::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens, TokenStreamExt};
use std::{collections::HashMap, str};
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Ident, Index,
    Type, Variant,
};

use attributes::{Attributes, FuncArg, StructAttributes};
use error::Error;

#[proc_macro_derive(Tabled, attributes(tabled))]
#[proc_macro_error]
pub fn tabled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_tabled(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_tabled(ast: &DeriveInput) -> TokenStream {
    let attrs = StructAttributes::parse(&ast.attrs)
        .map_err(error::abort)
        .unwrap();

    let length = get_tabled_length(ast, &attrs)
        .map_err(error::abort)
        .unwrap();
    let info = collect_info(ast, &attrs).map_err(error::abort).unwrap();
    let fields = info.values;
    let headers = info.headers;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics ::tabled::Tabled for #name #ty_generics #where_clause {
            const LENGTH: usize = #length;

            fn fields(&self) -> Vec<::std::borrow::Cow<'_, str>> {
                #fields
            }

            fn headers() -> Vec<::std::borrow::Cow<'static, str>> {
                #headers
            }
        }
    };

    expanded
}

fn get_tabled_length(ast: &DeriveInput, attrs: &StructAttributes) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(data) => get_fields_length(&data.fields),
        Data::Enum(data) => {
            if attrs.inline {
                Ok(quote! { 1 })
            } else {
                get_enum_length(data)
            }
        }
        Data::Union(_) => Err(Error::message("Union type isn't supported")),
    }
}

fn get_fields_length(fields: &Fields) -> Result<TokenStream, Error> {
    let size_components = fields
        .iter()
        .map(|field| {
            let attributes = Attributes::parse(&field.attrs)?;
            Ok((field, attributes))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .filter(|(_, attr)| !attr.is_ignored())
        .map(|(field, attr)| {
            if attr.inline {
                let field_type = &field.ty;
                quote!({<#field_type as Tabled>::LENGTH})
            } else {
                quote!({ 1 })
            }
        });

    let size_components = std::iter::once(quote!(0)).chain(size_components);

    let mut stream = TokenStream::new();
    stream.append_separated(size_components, syn::token::Add::default());

    Ok(stream)
}

fn get_enum_length(enum_ast: &DataEnum) -> Result<TokenStream, Error> {
    let variant_sizes = get_enum_variant_length(enum_ast);

    let mut stream = TokenStream::new();
    for (i, size) in variant_sizes.enumerate() {
        let size = size?;

        if i != 0 {
            stream.append_all(syn::token::Add::default().into_token_stream());
        }

        stream.append_all(size);
    }

    Ok(stream)
}

fn get_enum_variant_length(
    enum_ast: &DataEnum,
) -> impl Iterator<Item = Result<TokenStream, Error>> + '_ {
    enum_ast
        .variants
        .iter()
        .map(|variant| -> Result<_, Error> {
            let attributes = Attributes::parse(&variant.attrs)?;
            Ok((variant, attributes))
        })
        .filter(|result| result.is_err() || matches!(result, Ok((_, attr)) if !attr.is_ignored()))
        .map(|result| {
            let (variant, attr) = result?;

            if attr.inline {
                get_fields_length(&variant.fields)
            } else {
                Ok(quote!(1))
            }
        })
}

fn collect_info(ast: &DeriveInput, attrs: &StructAttributes) -> Result<Impl, Error> {
    match &ast.data {
        Data::Struct(data) => collect_info_struct(data, attrs),
        Data::Enum(data) => collect_info_enum(data, attrs, &ast.ident),
        Data::Union(_) => Err(Error::message("Union type isn't supported")),
    }
}

fn collect_info_struct(ast: &DataStruct, attrs: &StructAttributes) -> Result<Impl, Error> {
    info_from_fields(&ast.fields, attrs, field_var_name, "")
}

// todo: refactoring. instead of using a lambda + prefix
// we could just not emit `self.` `_x` inside
// So the called would prefix it on its own
fn info_from_fields(
    fields: &Fields,
    attrs: &StructAttributes,
    field_name: impl Fn(usize, &Field) -> TokenStream,
    header_prefix: &str,
) -> Result<Impl, Error> {
    let count_fields = fields.len();

    let fields = fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| -> Result<_, Error> {
            let mut attributes = Attributes::parse(&field.attrs)?;
            merge_attributes(&mut attributes, attrs);

            Ok((i, field, attributes))
        });

    let mut headers = Vec::new();
    let mut values = Vec::new();
    let mut reorder = HashMap::new();

    let mut skipped = 0;
    for result in fields {
        let (i, field, attributes) = result?;
        if attributes.is_ignored() {
            skipped += 1;
            continue;
        }

        if let Some(order) = attributes.order {
            if order >= count_fields {
                return Err(Error::message(format!(
                    "An order index '{order}' is out of fields scope"
                )));
            }

            reorder.insert(order, i - skipped);
        }

        let header = field_headers(field, i, &attributes, header_prefix);
        headers.push(header);

        let field_name = field_name(i, field);
        let value = get_field_fields(&field_name, &attributes);
        values.push(value);
    }

    if !reorder.is_empty() {
        values = reorder_fields(&reorder, &values);
        headers = reorder_fields(&reorder, &headers);
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

fn reorder_fields<T: Clone>(order: &HashMap<usize, usize>, elements: &[T]) -> Vec<T> {
    let mut out: Vec<Option<T>> = Vec::with_capacity(elements.len());
    out.resize(elements.len(), None);

    for (pos, index) in order {
        let value = elements[*index].clone();
        out[*pos] = Some(value);
    }

    let mut j = 0;
    for el in &mut out {
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
    if prefix.is_empty() {
        quote!(vec![::std::borrow::Cow::Borrowed(#header_name)])
    } else {
        let name = format!("{prefix}{header_name}");
        quote!(vec![::std::borrow::Cow::Borrowed(#name)])
    }
}

fn collect_info_enum(
    ast: &DataEnum,
    attrs: &StructAttributes,
    name: &Ident,
) -> Result<Impl, Error> {
    match &attrs.inline {
        true => {
            let enum_name = attrs
                .inline_value
                .clone()
                .unwrap_or_else(|| name.to_string());

            collect_info_enum_inlined(ast, attrs, enum_name)
        }
        false => _collect_info_enum(ast, attrs),
    }
}

fn _collect_info_enum(ast: &DataEnum, attrs: &StructAttributes) -> Result<Impl, Error> {
    // reorder variants according to order (if set)
    let orderedvariants = reodered_variants(ast)?;

    let mut headers_list = Vec::new();
    let mut variants = Vec::new();
    for v in orderedvariants {
        let mut attributes = Attributes::parse(&v.attrs)?;
        merge_attributes(&mut attributes, attrs);
        if attributes.is_ignored() {
            continue;
        }

        let info = info_from_variant(v, &attributes, attrs)?;
        variants.push((v, info.values));
        headers_list.push(info.headers);
    }

    let variant_sizes = get_enum_variant_length(ast)
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter();
    let values = values_for_enum(variant_sizes, &variants);

    let headers = quote! {
        vec![
            #(#headers_list,)*
        ]
        .concat()
    };

    Ok(Impl { headers, values })
}

fn collect_info_enum_inlined(
    ast: &DataEnum,
    attrs: &StructAttributes,
    enum_name: String,
) -> Result<Impl, Error> {
    let orderedvariants = reodered_variants(ast)?;

    let mut variants = Vec::new();
    let mut names = Vec::new();
    for variant in orderedvariants {
        let mut attributes = Attributes::parse(&variant.attrs)?;
        merge_attributes(&mut attributes, attrs);
        let mut name = String::new();
        if !attributes.is_ignored() {
            name = variant_name(variant, &attributes);
        }

        variants.push(match_variant(variant));
        names.push(name);
    }

    let headers = quote! { vec![::std::borrow::Cow::Borrowed(#enum_name)] };
    let values = quote! {
        #[allow(unused_variables)]
        match &self {
            #(Self::#variants => vec![::std::borrow::Cow::Borrowed(#names)],)*
        }
    };

    Ok(Impl { headers, values })
}

fn info_from_variant(
    variant: &Variant,
    attr: &Attributes,
    attrs: &StructAttributes,
) -> Result<Impl, Error> {
    if attr.inline {
        let prefix = attr
            .inline_prefix
            .as_ref()
            .map_or_else(|| "", |s| s.as_str());
        return info_from_fields(&variant.fields, attrs, variant_var_name, prefix);
    }

    let variant_name = variant_name(variant, attr);
    let value = if let Some(func) = &attr.display_with {
        let args = match &attr.display_with_args {
            None => None,
            Some(args) => match args.is_empty() {
                true => None,
                false => {
                    let args = args.iter().map(fnarg_tokens).collect::<Vec<_>>();
                    Some(quote!( #(#args,)* ))
                }
            },
        };

        let call = match args {
            Some(args) => use_function(&args, func),
            None => use_function_no_args(func),
        };

        quote! { ::std::borrow::Cow::from(#call) }
    } else {
        let default_value = "+";
        quote! { ::std::borrow::Cow::Borrowed(#default_value) }
    };

    // we need exactly string because of it must be inlined as string
    let headers = quote! { vec![::std::borrow::Cow::Borrowed(#variant_name)] };
    // we need exactly string because of it must be inlined as string
    let values = quote! { vec![#value] };

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
                .map(|header| {
                    let header = format!("{}{}{}", #prefix, #inline_prefix, header);
                    ::std::borrow::Cow::Owned(header)
                })
                .collect::<Vec<_>>()
        }
    }
}

fn get_field_fields(field: &TokenStream, attr: &Attributes) -> TokenStream {
    if attr.inline {
        return quote! { #field.fields() };
    }

    if let Some(func) = &attr.display_with {
        let args = match &attr.display_with_args {
            None => Some(quote!(&#field)),
            Some(args) => match args.is_empty() {
                true => None,
                false => {
                    let args = args.iter().map(fnarg_tokens).collect::<Vec<_>>();
                    Some(quote!( #(#args,)* ))
                }
            },
        };

        let call = match args {
            Some(args) => use_function(&args, func),
            None => use_function_no_args(func),
        };

        return quote!(vec![::std::borrow::Cow::from(#call)]);
    }

    quote!(vec![::std::borrow::Cow::Owned(format!("{}", #field))])
}

fn use_function(args: &TokenStream, function: &str) -> TokenStream {
    let path: syn::Result<syn::ExprPath> = syn::parse_str(function);
    match path {
        Ok(path) => {
            quote! { #path(#args) }
        }
        Err(_) => {
            let function = Ident::new(function, proc_macro2::Span::call_site());
            quote! { #function(#args) }
        }
    }
}

fn use_function_no_args(function: &str) -> TokenStream {
    let path: syn::Result<syn::ExprPath> = syn::parse_str(function);
    match path {
        Ok(path) => {
            quote! { #path() }
        }
        Err(_) => {
            let function = Ident::new(function, proc_macro2::Span::call_site());
            quote! { #function() }
        }
    }
}

fn field_var_name(index: usize, field: &Field) -> TokenStream {
    let f = field.ident.as_ref().map_or_else(
        || Index::from(index).to_token_stream(),
        quote::ToTokens::to_token_stream,
    );
    quote!(self.#f)
}

fn variant_var_name(index: usize, field: &Field) -> TokenStream {
    match &field.ident {
        Some(indent) => indent.to_token_stream(),
        None => Ident::new(
            format!("x_{index}").as_str(),
            proc_macro2::Span::call_site(),
        )
        .to_token_stream(),
    }
}

fn values_for_enum(
    variant_sizes: impl Iterator<Item = TokenStream>,
    variants: &[(&Variant, TokenStream)],
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
                let fields: Vec<::std::borrow::Cow<'_, str>> = #fields;

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
        let mut out_vec = vec![::std::borrow::Cow::Borrowed(""); size];

        #[allow(unused_variables)]
        match &self {
            #stream
            _ => return vec![::std::borrow::Cow::Borrowed(""); size], // variant is hidden so we return an empty vector
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

fn merge_attributes(attr: &mut Attributes, global_attr: &StructAttributes) {
    if attr.rename_all.is_none() {
        attr.rename_all = global_attr.rename_all;
    }
}

fn fnarg_tokens(arg: &FuncArg) -> TokenStream {
    match arg {
        FuncArg::SelfRef => quote! { &self },
        FuncArg::Byte(val) => quote! { #val },
        FuncArg::Char(val) => quote! { #val },
        FuncArg::Bool(val) => quote! { #val },
        FuncArg::Uint(val) => quote! { #val },
        FuncArg::Int(val) => quote! { #val },
        FuncArg::Float(val) => quote! { #val },
        FuncArg::String(val) => quote! { #val },
        FuncArg::Bytes(val) => {
            let val = syn::LitByteStr::new(val, proc_macro2::Span::call_site());
            quote! { #val }
        }
    }
}

fn reodered_variants(ast: &DataEnum) -> Result<Vec<&Variant>, Error> {
    let mut reorder = HashMap::new();
    let mut skip = 0;
    let count = ast.variants.len();
    for (i, attr) in ast
        .variants
        .iter()
        .map(|v| Attributes::parse(&v.attrs).unwrap_or_default())
        .enumerate()
    {
        if attr.is_ignored {
            skip += 1;
            continue;
        }

        if let Some(order) = attr.order {
            if order >= count {
                return Err(Error::message(format!(
                    "An order index '{order}' is out of fields scope"
                )));
            }

            reorder.insert(order, i - skip);
        }
    }

    let mut orderedvariants = ast.variants.iter().collect::<Vec<_>>();
    if !reorder.is_empty() {
        orderedvariants = reorder_fields(&reorder, &orderedvariants);
    }

    Ok(orderedvariants)
}
