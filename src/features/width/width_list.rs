use std::iter::FromIterator;

use papergrid::records::Records;

use crate::{Table, TableOption};

/// A structure used to set [`Table`] width via a list of columns widths.
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
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = usize>,
    {
        Self::new(iter.into_iter().collect())
    }
}

impl<R> TableOption<R> for WidthList
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if self.list.len() < table.count_columns() {
            return;
        }

        table.cache_width(self.list.clone());
        table.destroy_height_cache();
    }
}
