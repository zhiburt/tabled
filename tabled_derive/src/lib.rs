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
                vec![#(format!("{}", self.#fields),)*]
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
        _ => unimplemented!(),
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

fn get_fields(d: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match d {
        syn::Data::Struct(st) => get_st_fields(st),
        _ => unimplemented!(),
    }
}

fn get_st_fields(st: &syn::DataStruct) -> Vec<proc_macro2::TokenStream> {
    st.fields
        .iter()
        .map(|f| f.ident.as_ref())
        .enumerate()
        .map(|(i, f)| f.map_or_else(|| syn::Index::from(i).to_token_stream(), |f| quote!(#f)))
        .collect()
}
