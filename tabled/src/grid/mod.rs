//! Module is responsible for tables underlyign grid.
//!
//! It might be used when implementing your own [`TableOption`] and [`CellOption`].
//!
//! [`TableOption`]: crate::settings::TableOption
//! [`CellOption`]: crate::settings::CellOption

pub mod config;
pub mod dimension;
pub mod records;

pub use papergrid::ansi;
pub use papergrid::colors;
pub use papergrid::util;

pub use papergrid::grid::compact::CompactGrid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::grid::iterable::Grid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::grid::peekable::PeekableGrid;
