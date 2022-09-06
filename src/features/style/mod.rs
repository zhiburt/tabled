//! Module contains different things to tweak the style of the [`Table`].
//!
//! [`Table`]: crate::Table

pub mod raw_style;
pub mod span_border_correction;

#[allow(clippy::module_inception)]
pub mod style;

#[cfg(feature = "color")]
pub mod raw_style_colored;
#[cfg(feature = "color")]
pub mod symbol;

pub use self::{
    raw_style::RawStyle,
    span_border_correction::StyleCorrectSpan,
    style::{HorizontalLine, Line, Style, VerticalLine},
};

#[cfg(feature = "color")]
pub use self::{raw_style_colored::RawStyleColored, symbol::Symbol};
