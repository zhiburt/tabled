//! The module contains [`LimitColumns`] records iterator.

use crate::grid::records::IntoRecords;

/// An iterator which limits amount of columns.
#[derive(Debug)]
pub struct LimitColumns<I> {
    records: I,
    limit: usize,
}

impl LimitColumns<()> {
    /// Creates new [`LimitColumns`].
    pub fn new<I: IntoRecords>(records: I, limit: usize) -> LimitColumns<I> {
        LimitColumns { records, limit }
    }
}

impl<I> IntoRecords for LimitColumns<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = LimitColumnsColumnsIter<<I::IterColumns as IntoIterator>::IntoIter>;
    type IterRows = LimitColumnsIter<<I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        LimitColumnsIter {
            iter: self.records.iter_rows().into_iter(),
            limit: self.limit,
        }
    }
}

/// An iterator over rows for [`LimitColumns`].
#[derive(Debug)]
pub struct LimitColumnsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for LimitColumnsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = LimitColumnsColumnsIter<<I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(LimitColumnsColumnsIter {
            iter: iter.into_iter(),
            limit: self.limit,
        })
    }
}

/// An iterator over columns for [`LimitColumns`].
#[derive(Debug)]
pub struct LimitColumnsColumnsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for LimitColumnsColumnsIter<I>
where
    I: Iterator,
    I::Item: AsRef<str>,
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
