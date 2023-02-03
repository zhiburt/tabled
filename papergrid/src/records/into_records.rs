/// The representation of data, rows and columns of a [`Grid`].
///
/// [`Grid`]: crate::Grid
pub trait IntoRecords {
    type Cell: AsRef<str>;
    type IterColumns: IntoIterator<Item = Self::Cell>;
    type IterRows: IntoIterator<Item = Self::IterColumns>;

    /// Returns an iterator over rows.
    fn iter_rows(self) -> Self::IterRows;
}

impl<T> IntoRecords for T
where
    T: IntoIterator,
    <T as IntoIterator>::Item: IntoIterator,
    <<T as IntoIterator>::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = <<T as IntoIterator>::Item as IntoIterator>::Item;
    type IterColumns = <T as IntoIterator>::Item;
    type IterRows = <T as IntoIterator>::IntoIter;

    fn iter_rows(self) -> Self::IterRows {
        self.into_iter()
    }
}
