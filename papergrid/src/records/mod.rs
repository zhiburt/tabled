//! The module contains a [Records] abstraction of a [`Grid`] trait and its implementers.
//!
//! [`Grid`]: crate::grid::spanned::Grid

mod into_records;
mod iter_records;

pub use into_records::IntoRecords;
pub use iter_records::IterRecords;

/// Records represents table data.
pub trait Records {
    /// Iterator which goes over rows.
    type Iter: IntoRecords;

    /// Returns a iterator over rows.
    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows;

    /// Returns count of columns in the records.
    fn count_columns(&self) -> usize;

    /// Hint amount of rows in the records.
    fn hint_count_rows(&self) -> Option<usize>;
}
