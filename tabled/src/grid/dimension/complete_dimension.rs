use std::borrow::Cow;

use crate::grid::{
    config::{ColoredConfig, SpannedConfig},
    dimension::{Dimension, Estimate, SpannedGridDimension},
    records::Records,
};

/// CompleteDimension is a [`Dimension`] implementation for a [`Table`]
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CompleteDimension<'a> {
    width: Option<Cow<'a, [usize]>>,
    height: Option<Cow<'a, [usize]>>,
}

impl CompleteDimension<'_> {
    /// Checks whether is the dimensions is set.
    pub fn is_complete(&self) -> bool {
        self.width.is_some() && self.height.is_some()
    }

    /// Checks whether is nothing was set.
    pub fn is_empty(&self) -> bool {
        self.width.is_none() && self.height.is_none()
    }

    /// Set column widths.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
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
    /// In general the method is only considered to be useful to a [`TableOption`].
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
    pub fn from_origin(&self) -> CompleteDimension<'_> {
        let width = self.width.as_deref().map(Cow::Borrowed);
        let height = self.height.as_deref().map(Cow::Borrowed);

        CompleteDimension { width, height }
    }
}

impl Dimension for CompleteDimension<'_> {
    fn get_width(&self, column: usize) -> usize {
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

impl<R: Records> Estimate<R, SpannedConfig> for CompleteDimension<'_> {
    fn estimate(&mut self, records: R, cfg: &SpannedConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(Cow::Owned(SpannedGridDimension::height(records, cfg)));
            }
            (false, true) => {
                self.width = Some(Cow::Owned(SpannedGridDimension::width(records, cfg)));
            }
            (false, false) => {
                let mut dims = SpannedGridDimension::default();
                dims.estimate(records, cfg);

                let (width, height) = dims.get_values();
                self.width = Some(Cow::Owned(width));
                self.height = Some(Cow::Owned(height));
            }
        }
    }
}

impl<R: Records> Estimate<R, ColoredConfig> for CompleteDimension<'_> {
    fn estimate(&mut self, records: R, cfg: &ColoredConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(Cow::Owned(SpannedGridDimension::height(records, cfg)));
            }
            (false, true) => {
                self.width = Some(Cow::Owned(SpannedGridDimension::width(records, cfg)));
            }
            (false, false) => {
                let mut dims = SpannedGridDimension::default();
                dims.estimate(records, cfg);

                let (width, height) = dims.get_values();
                self.width = Some(Cow::Owned(width));
                self.height = Some(Cow::Owned(height));
            }
        }
    }
}
