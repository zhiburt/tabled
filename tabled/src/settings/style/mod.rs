//! This module contains a list of primitives which can be applied to change [`Table`] style.
//!
//! ## [`Style`]
//!
//! It is responsible for a table border style.
//! An individual cell border can be set by [`Border`].
//!  
//! ### Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{Table, settings::Style};
//!
//! let data = vec!["Hello", "2022"];
//! let mut table = Table::new(&data);
//! table.with(Style::psql());
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         " &str  \n",
//!         "-------\n",
//!         " Hello \n",
//!         " 2022  ",
//!     )
//! )
//! ```
//!
//! ## [`LineText`]
//!
//! It's used to override a border with a custom text.
//!
//! ### Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{
//!     Table, settings::{style::{LineText, Style}, object::Rows},
//! };
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .with(LineText::new("Santa", Rows::single(1)))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         " &str  \n",
//!         "Santa--\n",
//!         " Hello \n",
//!         " 2022  ",
//!     )
//! )
//! ```
//!
//! ## [`Border`]
//!
//! [`Border`] can be used to modify cell's borders.
//!
//! It's possible to set a collored border when `color` feature is on.
//!
//! ### Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{Table, settings::{Modify, Style, style::Border}};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .modify((0, 0), Border::inherit(Style::modern()))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "┌───────┐\n",
//!         "│ &str  │\n",
//!         "└───────┘\n",
//!         "  Hello  \n",
//!         "  2022   ",
//!     )
//! )
//! ```
//!
//! ## [`Theme`]
//!
//! A different representation of [`Theme`].
//! With no checks in place.
//!
//! It also contains a list of types to support colors.
//!
//! [`Table`]: crate::Table
//! [`BorderText`]: crate::settings::style::BorderText
//! [`Theme`]: crate::settings::themes::Theme

mod border;
mod builder;
mod horizontal_line;
mod offset;
mod vertical_line;

#[cfg(feature = "std")]
mod border_color;
#[cfg(feature = "std")]
mod border_text;
#[cfg(feature = "std")]
mod line_char;
#[cfg(feature = "std")]
mod span_border_correction;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    border_color::BorderColor, border_text::LineText, line_char::LineChar,
    span_border_correction::BorderSpanCorrection,
};

pub use self::{
    border::Border,
    builder::{On, Style},
    horizontal_line::HorizontalLine,
    offset::Offset,
    vertical_line::VerticalLine,
};

use crate::grid::config::{Borders, CompactConfig, CompactMultilineConfig};
use crate::settings::TableOption;

#[cfg(feature = "std")]
use crate::grid::config::ColoredConfig;

#[cfg(feature = "std")]
impl<R, D> TableOption<R, ColoredConfig, D> for Borders<char> {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg_clear_borders(cfg);
        cfg.set_borders(self);
    }
}

impl<R, D> TableOption<R, CompactConfig, D> for Borders<char> {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_borders(self);
    }
}

impl<R, D> TableOption<R, CompactMultilineConfig, D> for Borders<char> {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_borders(self);
    }
}

#[cfg(feature = "std")]
fn cfg_clear_borders(cfg: &mut ColoredConfig) {
    cfg.remove_borders();
    cfg.remove_borders_colors();
    cfg.remove_vertical_chars();
    cfg.remove_horizontal_chars();
    cfg.remove_color_line_horizontal();
    cfg.remove_color_line_vertical();
}
