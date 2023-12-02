use std::borrow::Cow;

use crate::grid::{
    config::{ColoredConfig, SpannedConfig},
    dimension::{Dimension, Estimate, SpannedVecRecordsDimension},
    records::vec_records::{Cell, VecRecords},
};

/// CompleteDimension is a [`Dimension`] implementation for a [`Table`]
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CompleteDimensionVecRecords<'a> {
    width: Option<Cow<'a, [usize]>>,
    height: Option<Cow<'a, [usize]>>,
}

impl CompleteDimensionVecRecords<'_> {
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
    pub fn set_widths(&mut self, columns: Vec<usize>) {
        self.width = Some(Cow::Owned(columns));
    }

    /// Get column widths.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided widths.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn get_widths(&self) -> Option<&'_ [usize]> {
        self.width.as_deref()
    }

    /// Set rows heights.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided heights.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn set_heights(&mut self, rows: Vec<usize>) {
        self.height = Some(Cow::Owned(rows));
    }

    /// Get row heights.
    ///
    /// In general the method is only considered to be useful to a [`TableOption`].
    ///
    /// BE CAREFUL WITH THIS METHOD as it supposed that the content is not bigger than the provided widths.
    ///
    /// [`TableOption`]: crate::settings::TableOption
    pub fn get_heights(&self) -> Option<&'_ [usize]> {
        self.height.as_deref()
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
    pub fn from_origin(&self) -> CompleteDimensionVecRecords<'_> {
        let width = self.width.as_deref().map(Cow::Borrowed);
        let height = self.height.as_deref().map(Cow::Borrowed);

        CompleteDimensionVecRecords { width, height }
    }

    /// Copies a reference from self.
    pub fn into_inner(self) -> (Option<Vec<usize>>, Option<Vec<usize>>) {
        let width = self.width.map(|list| list.into_owned());
        let height = self.height.map(|list| list.into_owned());

        (width, height)
    }
}

impl Dimension for CompleteDimensionVecRecords<'_> {
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

impl<T: AsRef<str> + Cell> Estimate<&VecRecords<T>, SpannedConfig>
    for CompleteDimensionVecRecords<'_>
{
    fn estimate(&mut self, records: &VecRecords<T>, cfg: &SpannedConfig) {
        match (self.width.is_some(), self.height.is_some()) {
            (true, true) => {}
            (true, false) => {
                self.height = Some(Cow::Owned(SpannedVecRecordsDimension::height(records, cfg)));
            }
            (false, true) => {
                self.width = Some(Cow::Owned(SpannedVecRecordsDimension::width(records, cfg)));
            }
            (false, false) => {
                let mut dims = SpannedVecRecordsDimension::default();
                dims.estimate(records, cfg);

                let (width, height) = dims.get_values();
                self.width = Some(Cow::Owned(width));
                self.height = Some(Cow::Owned(height));
            }
        }
    }
}

impl<T: AsRef<str> + Cell> Estimate<&VecRecords<T>, ColoredConfig>
    for CompleteDimensionVecRecords<'_>
{
    fn estimate(&mut self, records: &VecRecords<T>, cfg: &ColoredConfig) {
        self.estimate(records, cfg.as_ref())
    }
}
