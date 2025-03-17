#![allow(clippy::uninlined_format_args)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

extern crate proc_macro;

mod attributes;
mod casing_style;
mod error;
mod parse;

use attributes::FormatArg;
use proc_macro2::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::{quote, ToTokens, TokenStreamExt};
use std::{collections::HashMap, str};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{
    parse_macro_input, token, Data, DataEnum, DataStruct, DeriveInput, ExprPath, Field, Fields,
    Ident, Index, PathSegment, Type, TypePath, Variant,
};

use crate::attributes::{FieldAttributes, TypeAttributes};
use crate::error::Error;

type FieldNameFn = fn(usize, &Field) -> TokenStream;

#[proc_macro_derive(Tabled, attributes(tabled))]
#[proc_macro_error]
pub fn tabled(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ast = impl_tabled(&input);
    proc_macro::TokenStream::from(ast)
}

fn impl_tabled(ast: &DeriveInput) -> TokenStream {
    let attrs = TypeAttributes::parse(&ast.attrs)
        .map_err(error::abort)
        .unwrap();

    let tabled_trait_path = get_crate_name_expr(&attrs).map_err(error::abort).unwrap();

    let length = get_tabled_length(ast, &attrs, &tabled_trait_path)
        .map_err(error::abort)
        .unwrap();
    let info = collect_info(ast, &attrs, &tabled_trait_path)
        .map_err(error::abort)
        .unwrap();
    let fields = info.values;
    let headers = info.headers;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #tabled_trait_path for #name #ty_generics #where_clause {
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

fn get_tabled_length(
    ast: &DeriveInput,
    attrs: &TypeAttributes,
    trait_path: &ExprPath,
) -> Result<TokenStream, Error> {
    match &ast.data {
        Data::Struct(data) => get_fields_length(&data.fields, trait_path),
        Data::Enum(data) => {
            if attrs.inline {
                Ok(quote! { 1 })
            } else {
                get_enum_length(data, trait_path)
            }
        }
        Data::Union(_) => Err(Error::message("Union type isn't supported")),
    }
}

fn get_fields_length(fields: &Fields, tabled_trait: &ExprPath) -> Result<TokenStream, Error> {
    let size_components = fields
        .iter()
        .map(|field| {
            let attributes = FieldAttributes::parse(&field.attrs)?;
            Ok((field, attributes))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .filter(|(_, attr)| !attr.is_ignored)
        .map(|(field, attr)| {
            if attr.inline {
                if attr.map.is_some() {
                    match attr.map_type {
                        Some(map_type) => Ok(quote!({ <#map_type as #tabled_trait>::LENGTH })),
                        None => Err(Error::new(
                            "map type was not given",
                            field.span(),
                            Some(String::from("provide a type to map attribute")),
                        )),
                    }
                } else {
                    let field_type = &field.ty;
                    Ok(quote!({<#field_type as #tabled_trait>::LENGTH}))
                }
            } else {
                Ok(quote!({ 1 }))
            }
        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter();

    let size_components = std::iter::once(quote!(0)).chain(size_components);

    let mut stream = TokenStream::new();
    stream.append_separated(size_components, syn::token::Plus::default());

    Ok(stream)
}

fn get_enum_length(enum_ast: &DataEnum, trait_path: &ExprPath) -> Result<TokenStream, Error> {
    let variant_sizes = get_enum_variant_length(enum_ast, trait_path);

    let mut stream = TokenStream::new();
    for (i, size) in variant_sizes.enumerate() {
        let size = size?;

        if i != 0 {
            stream.append_all(syn::token::Plus::default().into_token_stream());
        }

        stream.append_all(size);
    }

    Ok(stream)
}

fn get_enum_variant_length<'a>(
    enum_ast: &'a DataEnum,
    trait_path: &'a ExprPath,
) -> impl Iterator<Item = Result<TokenStream, Error>> + 'a {
    enum_ast
        .variants
        .iter()
        .map(|variant| -> Result<_, Error> {
            let attributes = FieldAttributes::parse(&variant.attrs)?;
            Ok((variant, attributes))
        })
        .filter(|result| result.is_err() || matches!(result, Ok((_, attr)) if !attr.is_ignored))
        .map(move |result| {
            let (variant, attr) = result?;

            if attr.inline {
                get_fields_length(&variant.fields, trait_path)
            } else {
                Ok(quote!(1))
            }
        })
}

fn collect_info(
    ast: &DeriveInput,
    attrs: &TypeAttributes,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    match &ast.data {
        Data::Struct(data) => collect_info_struct(data, attrs, trait_path),
        Data::Enum(data) => collect_info_enum(data, attrs, &ast.ident, trait_path),
        Data::Union(_) => Err(Error::message("Union type isn't supported")),
    }
}

fn collect_info_struct(
    ast: &DataStruct,
    attrs: &TypeAttributes,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    info_from_fields(&ast.fields, attrs, struct_field_name, "", trait_path)
}

// todo: refactoring. instead of using a lambda + prefix
// we could just not emit `self.` `_x` inside
// So the called would prefix it on its own
fn info_from_fields(
    fields: &Fields,
    attrs: &TypeAttributes,
    field_name: FieldNameFn,
    header_prefix: &str,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    let count_fields = fields.len();

    let attributes = fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| -> Result<_, Error> {
            let mut attributes = FieldAttributes::parse(&field.attrs)?;
            merge_attributes(&mut attributes, attrs);

            Ok((i, field, attributes))
        });

    let mut headers = Vec::new();
    let mut values = Vec::new();
    let mut reorder = HashMap::new();

    let mut skipped = 0;
    for result in attributes {
        let (i, field, attributes) = result?;
        if attributes.is_ignored {
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

        let header = field_headers(field, i, &attributes, header_prefix, trait_path);
        headers.push(header);

        let field_name_result = field_name(i, field);
        let value = get_field_fields(
            &field_name_result,
            &field.ty,
            &attributes,
            fields,
            field_name,
            attrs,
        );
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
    attrs: &FieldAttributes,
    prefix: &str,
    trait_path: &ExprPath,
) -> TokenStream {
    if attrs.inline {
        let prefix = attrs
            .inline_prefix
            .as_ref()
            .map_or_else(|| "", |s| s.as_str());

        if attrs.map.is_some() {
            if let Some(map_type) = &attrs.map_type {
                return get_type_headers(map_type, prefix, "", trait_path);
            } else {
                // NOTE: A panic already must have been raised
                unreachable!();
            }
        }

        return get_type_headers(&field.ty, prefix, "", trait_path);
    }

    let header_name = field_header_name(field, attrs, index);
    if prefix.is_empty() {
        quote!(vec![::std::borrow::Cow::Borrowed(#header_name)])
    } else {
        let name = format!("{prefix}{header_name}");
        quote!(vec![::std::borrow::Cow::Borrowed(#name)])
    }
}

fn collect_info_enum(
    ast: &DataEnum,
    attrs: &TypeAttributes,
    name: &Ident,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    match &attrs.inline {
        true => {
            let enum_name = attrs
                .inline_value
                .clone()
                .unwrap_or_else(|| name.to_string());

            collect_info_enum_inlined(ast, attrs, enum_name)
        }
        false => _collect_info_enum(ast, attrs, trait_path),
    }
}

fn _collect_info_enum(
    ast: &DataEnum,
    attrs: &TypeAttributes,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    // reorder variants according to order (if set)
    let orderedvariants = reodered_variants(ast)?;

    let mut headers_list = Vec::new();
    let mut variants = Vec::new();
    for v in orderedvariants {
        let mut attributes = FieldAttributes::parse(&v.attrs)?;
        merge_attributes(&mut attributes, attrs);
        if attributes.is_ignored {
            continue;
        }

        let info = info_from_variant(v, &attributes, attrs, trait_path)?;
        variants.push((v, info.values));
        headers_list.push(info.headers);
    }

    let variant_sizes = get_enum_variant_length(ast, trait_path)
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter();
    let values = values_for_enum(variant_sizes, &variants, trait_path);

    let headers = quote! {
        [
            #(#headers_list,)*
        ]
        .concat()
    };

    Ok(Impl { headers, values })
}

fn collect_info_enum_inlined(
    ast: &DataEnum,
    attrs: &TypeAttributes,
    enum_name: String,
) -> Result<Impl, Error> {
    let orderedvariants = reodered_variants(ast)?;

    let mut variants = Vec::new();
    let mut names = Vec::new();
    for variant in orderedvariants {
        let mut attributes = FieldAttributes::parse(&variant.attrs)?;
        merge_attributes(&mut attributes, attrs);
        let mut name = String::new();
        if !attributes.is_ignored {
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
    attr: &FieldAttributes,
    attrs: &TypeAttributes,
    trait_path: &ExprPath,
) -> Result<Impl, Error> {
    if attr.inline {
        let prefix = attr
            .inline_prefix
            .as_ref()
            .map_or_else(|| "", |s| s.as_str());
        return info_from_fields(
            &variant.fields,
            attrs,
            variant_field_name,
            prefix,
            trait_path,
        );
    }

    let variant_name = variant_name(variant, attr);
    let value = if let Some(func) = &attr.display_with {
        let args = match &attr.display_with_args {
            Some(args) => {
                args_to_tokens_with(&Fields::Unit, &quote!(self), struct_field_name, args)
            }
            None => quote!(&self),
        };

        let result = use_function(&args, func);

        quote! { ::std::borrow::Cow::from(#result) }
    } else if let Some(fmt) = &attr.format {
        let args = attr
            .format_with_args
            .as_ref()
            .and_then(|args| args_to_tokens(&Fields::Unit, struct_field_name, args));

        let call = match args {
            Some(args) => use_format(fmt, &args),
            None => use_format_no_args(fmt),
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

fn get_type_headers(
    field_type: &Type,
    inline_prefix: &str,
    prefix: &str,
    tabled_trait: &ExprPath,
) -> TokenStream {
    if prefix.is_empty() && inline_prefix.is_empty() {
        quote! { <#field_type as #tabled_trait>::headers() }
    } else {
        quote! {
            <#field_type as #tabled_trait>::headers().into_iter()
                .map(|header| {
                    let header = format!("{}{}{}", #prefix, #inline_prefix, header);
                    ::std::borrow::Cow::Owned(header)
                })
                .collect::<Vec<_>>()
        }
    }
}

fn get_field_fields(
    field: &TokenStream,
    field_type: &Type,
    attr: &FieldAttributes,
    fields: &Fields,
    field_name: FieldNameFn,
    type_attrs: &TypeAttributes,
) -> TokenStream {
    let mut field = std::borrow::Cow::Borrowed(field);
    if let Some(map_fn) = &attr.map {
        let arg = quote!(&#field);
        let result = use_function(&arg, map_fn);
        field = std::borrow::Cow::Owned(result);

        if attr.inline {
            return quote! {
                {
                    let field = #field;
                    let fields = field.fields();
                    let fields = fields.into_iter()
                        .map(|f| f.into_owned())
                        .map(std::borrow::Cow::Owned)
                        .collect::<Vec<_>>();
                    fields
                }
            };
        }
    }

    if attr.inline {
        return quote! { #field.fields() };
    }

    if let Some(func) = &attr.display_with {
        let args = match &attr.display_with_args {
            Some(args) => args_to_tokens_with(fields, &field, field_name, args),
            None => quote!(&#field),
        };

        let result = use_function(&args, func);

        return quote!(vec![::std::borrow::Cow::from(#result)]);
    } else if let Some(fmt) = &attr.format {
        let args = attr
            .format_with_args
            .as_ref()
            .and_then(|args| args_to_tokens(fields, field_name, args));

        let call = match args {
            Some(args) => use_format(fmt, &args),
            None => use_format_with_one_arg(fmt, &field),
        };

        return quote!(vec![::std::borrow::Cow::Owned(#call)]);
    }

    if let Some(i) = find_display_type(field_type, &type_attrs.display_types) {
        let (_, func, args) = &type_attrs.display_types[i];
        let args = args_to_tokens_with(fields, &field, field_name, args);
        let func = use_function(&args, func);

        return quote!(vec![::std::borrow::Cow::from(#func)]);
    }

    quote!(vec![::std::borrow::Cow::Owned(format!("{}", #field))])
}

fn args_to_tokens(
    fields: &Fields,
    field_name: fn(usize, &Field) -> TokenStream,
    args: &[FormatArg],
) -> Option<TokenStream> {
    if args.is_empty() {
        return None;
    }

    let args = args
        .iter()
        .map(|arg| fnarg_tokens(arg, fields, field_name))
        .collect::<Vec<_>>();
    Some(quote!( #(#args,)* ))
}

fn args_to_tokens_with(
    fields: &Fields,
    field: &TokenStream,
    field_name: fn(usize, &Field) -> TokenStream,
    args: &[FormatArg],
) -> TokenStream {
    if args.is_empty() {
        return quote!(&#field);
    }

    let mut out = vec![quote!(&#field)];
    for arg in args {
        let arg = fnarg_tokens(arg, fields, field_name);
        out.push(arg);
    }

    quote!( #(#out,)* )
}

fn find_display_type(ty: &Type, types: &[(TypePath, String, Vec<FormatArg>)]) -> Option<usize> {
    let p: &TypePath = match ty {
        Type::Path(path) => path,
        _ => return None,
    };

    // NOTICE:
    // We do iteration in a back order to satisfy a later set argument first.
    //
    // TODO: Maybe we shall change the data structure for it rather then doing a reverse iteration?
    // I am just saying it's dirty a little.
    let args = types.iter().enumerate().rev();
    for (i, (arg, _, _)) in args {
        if arg.path == p.path {
            return Some(i);
        }

        // NOTICE:
        // There's a specical case where we wanna match a type without a generic,
        // e.g. 'Option' with which we wanna match all 'Option's.
        //
        // Because in the scope we can only have 1 type name, it's considered to be good,
        // and nothing must be broken.
        let arg_segment = arg.path.segments.last();
        let type_segment = p.path.segments.last();
        if let Some(arg) = arg_segment {
            if arg.arguments.is_empty() {
                if let Some(p) = type_segment {
                    if p.ident == arg.ident {
                        return Some(i);
                    }
                }
            }
        }
    }

    None
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

fn use_format(custom_format: &str, args: &TokenStream) -> TokenStream {
    quote! { format!(#custom_format, #args) }
}

fn use_format_with_one_arg(custom_format: &str, field: &TokenStream) -> TokenStream {
    quote! { format!(#custom_format, #field) }
}

fn use_format_no_args(custom_format: &str) -> TokenStream {
    quote! { format!(#custom_format) }
}

fn struct_field_name(index: usize, field: &Field) -> TokenStream {
    let f = field.ident.as_ref().map_or_else(
        || Index::from(index).to_token_stream(),
        quote::ToTokens::to_token_stream,
    );
    quote!(self.#f)
}

fn variant_field_name(index: usize, field: &Field) -> TokenStream {
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
    tabled_trait: &ExprPath,
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

        let size = <Self as #tabled_trait>::LENGTH;
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
        .map(|(index, field)| variant_field_name(index, field))
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

fn variant_name(variant: &Variant, attributes: &FieldAttributes) -> String {
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

fn field_header_name(f: &Field, attr: &FieldAttributes, index: usize) -> String {
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

fn merge_attributes(attr: &mut FieldAttributes, global_attr: &TypeAttributes) {
    if attr.rename_all.is_none() {
        attr.rename_all = global_attr.rename_all;
    }
}

// to resolve invocation issues withing a macros calls
// we need such a workround
struct ExprSelfReplace<'a>(Option<(&'a Fields, FieldNameFn)>);

impl syn::visit_mut::VisitMut for ExprSelfReplace<'_> {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match &node {
            syn::Expr::Path(path) => {
                let indent = path.path.get_ident();
                if let Some(indent) = indent {
                    if indent == "self" {
                        *node = syn::parse_quote! { (&self) };
                        return;
                    }
                }
            }
            syn::Expr::Field(field) => {
                // treating a enum variant structs which we can't reference by 'self'
                if let Some((fields, field_name)) = &self.0 {
                    // check that it's plain self reference

                    if let syn::Expr::Path(path) = field.base.as_ref() {
                        let indent = path.path.get_ident();
                        if let Some(indent) = indent {
                            if indent != "self" {
                                return;
                            }
                        }
                    }

                    let used_field = {
                        match &field.member {
                            syn::Member::Named(ident) => ident.to_string(),
                            syn::Member::Unnamed(index) => index.index.to_string(),
                        }
                    };

                    // We find the corresponding field in the local object fields instead of using self,
                    // which would be a higher level object. This is for nested structures.

                    for (i, field) in fields.iter().enumerate() {
                        let field_name_result = (field_name)(i, field);
                        let field_name = field
                            .ident
                            .as_ref()
                            .map_or_else(|| i.to_string(), |i| i.to_string());
                        if field_name == used_field {
                            *node = syn::parse_quote! { #field_name_result };
                            return;
                        }
                    }
                }
            }
            syn::Expr::Macro(_) => {
                // NOTE: Can we parse inners of Macros?
                //
                // A next example will fail on `self.f1` usage
                // with such error 'expected value, found module `self`'
                //
                // ```
                // some_macro! {
                //     struct Something {
                //         #[tabled(display("_", format!("", self.f1)))]
                //         field: Option<sstr>,
                //         f1: usize,
                //     }
                // }
                // ```
            }
            _ => (),
        }

        // Delegate to the default impl to visit nested expressions.
        syn::visit_mut::visit_expr_mut(self, node);
    }
}

fn fnarg_tokens(arg: &FormatArg, fields: &Fields, field_name: FieldNameFn) -> TokenStream {
    let mut exp = arg.expr.clone();

    ExprSelfReplace(Some((fields, field_name))).visit_expr_mut(&mut exp);

    quote!(#exp)
}

fn reodered_variants(ast: &DataEnum) -> Result<Vec<&Variant>, Error> {
    let mut reorder = HashMap::new();
    let mut skip = 0;
    let count = ast.variants.len();
    for (i, attr) in ast
        .variants
        .iter()
        .map(|v| FieldAttributes::parse(&v.attrs).unwrap_or_default())
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

fn get_crate_name_expr(attrs: &TypeAttributes) -> Result<ExprPath, Error> {
    let crate_name = attrs
        .crate_name
        .clone()
        .unwrap_or_else(|| String::from("::tabled"));
    let crate_name = parse_crate_name(&crate_name)?;
    Ok(create_tabled_trait_path(crate_name))
}

fn parse_crate_name(name: &str) -> Result<ExprPath, Error> {
    syn::parse_str(name).map_err(|_| Error::message("unexpected crate attribute type"))
}

fn create_tabled_trait_path(mut p: ExprPath) -> ExprPath {
    p.path.segments.push(PathSegment {
        ident: Ident::new("Tabled", proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    });
    p
}
