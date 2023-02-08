//! This module contains a Margin settings of a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{Margin, Style, TableIteratorExt};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let mut table = data.table();
//! table.with(Style::markdown()).with(Margin::new(3, 3, 1, 0));
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "               \n",
//!         "   | &str  |   \n",
//!         "   |-------|   \n",
//!         "   | Hello |   \n",
//!         "   | World |   \n",
//!         "   | !     |   ",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table

mod table_margin;
mod table_margin_color;

pub use table_margin::Margin;
pub use table_margin_color::MarginColor;
