use crate::{
    grid::config::{Entity, GridConfig, Indent, Padding as GridPadding},
    settings::{CellOption, TableOption},
};

// #[cfg(feature = "color")]
// use crate::style::Color;

/// Padding is responsible for a left/right/top/bottom inner indent of a particular cell.
///
/// ```rust,no_run
/// # use tabled::{Style, Padding, object::Rows, Table, Modify};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data).with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).set_fill('>', '<', '^', 'V')));
/// ```
#[derive(Debug)]
pub struct Padding(GridPadding);

impl Padding {
    /// Construct's an Padding object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Self::set_fill`] function.
    pub const fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        let indent = Indent::spaced;
        Self(GridPadding::new(
            indent(left),
            indent(right),
            indent(top),
            indent(bottom),
        ))
    }

    /// Construct's an Padding object with all sides set to 0.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Self::set_fill`] function.
    pub const fn zero() -> Self {
        let indent = Indent::spaced(0);
        Self(GridPadding::new(indent, indent, indent, indent))
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

impl<R> CellOption<R> for Padding {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        cfg.set_padding(entity, self.0);
    }
}

impl<R, D> TableOption<R, D> for Padding {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        cfg.set_padding(Entity::Global, self.0);
    }
}
