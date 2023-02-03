//! The module contains a [Records] abstraction of a [`Grid`] trait and its implementers.
//!
//! [`Grid`]: crate::Grid

mod into_records;
mod iter_records;

pub use into_records::IntoRecords;
pub use iter_records::IterRecords;

pub trait Records {
    type Iter: IntoRecords;

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows;

    fn count_columns(&self) -> usize;

    fn hint_count_rows(&self) -> Option<usize>;
}
