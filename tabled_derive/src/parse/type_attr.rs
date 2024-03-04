use proc_macro2::{Ident, Span};
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, token, Attribute, LitBool, LitStr, Token,
};

pub fn parse_type_attributes(
    attributes: &[Attribute],
) -> impl Iterator<Item = syn::Result<impl Iterator<Item = TypeAttr>>> + '_ {
    attributes
        .iter()
        .filter(|attr| attr.path.is_ident("tabled"))
        .map(|attr| attr.parse_args_with(Punctuated::<TypeAttr, Token![,]>::parse_terminated))
        .map(|result| result.map(IntoIterator::into_iter))
}

pub struct TypeAttr {
    pub ident: Ident,
    pub kind: TypeAttrKind,
}

impl TypeAttr {
    pub fn new(ident: Ident, kind: TypeAttrKind) -> Self {
        Self { ident, kind }
    }
}

#[derive(Clone)]
pub enum TypeAttrKind {
    Inline(LitBool, Option<LitStr>),
    RenameAll(LitStr),
    Crate(LitStr),
}

impl Parse for TypeAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use TypeAttrKind::*;

        if input.peek(syn::token::Crate) {
            let crate_token: syn::token::Crate = input.parse()?;
            let _ = input.parse::<Token![=]>()?;
            let value = input.parse::<LitStr>()?;

            return Ok(Self::new(
                Ident::new("crate", crate_token.span),
                Crate(value),
            ));
        }

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                let lit = input.parse::<LitStr>()?;

                if let "rename_all" = name_str.as_str() {
                    return Ok(Self::new(name, RenameAll(lit)));
                }
            }

            if input.peek(LitBool) {
                let lit = input.parse::<LitBool>()?;

                if let "inline" = name_str.as_str() {
                    return Ok(Self::new(name, Inline(lit, None)));
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

                if let "inline" = name_str.as_str() {
                    return Ok(Self::new(
                        name,
                        Inline(LitBool::new(true, Span::call_site()), Some(lit)),
                    ));
                }
            }

            return Err(syn::Error::new(
                _paren.span,
                "expected a `string literal` in parenthesis",
            ));
        }

        if let "inline" = name_str.as_str() {
            return Ok(Self::new(
                name,
                Inline(LitBool::new(true, Span::call_site()), None),
            ));
        }

        Err(syn::Error::new(
            name.span(),
            format!("unexpected attribute: {name_str}"),
        ))
    }
}
