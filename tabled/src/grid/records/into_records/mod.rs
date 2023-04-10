//! The module contains a list of helpers for [`IntoRecords`]
//!
//! [`IntoRecords`]: crate::grid::records::IntoRecords

pub mod limit_column_records;
pub mod limit_row_records;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod buf_records;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod either_string;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod truncate_records;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use buf_records::{BufColumns, BufRows};
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use truncate_records::TruncateContent;

pub use limit_column_records::LimitColumns;
pub use limit_row_records::LimitRows;
