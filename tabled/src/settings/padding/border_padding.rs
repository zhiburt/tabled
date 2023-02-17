use papergrid::grid::compact::CompactConfig;

use crate::{
    grid::config::{Entity, Indent, Sides},
    settings::TableOption,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
use crate::grid::spanned::config::GridConfig;

/// Padding is responsible for a left/right/top/bottom inner indent of a particular cell.
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// # use tabled::{settings::{style::Style, padding::Padding, object::Rows, Modify}, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data).with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).fill('>', '<', '^', 'V')));
/// ```
#[derive(Debug)]
pub struct Padding(Sides<Indent>);

impl Padding {
    /// Construct's an Padding object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Padding::fill`] function.
    pub const fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        let indent = Indent::spaced;
        Self(Sides::new(
            indent(left),
            indent(right),
            indent(top),
            indent(bottom),
        ))
    }

    /// Construct's an Padding object with all sides set to 0.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Padding::fill`] function.
    pub const fn zero() -> Self {
        let indent = Indent::spaced(0);
        Self(Sides::new(indent, indent, indent, indent))
    }

    /// The function, sets a characters for the padding on an each side.
    pub const fn fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.0.left.fill = left;
        self.0.right.fill = right;
        self.0.top.fill = top;
        self.0.bottom.fill = bottom;
        self
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<R> crate::settings::CellOption<R, GridConfig> for Padding {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, entity: Entity) {
        cfg.set_padding(entity, self.0);
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<R, D> TableOption<R, D, GridConfig> for Padding {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        cfg.set_padding(Entity::Global, self.0);
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for Padding {
    fn change(&mut self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_padding(self.0);
    }
}
