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
//! use tabled::{Table, settings::style::{LineText, Style}};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .with(LineText::new("Santa").horizontal(1))
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
//! use tabled::{Table, settings::{Modify, Style}};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .modify((0, 0), Style::modern().get_frame())
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
//! ## [`RawStyle`]
//!
//! A different representation of [`Style`].
//! With no checks in place.
//!
//! It also contains a list of types to support colors.
//!
//! [`Table`]: crate::Table
//! [`BorderText`]: crate::settings::style::BorderText
//! [`RawStyle`]: crate::settings::style::RawStyle

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
#[allow(clippy::module_inception)]
mod style;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    border_color::BorderColor, border_text::LineText, line_char::LineChar, offset::Offset,
    span_border_correction::BorderSpanCorrection, style::Style,
};

pub use self::{
    border::Border,
    builder::{On, StyleBuilder},
    horizontal_line::HorizontalLine,
    vertical_line::VerticalLine,
};
