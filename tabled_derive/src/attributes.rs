use syn::{Attribute, Lit, LitInt};

use crate::{
    casing_style::CasingStyle,
    error::Error,
    parse::{
        field_attr::{parse_field_attributes, FieldAttr, FieldAttrKind},
        type_attr::{parse_type_attributes, TypeAttr, TypeAttrKind},
    },
};

#[derive(Debug, Default)]
pub struct FieldAttributes {
    pub is_ignored: bool,
    pub inline: bool,
    pub inline_prefix: Option<String>,
    pub rename: Option<String>,
    pub rename_all: Option<CasingStyle>,
    pub display_with: Option<String>,
    pub display_with_args: Option<Vec<FuncArg>>,
    pub order: Option<usize>,
    pub format: Option<String>,
    pub format_with_args: Option<Vec<FuncArg>>,
}

impl FieldAttributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
    }

    fn fill_attributes(&mut self, attrs: &[Attribute]) -> Result<(), Error> {
        for attrs in parse_field_attributes(attrs) {
            let attrs = attrs?;
            for attr in attrs {
                self.insert_attribute(attr)?;
            }
        }

        Ok(())
    }

    fn insert_attribute(&mut self, attr: FieldAttr) -> Result<(), Error> {
        match attr.kind {
            FieldAttrKind::Skip(b) => {
                if b.value {
                    self.is_ignored = true;
                }
            }
            FieldAttrKind::Inline(b, prefix) => {
                if b.value {
                    self.inline = true;
                }

                if let Some(prefix) = prefix {
                    self.inline_prefix = Some(prefix.value());
                }
            }
            FieldAttrKind::Rename(value) => self.rename = Some(value.value()),
            FieldAttrKind::RenameAll(lit) => {
                self.rename_all = Some(CasingStyle::from_lit(&lit)?);
            }
            FieldAttrKind::DisplayWith(path, comma, args) => {
                self.display_with = Some(path.value());
                if comma.is_some() {
                    let args = args
                        .into_iter()
                        .map(|lit| parse_func_arg(&lit))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.display_with_args = Some(args);
                }
            }
            FieldAttrKind::FormatWith(format, comma, args) => {
                self.format = Some(format.value());
                if comma.is_some() {
                    let args = args
                        .into_iter()
                        .map(|lit| parse_func_arg(&lit))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.format_with_args = Some(args);
                }
            }
            FieldAttrKind::Order(value) => self.order = Some(lit_int_to_usize(&value)?),
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct TypeAttributes {
    pub rename_all: Option<CasingStyle>,
    pub inline: bool,
    pub inline_value: Option<String>,
    pub crate_name: Option<String>,
}

impl TypeAttributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
    }

    fn fill_attributes(&mut self, attrs: &[Attribute]) -> Result<(), Error> {
        for attrs in parse_type_attributes(attrs) {
            let attrs = attrs?;
            for attr in attrs {
                self.insert_attribute(attr)?;
            }
        }

        Ok(())
    }

    fn insert_attribute(&mut self, attr: TypeAttr) -> Result<(), Error> {
        match attr.kind {
            TypeAttrKind::Crate(crate_name) => {
                let name = crate_name.value();
                if !name.is_empty() {
                    self.crate_name = Some(name);
                }
            }
            TypeAttrKind::Inline(b, prefix) => {
                if b.value {
                    self.inline = true;
                }

                if let Some(prefix) = prefix {
                    self.inline_value = Some(prefix.value());
                }
            }
            TypeAttrKind::RenameAll(lit) => {
                self.rename_all = Some(CasingStyle::from_lit(&lit)?);
            }
        }

        Ok(())
    }
}

fn lit_int_to_usize(value: &LitInt) -> Result<usize, Error> {
    value.base10_parse::<usize>().map_err(|e| {
        Error::new(
            format!("Failed to parse {:?} as usize; {}", value.to_string(), e),
            value.span(),
            None,
        )
    })
}

#[derive(Debug)]
pub enum FuncArg {
    SelfRef,
    SelfProperty(String),
    Byte(u8),
    Char(char),
    Bool(bool),
    Uint(usize),
    Int(isize),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

fn parse_func_arg(expr: &syn::Expr) -> syn::Result<FuncArg> {
    use syn::spanned::Spanned;

    match expr {
        syn::Expr::Lit(lit) => match &lit.lit {
            Lit::Str(val) => Ok(FuncArg::String(val.value())),
            Lit::ByteStr(val) => Ok(FuncArg::Bytes(val.value())),
            Lit::Byte(val) => Ok(FuncArg::Byte(val.value())),
            Lit::Char(val) => Ok(FuncArg::Char(val.value())),
            Lit::Bool(val) => Ok(FuncArg::Bool(val.value())),
            Lit::Float(val) => val.base10_parse::<f64>().map(FuncArg::Float),
            Lit::Int(val) => {
                if val.base10_digits().starts_with('-') {
                    val.base10_parse::<isize>().map(FuncArg::Int)
                } else {
                    val.base10_parse::<usize>().map(FuncArg::Uint)
                }
            }
            Lit::Verbatim(val) => Err(syn::Error::new(val.span(), "unsupported argument")),
        },
        syn::Expr::Path(path) => {
            let indent = path.path.get_ident().map(|indent| indent.to_string());
            if matches!(indent.as_deref(), Some("self" | "Self")) {
                Ok(FuncArg::SelfRef)
            } else {
                Err(syn::Error::new(path.span(), "unsupported argument"))
            }
        }
        syn::Expr::Field(field) => match &field.member {
            syn::Member::Named(ident) => Ok(FuncArg::SelfProperty(ident.to_string())),
            syn::Member::Unnamed(index) => Ok(FuncArg::SelfProperty(index.index.to_string())),
        },
        expr => Err(syn::Error::new(expr.span(), "unsupported argument")),
    }
}
