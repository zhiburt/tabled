//! The module contains [`Records`], [`ExactRecords`], [`RecordsMut`], [`Resizable`] traits
//! and its implementations.
//!
//! Also it provies a list of helpers for a user built [`Records`] via [`into_records`].

mod empty_records;
mod records_mut;
mod resizable;

pub mod into_records;

pub use empty_records::EmptyRecords;
pub use papergrid::records::{ExactRecords, IntoRecords, IterRecords, PeekableRecords, Records};
pub use records_mut::RecordsMut;
pub use resizable::Resizable;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::records::vec_records;
