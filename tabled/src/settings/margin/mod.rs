//! This module contains a Margin settings of a [`Table`].
//!
//! # Example
//!
//! ```
//! use tabled::{settings::{margin::Margin, style::Style}, Table};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let mut table = Table::new(data);
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
