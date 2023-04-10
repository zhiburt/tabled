/// [`Records`] extension which guarantees the amount of rows.
///
/// [`Records`]: crate::records::Records
pub trait ExactRecords {
    /// Returns an exact amount of rows in records.
    ///
    /// It must be guaranteed that an iterator will yield this amount.
    fn count_rows(&self) -> usize;
}

impl<T> ExactRecords for &T
where
    T: ExactRecords,
{
    fn count_rows(&self) -> usize {
        T::count_rows(self)
    }
}

#[cfg(feature = "std")]
impl<T> ExactRecords for Vec<T> {
    fn count_rows(&self) -> usize {
        self.len()
    }
}

impl<T> ExactRecords for [T] {
    fn count_rows(&self) -> usize {
        self.len()
    }
}
