use std::iter::FromIterator;

use papergrid::{records::Records, GridConfig};

use crate::{records::ExactRecords, table::general::TableDimension, TableOption};

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
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = usize>,
    {
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
