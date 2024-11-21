//! Module contains a list of table representatives.
//!
//! ## [`Table`]
//!
//! A default table implementation.
//!
//! At it's core it keeps data buffered.
//! Be cautious about it.
//!
//! Peek it by default.
//!
//! ## [`IterTable`]
//!
//! Just like [`Table`] but it's API is a bit different to serve better in context
//! where there is a memory limit.
//!
//! It's different in implementation algorithms.
//!
//! From performance point of view it's similar to [`Table`], may be a bit slower.
//! Test it on your specific table representation.
//!
//! Peek it when you want to have a feature full table.
//! But you have a memory conserns.
//!
//! ## [`PoolTable`]
//!
//! A table with a greater control of a layout.
//! So you can build tables with a different layout/look easily.
//!
//! Peek it when you need it.
//!
//! ## [`CompactTable`]
//!
//! A table with a limited subset of settings but it works in a `no-std` context.
//! And it consumes the least amount of memory/cpu.
//! Cause it print records one by one.
//!
//! Peek it when your data contains a single line only,
//! and you don't need lots a features.
//! Or you're at `no-std` context.
//!
//! It's likely the fastest table in this limited context.
//!
//! ## [`ExtendedTable`]
//!
//! It's a table which is useful for showing large amount of data.
//! Though it's performance is generic.
//!
//! Peek it when you need it.

mod compact;

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
