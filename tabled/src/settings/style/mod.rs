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
//! ## [`BorderText`]
//!
//! It's used to override a border with a custom text.
//!
//! ### Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{Table, settings::style::{BorderText, Style}};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .with(BorderText::new("Santa").horizontal(1))
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
//!     .with(Modify::new((0, 0)).with(Style::modern().get_frame()))
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

#[cfg(feature = "std")]
mod border;
#[cfg(feature = "std")]
mod border_char;
#[cfg(feature = "std")]
mod border_color;
#[cfg(feature = "std")]
mod border_text;
#[cfg(feature = "std")]
mod offset;
#[cfg(feature = "std")]
mod raw_style;
#[cfg(feature = "std")]
mod span_border_correction;

mod builder;
mod horizontal_line;
mod line;
mod vertical_line;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    border::Border, border_char::BorderChar, border_color::BorderColor, border_text::BorderText,
    offset::Offset, raw_style::RawStyle, span_border_correction::BorderSpanCorrection,
};

pub use builder::{HorizontalLineIter, On, Style, VerticalLineIter};
pub use horizontal_line::HorizontalLine;
pub use line::Line;
pub use vertical_line::VerticalLine;
