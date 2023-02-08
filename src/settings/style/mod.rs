//! Module contains different things to tweak the style of the [`Table`].
//!
//! [`Table`]: crate::Table

mod border;
mod border_char;
mod border_color;
mod border_text;
mod horizontal_line;
mod line;
mod offset;
mod raw_style;
mod span_border_correction;
mod vertical_line;

pub mod builder;

pub use self::{
    border::Border, border_char::BorderChar, border_color::BorderColor, border_text::BorderText,
    builder::Style, horizontal_line::HorizontalLine, line::Line, offset::Offset,
    raw_style::RawStyle, span_border_correction::StyleCorrectSpan, vertical_line::VerticalLine,
};
