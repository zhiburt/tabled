use std::iter::FromIterator;

use papergrid::records::Records;

use crate::{Table, TableOption};

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

impl<R> TableOption<R> for HeightList
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if self.list.len() < table.count_rows() {
            return;
        }

        table.cache_height(self.list.clone());
        table.destroy_width_cache();
    }
}
