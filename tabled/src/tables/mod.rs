//! Module contains a list of table representatives.
//!
//! ## [`table`]
//!
//! A default table implementation.
//!
//! ## [`iter`]
//!
//! Just like [`table`] but it's API is a bit different to serve better in context
//! where there are a memory limits.
//!
//! ## [`extended`]
//!
//! Has a table which is useful for large amount of data.

mod compact;
mod util;

#[cfg(feature = "std")]
mod extended;
#[cfg(feature = "std")]
mod iter;
#[cfg(feature = "std")]
mod table;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use table::Table;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use iter::IterTable;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use extended::ExtendedTable;

pub use compact::CompactTable;
