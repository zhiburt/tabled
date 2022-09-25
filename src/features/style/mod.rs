//! Module contains different things to tweak the style of the [`Table`].
//!
//! [`Table`]: crate::Table

mod border;
mod border_char;
mod border_text;
mod horizontal_line;
mod line;
mod offset;
mod raw_style;
mod span_border_correction;
#[allow(clippy::module_inception)]
mod style;
mod vertical_line;

#[cfg(feature = "color")]
mod border_colored;
#[cfg(feature = "color")]
mod raw_style_colored;
#[cfg(feature = "color")]
mod symbol;

pub use self::{
    border::Border, border_char::BorderChar, border_text::BorderText,
    horizontal_line::HorizontalLine, line::Line, offset::Offset, raw_style::RawStyle,
    span_border_correction::StyleCorrectSpan, style::Style, vertical_line::VerticalLine,
};

#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use self::{border_colored::BorderColored, raw_style_colored::RawStyleColored, symbol::Symbol};
