//! This module contains a [`Padding`] setting of a cell on a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{TableIteratorExt, Padding, Style, Modify, object::Cell};
//!
//! let data = "2022".chars();
//!
//! let table = data.table()
//!     .with(Style::modern())
//!     .with(Modify::new(Cell(2, 0)).with(Padding::new(1, 1, 2, 2)))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "┌──────┐\n",
//!         "│ char │\n",
//!         "├──────┤\n",
//!         "│ 2    │\n",
//!         "├──────┤\n",
//!         "│      │\n",
//!         "│      │\n",
//!         "│ 0    │\n",
//!         "│      │\n",
//!         "│      │\n",
//!         "├──────┤\n",
//!         "│ 2    │\n",
//!         "├──────┤\n",
//!         "│ 2    │\n",
//!         "└──────┘",
//!     ),
//! );
//! ```
//!
//! [`Table`]: crate::Table

mod border_padding;
mod border_padding_color;

pub use border_padding::Padding;
pub use border_padding_color::PaddingColor;
