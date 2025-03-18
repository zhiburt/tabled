use syn::{Attribute, TypePath};

use crate::{
    attributes::FormatArg,
    casing_style::CasingStyle,
    error::Error,
    parse::type_attr::{parse_type_attributes, TypeAttr, TypeAttrKind},
};

#[derive(Default)]
pub struct TypeAttributes {
    pub rename_all: Option<CasingStyle>,
    pub inline: bool,
    pub inline_value: Option<String>,
    pub crate_name: Option<String>,
    pub display_types: Vec<(TypePath, String, Vec<FormatArg>)>,
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
            TypeAttrKind::DisplayType(type_name, func, args) => {
                let args = args.into_iter().map(FormatArg::new).collect();
                self.display_types.push((type_name, func.value(), args));
            }
        }

        Ok(())
    }
}
