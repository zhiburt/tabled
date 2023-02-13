use std::iter::FromIterator;

use crate::{
    grid::spanned::config::GridConfig, records::Records, settings::TableOption,
    tables::table::TableDimension,
};

/// A structure used to set [`Table`] width via a list of columns widths.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct WidthList {
    list: Vec<usize>,
}

impl WidthList {
    /// Creates a new object.
    pub fn new(list: Vec<usize>) -> Self {
        Self { list }
    }
}

impl From<Vec<usize>> for WidthList {
    fn from(list: Vec<usize>) -> Self {
        Self::new(list)
    }
}

impl FromIterator<usize> for WidthList {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<R> TableOption<R, TableDimension<'static>, GridConfig> for WidthList
where
    R: Records,
{
    fn change(
        &mut self,
        records: &mut R,
        _: &mut GridConfig,
        dimension: &mut TableDimension<'static>,
    ) {
        if self.list.len() < records.count_columns() {
            return;
        }

        dimension.set_widths(self.list.clone());
    }
}
