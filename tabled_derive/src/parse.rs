use proc_macro2::{Ident, Span};
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, token, Attribute, LitBool, LitInt, LitStr,
    Token,
};

pub fn parse_attributes(
    attributes: &[Attribute],
) -> impl Iterator<Item = syn::Result<impl Iterator<Item = TabledAttr>>> + '_ {
    attributes
        .iter()
        .filter(|attr| attr.path.is_ident("tabled"))
        .map(|attr| attr.parse_args_with(Punctuated::<TabledAttr, Token![,]>::parse_terminated))
        .map(|result| result.map(IntoIterator::into_iter))
}

pub struct TabledAttr {
    pub ident: Ident,
    pub kind: TabledAttrKind,
}

impl TabledAttr {
    pub fn new(ident: Ident, kind: TabledAttrKind) -> Self {
        Self { ident, kind }
    }
}

#[derive(Clone)]
pub enum TabledAttrKind {
    Skip(LitBool),
    Inline(LitBool, Option<LitStr>),
    Rename(LitStr),
    RenameAll(LitStr),
    DisplayWith(LitStr, bool),
    Order(LitInt),
}

impl Parse for TabledAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use TabledAttrKind::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                let lit = input.parse::<LitStr>()?;

                match name_str.as_str() {
                    "rename" => return Ok(Self::new(name, Rename(lit))),
                    "rename_all" => return Ok(Self::new(name, RenameAll(lit))),
                    "display_with" => return Ok(Self::new(name, DisplayWith(lit, false))),
                    _ => {}
                }
            }

            if input.peek(LitBool) {
                let lit = input.parse::<LitBool>()?;

                match name_str.as_str() {
                    "skip" => return Ok(Self::new(name, Skip(lit))),
                    "inline" => return Ok(Self::new(name, Inline(lit, None))),
                    _ => {}
                }
            }

            if input.peek(LitInt) {
                let lit = input.parse::<LitInt>()?;

                if let "order" = name_str.as_str() {
                    return Ok(Self::new(name, Order(lit)));
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

                match name_str.as_str() {
                    "display_with" => {
                        let use_self = if nested.peek(Token![,]) {
                            let _comma = nested.parse::<Token![,]>()?;
                            if nested.peek(syn::Ident) {
                                let ident = nested.parse::<syn::Ident>()?;
                                ident == "args"
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        return Ok(Self::new(name, DisplayWith(lit, use_self)));
                    }
                    "inline" => {
                        return Ok(Self::new(
                            name,
                            Inline(LitBool::new(true, Span::call_site()), Some(lit)),
                        ))
                    }
                    _ => {}
                }
            }

            return Err(syn::Error::new(
                _paren.span,
                "expected a `string literal` in parenthesis",
            ));
        }

        match name_str.as_str() {
            "skip" => return Ok(Self::new(name, Skip(LitBool::new(true, Span::call_site())))),
            "inline" => {
                return Ok(Self::new(
                    name,
                    Inline(LitBool::new(true, Span::call_site()), None),
                ))
            }
            _ => {}
        }

        Err(syn::Error::new(
            name.span(),
            format!("unexpected attribute: {name_str}"),
        ))
    }
}
