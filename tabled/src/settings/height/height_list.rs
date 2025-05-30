use std::iter::FromIterator;

use crate::{
    grid::dimension::CompleteDimension,
    grid::records::{ExactRecords, Records},
    settings::TableOption,
};

/// A structure used to set [`Table`] height via a list of rows heights.
///
/// [`Table`]: crate::Table
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

impl<R, C> TableOption<R, C, CompleteDimension> for HeightList
where
    R: ExactRecords + Records,
{
    fn change(self, records: &mut R, _: &mut C, dims: &mut CompleteDimension) {
        if self.list.len() < records.count_rows() {
            return;
        }

        dims.set_heights(self.list);
    }

    fn hint_change(&self) -> Option<papergrid::config::Entity> {
        // NOTE: is this correct?
        None
    }
}
