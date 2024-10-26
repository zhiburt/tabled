//! Module contains a list of configs for varios tables/grids.

#[cfg(feature = "std")]
mod colored_config;

#[cfg(feature = "std")]
mod spanned_config;

mod compact_config;
mod compact_multiline_config;

pub use papergrid::config::{
    AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity, EntityIterator, Formatting,
    HorizontalLine, Indent, Position, Sides, VerticalLine,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use spanned_config::{EntityMap, Offset, SpannedConfig};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use colored_config::{ColorMap, ColoredConfig};

pub use compact_config::CompactConfig;
pub use compact_multiline_config::CompactMultilineConfig;
