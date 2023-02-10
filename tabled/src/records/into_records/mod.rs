//! The module contains a list of helpers for [`IntoRecords`]
//!
//! [`IntoRecords`]: crate::records::IntoRecords

pub mod buf_records;
pub mod either_string;
pub mod limit_column_records;
pub mod limit_row_records;
pub mod truncate_records;

pub use buf_records::{BufColumns, BufRows};
pub use limit_column_records::LimitColumns;
pub use limit_row_records::LimitRows;
pub use truncate_records::TruncateContent;
