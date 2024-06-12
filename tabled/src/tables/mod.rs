//! Module contains a list of table representatives.
//!
//! ## [`Table`]
//!
//! A default table implementation.
//!
//! ## [`IterTable`]
//!
//! Just like [`Table`] but it's API is a bit different to serve better in context
//! where there is a memory limit.
//!
//! ## [`ExtendedTable`]
//!
//! It's a table which is useful for large amount of data.
//!
//! ## [`PoolTable`]
//!
//! A table with a greater control of a layout.

mod compact;
mod util;

#[cfg(feature = "std")]
mod extended;
#[cfg(feature = "std")]
mod iter;
#[cfg(feature = "std")]
mod table;
#[cfg(feature = "std")]
mod table_pool;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use table::Table;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use iter::IterTable;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use extended::ExtendedTable;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use table_pool::{PoolTable, TableValue};

pub use compact::CompactTable;

// todo: Create a PoolTable backend in papergrid with generics so it coulb be used differently
//       rather then with our own impl of dimension
//
// todo: Replace all usage of concrete configs to a AsRef<Config> generics, so some could be used interchangeably
//
// todo: Think about all the Config hierachly; we probably shall have something like a Decorator approach there.
//       config(borders) -> config(borders+colors) -> config(borders+colors+spans)
//
//       Or maybe many interfacies e.g ColorConfig, BorderConfig, AlignmentConfig etc.
