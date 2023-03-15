//! The module contains [`LimitRows`] records iterator.

use crate::grid::records::IntoRecords;

/// [`LimitRows`] is an records iterator which limits amount of rows.
#[derive(Debug)]
pub struct LimitRows<I> {
    records: I,
    limit: usize,
}

impl LimitRows<()> {
    /// Creates new [`LimitRows`] iterator.
    pub fn new<I: IntoRecords>(records: I, limit: usize) -> LimitRows<I> {
        LimitRows { records, limit }
    }
}

impl<I> IntoRecords for LimitRows<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = I::IterColumns;
    type IterRows = LimitRowsIter<<I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        LimitRowsIter {
            iter: self.records.iter_rows().into_iter(),
            limit: self.limit,
        }
    }
}

/// A rows iterator for [`LimitRows`]
#[derive(Debug)]
pub struct LimitRowsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for LimitRowsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }

        self.limit -= 1;

        self.iter.next()
    }
}
