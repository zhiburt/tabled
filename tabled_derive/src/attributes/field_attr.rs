use syn::{Attribute, LitInt};

use crate::{
    casing_style::CasingStyle,
    error::Error,
    parse::field_attr::{parse_field_attributes, FieldAttr, FieldAttrKind},
    parse::func_arg::FuncArg,
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
                        .map(|lit| FuncArg::parse_expr(&lit))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.display_with_args = Some(args);
                }
            }
            FieldAttrKind::FormatWith(format, comma, args) => {
                self.format = Some(format.value());
                if comma.is_some() {
                    let args = args
                        .into_iter()
                        .map(|lit| FuncArg::parse_expr(&lit))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.format_with_args = Some(args);
                }
            }
            FieldAttrKind::Order(value) => self.order = Some(lit_int_to_usize(&value)?),
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
