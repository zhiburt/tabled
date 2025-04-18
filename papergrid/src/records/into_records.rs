/// The representation of data, rows and columns of a grid.
pub trait IntoRecords {
    /// A string representation of a grid cell.
    type Cell;

    /// Cell iterator inside a row.
    type IterColumns: IntoIterator<Item = Self::Cell>;

    /// Rows iterator.
    type IterRows: IntoIterator<Item = Self::IterColumns>;

    /// Returns an iterator over rows.
    fn iter_rows(self) -> Self::IterRows;
}

impl<T> IntoRecords for T
where
    T: IntoIterator,
    <T as IntoIterator>::Item: IntoIterator,
{
    type Cell = <<T as IntoIterator>::Item as IntoIterator>::Item;
    type IterColumns = <T as IntoIterator>::Item;
    type IterRows = <T as IntoIterator>::IntoIter;

    fn iter_rows(self) -> Self::IterRows {
        self.into_iter()
    }
}
