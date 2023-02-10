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
//! ```rust
//! use tabled::{Table, settings::{Settings, style::Style, padding::Padding}};
//!
//! let table_config = Settings::default()
//!     .with(Padding::new(2, 2, 1, 1))
//!     .with(Style::rounded());
//!
//! let data = [[2023;10]; 3];
//!
//! let table = Table::new(data).with(table_config).to_string();
//!
//! assert_eq!(
//!     table,
//!     "╭────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────╮\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │  0     │  1     │  2     │  3     │  4     │  5     │  6     │  7     │  8     │  9     │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      ├────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │  2023  │\n\
//!      │        │        │        │        │        │        │        │        │        │        │\n\
//!      ╰────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────╯"
//! )
//! ```

mod cell_option;
mod modify;
mod settings_list;
mod table_option;

pub mod object;

pub mod alignment;
pub mod color;
pub mod concat;
pub mod disable;
pub mod extract;
pub mod format;
pub mod formatting;
pub mod height;
pub mod highlight;
pub mod locator;
pub mod margin;
pub mod measurement;
pub mod merge;
pub mod padding;
pub mod panel;
pub mod peaker;
pub mod rotate;
pub mod shadow;
pub mod span;
pub mod style;
pub mod width;

pub use cell_option::CellOption;
pub use modify::{Modify, ModifyList};
pub use settings_list::{EmptySettings, Settings};
pub use table_option::TableOption;
