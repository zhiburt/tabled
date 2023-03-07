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
//! use tabled::{Table, settings::{Settings, style::Style, padding::Padding}};
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

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod cell_option;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod modify;
mod settings_list;
mod table_option;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod object;

pub mod alignment;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod concat;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod disable;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod extract;
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
pub mod margin;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod measurement;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod merge;
pub mod padding;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod panel;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod peaker;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod rotate;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod shadow;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod span;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod split;
pub mod style;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod color;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod width;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use cell_option::CellOption;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use modify::{Modify, ModifyList};

pub use settings_list::{EmptySettings, Settings};
pub use table_option::TableOption;
