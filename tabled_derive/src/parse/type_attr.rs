use proc_macro2::{Ident, Span};
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, spanned::Spanned, token, Attribute,
    LitBool, LitStr, Token, TypePath,
};

pub fn parse_type_attributes(
    attributes: &[Attribute],
) -> impl Iterator<Item = syn::Result<impl Iterator<Item = TypeAttr>>> + '_ {
    attributes
        .iter()
        .filter(|attr| attr.path().is_ident("tabled"))
        .map(|attr| attr.parse_args_with(Punctuated::<TypeAttr, Token![,]>::parse_terminated))
        .map(|result| result.map(IntoIterator::into_iter))
}

pub struct TypeAttr {
    pub kind: TypeAttrKind,
}

impl TypeAttr {
    pub fn new(kind: TypeAttrKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone)]
pub enum TypeAttrKind {
    Inline(LitBool, Option<LitStr>),
    RenameAll(LitStr),
    Crate(LitStr),
    DisplayType(TypePath, LitStr, Punctuated<syn::Expr, Token!(,)>),
}

impl Parse for TypeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use TypeAttrKind::*;

        if input.peek(syn::token::Crate) {
            let _: syn::token::Crate = input.parse()?;
            let _ = input.parse::<Token![=]>()?;
            let value = input.parse::<LitStr>()?;

            return Ok(Self::new(Crate(value)));
        }

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                let lit = input.parse::<LitStr>()?;

                if name_str.as_str() == "rename_all" {
                    return Ok(Self::new(RenameAll(lit)));
                }
            }

            if input.peek(LitBool) {
                let lit = input.parse::<LitBool>()?;

                if name_str.as_str() == "inline" {
                    return Ok(Self::new(Inline(lit, None)));
                }
            }

            return Err(syn::Error::new(
                assign_token.span,
                "expected `string literal` or `expression` after `=`",
            ));
        }

        if input.peek(token::Paren) {
            let nested;
            let _paren = parenthesized!(nested in input);

            if nested.peek(LitStr) {
                let lit = nested.parse::<LitStr>()?;

                if name_str.as_str() == "inline" {
                    return Ok(Self::new(Inline(
                        LitBool::new(true, Span::call_site()),
                        Some(lit),
                    )));
                }
            }

            if name_str.as_str() == "display_type" {
                let path = nested.parse::<TypePath>()?;
                let _comma = nested.parse::<Token![,]>()?;
                let lit = nested.parse::<LitStr>()?;

                let mut args: Punctuated<syn::Expr, token::Comma> = Punctuated::new();
                if nested.peek(Token![,]) {
                    _ = nested.parse::<Token![,]>()?;
                    while !nested.is_empty() {
                        let val = nested.parse()?;
                        args.push_value(val);
                        if nested.is_empty() {
                            break;
                        }
                        let punct = nested.parse()?;
                        args.push_punct(punct);
                    }
                }

                return Ok(Self::new(DisplayType(path, lit, args)));
            }

            return Err(syn::Error::new(
                _paren.span.span(),
                "expected a `string literal` in parenthesis",
            ));
        }

        if name_str.as_str() == "inline" {
            return Ok(Self::new(Inline(
                LitBool::new(true, Span::call_site()),
                None,
            )));
        }

        Err(syn::Error::new(
            name.span(),
            format!("unexpected attribute: {name_str}"),
        ))
    }
}
