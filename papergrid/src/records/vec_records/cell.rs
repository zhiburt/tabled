/// Cell implementation which can be used with [`VecRecords`].
///
/// [`VecRecords`]: crate::records::vec_records::VecRecords
pub trait Cell {
    /// Gets a text.
    fn text(&self) -> &str;

    /// Gets a line by index.
    fn line(&self, line: usize) -> &str;

    /// Returns a number of lines cell has.
    fn count_lines(&self) -> usize;

    /// Returns a width of cell.
    fn width(&self) -> usize;

    /// Returns a width of cell line.
    fn line_width(&self, line: usize) -> usize;
}

impl<T> Cell for &T
where
    T: Cell,
{
    fn text(&self) -> &str {
        T::text(self)
    }

    fn line(&self, line: usize) -> &str {
        T::line(self, line)
    }

    fn count_lines(&self) -> usize {
        T::count_lines(self)
    }

    fn width(&self) -> usize {
        T::width(self)
    }

    fn line_width(&self, line: usize) -> usize {
        T::line_width(self, line)
    }
}
