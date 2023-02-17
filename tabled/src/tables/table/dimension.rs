use std::borrow::Cow;

use papergrid::{
    dimension::{Dimension, Estimate},
    grid::spanned::{ExactDimension, GridConfig},
    records::Records,
};

/// TableDimension is a [`Dimension`] implementation for a [`Table`]
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, Clone)]
pub struct TableDimension<'a> {
    width: Option<Cow<'a, [usize]>>,
    height: Option<Cow<'a, [usize]>>,
}

impl TableDimension<'_> {
    /// Checks whether is the dimensions is set.
    pub fn is_complete(&self) -> bool {
        self.width.is_some() && self.height.is_some()
    }

    /// Set column widths.
    ///
    /// In general the method is only considered to be usefull to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided widths.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn set_widths(&mut self, columns: Vec<usize>) -> bool {
        self.width = Some(Cow::Owned(columns));

        true
    }

    /// Set rows heights.
    ///
    /// In general the method is only considered to be usefull to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided heights.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn set_heights(&mut self, rows: Vec<usize>) -> bool {
        self.height = Some(Cow::Owned(rows));

        true
    }

    /// Force width estimation.
    pub fn clear_width(&mut self) {
        self.width = None;
    }

    /// Force height estimation.
    pub fn clear_height(&mut self) {
        self.height = None;
    }

    /// Copies a reference from self.
    pub fn from_origin(&self) -> TableDimension<'_> {
        let width = self.width.as_deref().map(Cow::Borrowed);
        let height = self.height.as_deref().map(Cow::Borrowed);

        TableDimension { width, height }
    }
}

impl Dimension for TableDimension<'_> {
    fn get_width(&self, column: usize) -> usize {
        println!("xxx {:?}", self);

        let width = self
            .width
            .as_ref()
            .expect("It must always be Some at this point");

        width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        let height = self
            .height
            .as_ref()
            .expect("It must always be Some at this point");

        height[row]
    }
}

impl Estimate<GridConfig> for TableDimension<'_> {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(Cow::Owned(ExactDimension::height(records, cfg)));
            }
            (false, true) => {
                self.width = Some(Cow::Owned(ExactDimension::width(records, cfg)));
            }
            (false, false) => {
                let mut dims = ExactDimension::default();
                dims.estimate(records, cfg);

                let (width, height) = dims.get_values();
                self.width = Some(Cow::Owned(width));
                self.height = Some(Cow::Owned(height));
            }
        }
    }
}
