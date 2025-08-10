use super::{IntoRecords, Records};

/// A [Records] implementation for any [IntoIterator].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IterRecords<I> {
    iter: I,
    count_columns: usize,
    count_rows: Option<usize>,
}

impl<I> IterRecords<I> {
    /// Returns a new [IterRecords] object.
    pub const fn new(iter: I, count_columns: usize, count_rows: Option<usize>) -> Self {
        Self {
            iter,
            count_columns,
            count_rows,
        }
    }
}

impl<I> IntoRecords for IterRecords<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = I::IterColumns;
    type IterRows = I::IterRows;

    fn iter_rows(self) -> Self::IterRows {
        self.iter.iter_rows()
    }
}

impl<I> Records for IterRecords<I>
where
    I: IntoRecords,
{
    type Iter = I;

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        self.iter.iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.count_columns
    }

    fn hint_count_rows(&self) -> Option<usize> {
        self.count_rows
    }
}

impl<'a, I> Records for &'a IterRecords<I>
where
    &'a I: IntoRecords,
{
    type Iter = &'a I;

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        (&self.iter).iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.count_columns
    }

    fn hint_count_rows(&self) -> Option<usize> {
        self.count_rows
    }
}
