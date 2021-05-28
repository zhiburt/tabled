extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Tabled)]
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
        syn::Data::Union(u) => get_union_headers(u),
    }
}

fn get_st_headers(st: &syn::DataStruct) -> Vec<String> {
    st.fields
        .iter()
        .map(|f| f.ident.as_ref())
        .enumerate()
        .map(|(i, f)| f.map_or_else(|| format!("{}", i), |f| f.to_string()))
        .collect()
}

fn get_enum_headers(e: &syn::DataEnum) -> Vec<String> {
    e.variants
        .iter()
        .map(|v| {
            let variant = v.ident.to_string();
            if v.fields.len() == 0 {
                vec![format!("{}", variant)]
            } else {
                v.fields
                    .iter()
                    .map(|f| f.ident.as_ref())
                    .enumerate()
                    .map(|(i, f)| {
                        f.map_or_else(
                            || format!("{}::{}", variant, i),
                            |f| format!("{}::{}", variant, f.to_string()),
                        )
                    })
                    .collect()
            }
        })
        .collect::<Vec<Vec<_>>>()
        .concat()
}

fn get_union_headers(u: &syn::DataUnion) -> Vec<String> {
    u.fields
        .named
        .iter()
        .enumerate()
        .map(|(i, _)| format!("field-{}", i))
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
    st.fields
        .iter()
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
    for variant in &e.variants {
        let fields = if variant.fields.is_empty() {
            vec![quote! { "+".to_string() }]
        } else {
            variant
                .fields
                .iter()
                .map(|f| f.ident.as_ref())
                .enumerate()
                .map(|(i, f)| {
                    f.map_or_else(
                        || todo!("a tuple based struct doesn't implemented; here supposed to be a generated Ident for a tuple"),
                        |f| f.to_token_stream(),
                    )
                })
                .collect::<Vec<_>>()
        };

        variant_field_shift.push(count_fields);
        variant_fields_len.push(fields.len());
        count_fields += fields.len();
        fields_per_variant.push(fields);
    }

    let variants = e
        .variants
        .iter()
        .map(|v| {
            let mut token = proc_macro2::TokenStream::new();
            token.append_all(v.ident.to_token_stream());

            let parameters = v
                .fields
                .iter()
                .map(|f| f.ident.as_ref())
                .enumerate()
                .map(|(i, f)| {
                    f.map_or_else(
                        || todo!("a tuple based struct doesn't implemented; here supposed to be a generated Ident for a tuple"),
                        |f| quote! { #f,},
                    )
                })
                .collect::<Vec<_>>();

            // todo: for tuple struct we should use a syn::token::Paren::default()
            syn::token::Brace::default().surround(&mut token, |s| {
                s.append_all(parameters);
            });

            token
        })
        .collect::<Vec<_>>();

    quote! {
        let size = #count_fields;
        let mut v: Vec<String> = std::iter::repeat(String::new()).take(size).collect();
        match &self {
            #(Self::#variants => {
                let fields = vec![#(#fields_per_variant.to_string()),*];

                for i in #variant_field_shift..#variant_field_shift+#variant_fields_len {
                    v[i] = fields[i-#variant_field_shift].clone();
                }

                v
            },)*
        }
    }
}
