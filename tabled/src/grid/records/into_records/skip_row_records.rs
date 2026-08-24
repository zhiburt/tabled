//! The module contains [`SkipRows`] records iterator.

use crate::grid::records::IntoRecords;

/// [`SkipRows`] is an records iterator which skips a number of rows from the top.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkipRows<I> {
    records: I,
    skip: usize,
}

impl SkipRows<()> {
    /// Creates new [`SkipRows`] iterator.
    pub fn new<I: IntoRecords>(records: I, skip: usize) -> SkipRows<I> {
        SkipRows { records, skip }
    }
}

impl<I> IntoRecords for SkipRows<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = I::IterColumns;
    type IterRows = SkipRowsIter<<I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        SkipRowsIter {
            iter: self.records.iter_rows().into_iter(),
            skip: self.skip,
        }
    }
}

/// A rows iterator for [`SkipRows`]
#[derive(Debug)]
pub struct SkipRowsIter<I> {
    iter: I,
    skip: usize,
}

impl<I> Iterator for SkipRowsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Drop the leading rows lazily on the first pull, then pass through.
        while self.skip > 0 {
            self.skip -= 1;
            let _ = self.iter.next()?;
        }

        self.iter.next()
    }
}
