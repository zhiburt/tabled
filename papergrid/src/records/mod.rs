//! The module contains a [Records] abstraction of a [`Grid`] trait and its implementers.
//!
//! [`Grid`]: crate::grid::iterable::Grid

mod exact_records;
mod into_records;
mod iter_records;
mod peekable_records;

pub use exact_records::ExactRecords;
pub use into_records::IntoRecords;
pub use iter_records::IterRecords;
pub use peekable_records::PeekableRecords;

#[cfg(feature = "std")]
pub mod vec_records;

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

// todo: Provide a convenient way to iter over columns
//
// probably fn iter_columns(self) -> Option<Self::Iter2>
//
// it'll likely speed up some algos

// note:
// Maybe simplify IntoRecords; we know count columns any way....
// and sometimes buffering and stuff hard to implement with this laye of abstraction
