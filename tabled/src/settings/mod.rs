//! Module contains various table configuration settings.
//!
//! There 2 types of settings;
//!
//! - [`CellOption`] which can modify only a cell.
//! - [`TableOption`] which can modify table as a whole.
//!
//! [`CellOption`] works on behave of [`Modify`] which is actually a [`TableOption`].
//!
//! Notice that it's possble to combine settings together by the help of [`Settings`].
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{Table, settings::{Settings, Style, Padding}};
//!
//! let table_config = Settings::default()
//!     .with(Padding::new(2, 2, 1, 1))
//!     .with(Style::rounded());
//!
//! let data = [[2023;9]; 3];
//!
//! let table = Table::new(data).with(table_config).to_string();
//!
//! assert_eq!(
//!     table,
//!     "╭────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────╮\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │  0     │  1     │  2     │  3     │  4     │  5     │  6     │  7     │  8     │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      ├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │\n\
//!      ╰────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────╯"
//! )
//! ```

mod cell_option;
mod settings_list;
mod table_option;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod object;

#[cfg(feature = "std")]
mod modify;

mod alignment;
mod extract;
mod margin;
mod padding;
mod rotate;

#[cfg(feature = "std")]
mod color;
#[cfg(feature = "std")]
mod concat;
#[cfg(feature = "std")]
mod duplicate;

pub mod style;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod disable;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod format;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod formatting;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod height;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod highlight;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod locator;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod measurement;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod merge;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod panel;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod peaker;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod shadow;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod span;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod split;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod themes;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod width;

pub use cell_option::CellOption;
pub use settings_list::{EmptySettings, Settings};
pub use table_option::TableOption;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use modify::{Modify, ModifyList};

pub use self::{
    alignment::Alignment, extract::Extract, margin::Margin, padding::Padding, rotate::Rotate,
    style::Style,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use self::{
    color::Color, concat::Concat, disable::Disable, duplicate::Dup, format::Format, height::Height,
    highlight::Highlight, merge::Merge, panel::Panel, shadow::Shadow, span::Span, style::Border,
    width::Width,
};
