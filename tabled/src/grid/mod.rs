//! Module is responsible for tables underlyign grid.
//!
//! It might be used when implementing your own [`TableOption`] and [`CellOption`].
//!
//! [`TableOption`]: crate::settings::TableOption
//! [`CellOption`]: crate::settings::CellOption
#[cfg(feature = "std")]
mod colored_config;

mod compact_multiline_config;

pub mod dimension;
pub mod records;

pub use papergrid::color;
pub use papergrid::colors;
pub use papergrid::util;

pub mod config {
    //! Module contains a list of configs for varios tables/grids.

    pub use papergrid::config::{
        compact::CompactConfig, AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity,
        EntityIterator, Indent, Line, Position, Sides,
    };

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub use papergrid::config::spanned::{
        EntityMap, Formatting, HorizontalLine, Offset, SpannedConfig, VerticalLine,
    };

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub use super::colored_config::{ColorMap, ColoredConfig};

    pub use super::compact_multiline_config::CompactMultilineConfig;
}

pub use papergrid::grid::compact::CompactGrid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::grid::iterable::Grid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use papergrid::grid::peekable::PeekableGrid;
