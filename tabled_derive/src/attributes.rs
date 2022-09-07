use syn::{Attribute, LitInt};

use crate::{casing_style::CasingStyle, error::Error, parse};

#[derive(Debug, Default)]
pub struct Attributes {
    pub is_ignored: bool,
    pub inline: bool,
    pub inline_prefix: Option<String>,
    pub rename: Option<String>,
    pub rename_all: Option<CasingStyle>,
    pub display_with: Option<String>,
    pub display_with_use_self: bool,
    pub order: Option<usize>,
}

impl Attributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut attributes = Self::default();
        attributes.fill_attributes(attrs)?;

        Ok(attributes)
    }

    fn fill_attributes(&mut self, attrs: &[Attribute]) -> Result<(), Error> {
        for attrs in parse::parse_attributes(attrs) {
            let attrs = attrs?;
            for attr in attrs {
                self.insert_attribute(attr)?;
            }
        }

        Ok(())
    }

    fn insert_attribute(&mut self, attr: parse::TabledAttr) -> Result<(), Error> {
        match attr.kind {
            parse::TabledAttrKind::Skip(b) => {
                if b.value {
                    self.is_ignored = true;
                }
            }
            parse::TabledAttrKind::Inline(b, prefix) => {
                if b.value {
                    self.inline = true;
                }

                if let Some(prefix) = prefix {
                    self.inline_prefix = Some(prefix.value());
                }
            }
            parse::TabledAttrKind::Rename(value) => self.rename = Some(value.value()),
            parse::TabledAttrKind::RenameAll(lit) => {
                self.rename_all = Some(CasingStyle::from_lit(&lit)?);
            }
            parse::TabledAttrKind::DisplayWith(path, use_self) => {
                self.display_with = Some(path.value());
                self.display_with_use_self = use_self;
            }
            parse::TabledAttrKind::Order(value) => self.order = Some(lit_int_to_usize(&value)?),
        }

        Ok(())
    }

    pub fn is_ignored(&self) -> bool {
        self.is_ignored
    }
}

pub struct ObjectAttributes {
    pub rename_all: Option<CasingStyle>,
}

impl ObjectAttributes {
    pub fn parse(attrs: &[Attribute]) -> Result<Self, Error> {
        let attrs = Attributes::parse(attrs)?;
        Ok(Self {
            rename_all: attrs.rename_all,
        })
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
