use std::iter::FromIterator;

use crate::{
    grid::config::GridConfig,
    records::{ExactRecords, Records},
    settings::TableOption,
    tables::table::TableDimension,
};

/// A structure used to set [`Table`] height via a list of rows heights.
#[derive(Debug)]
pub struct HeightList {
    list: Vec<usize>,
}

impl HeightList {
    /// Creates a new object.
    pub fn new(list: Vec<usize>) -> Self {
        Self { list }
    }
}

impl From<Vec<usize>> for HeightList {
    fn from(list: Vec<usize>) -> Self {
        Self::new(list)
    }
}

impl FromIterator<usize> for HeightList {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<R> TableOption<R, TableDimension<'static>> for HeightList
where
    R: ExactRecords + Records,
{
    fn change(&mut self, records: &mut R, _: &mut GridConfig, dims: &mut TableDimension<'static>) {
        if self.list.len() < records.count_rows() {
            return;
        }

        dims.set_heights(self.list.clone());
    }
}
