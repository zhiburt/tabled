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

mod border_color;
mod symbol;

pub use self::{
    border::Border, border_char::BorderChar, border_color::BorderColor, border_text::BorderText,
    horizontal_line::HorizontalLine, line::Line, offset::Offset, raw_style::RawStyle,
    span_border_correction::StyleCorrectSpan, style::Style, symbol::Symbol,
    vertical_line::VerticalLine,
};
