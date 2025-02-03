//! Crate provides a convinient functions for work with tables.
//!
//! ```ignore
//! use testing_table::test_table;
//!
//! test_table!(
//!     test_tabled,
//!     tabled::Table::new([[1, 2, 3]]),
//!     "+---+---+---+"
//!     "| 0 | 1 | 2 |"
//!     "+---+---+---+"
//!     "| 1 | 2 | 3 |"
//!     "+---+---+---+"
//! );
//! ```
//!
//! ```ignore
//! use testing_table::assert_table;
//!
//! assert_table!(
//!     tabled::Table::new([[1, 2, 3]]),
//!     "+---+---+---+"
//!     "| 0 | 1 | 2 |"
//!     "+---+---+---+"
//!     "| 1 | 2 | 3 |"
//!     "+---+---+---+"
//! );
//! ```
//!
//! ```
//! use testing_table::static_table;
//!
//! static_table!(
//!     "+---+---+---+"
//!     "| 0 | 1 | 2 |"
//!     "+---+---+---+"
//!     "| 1 | 2 | 3 |"
//!     "+---+---+---+"
//! );
//! ```
//!
//! It was developed as a sub-project of [`tabled`].
//!
//! [`tabled`]: https://github.com/zhiburt/tabled

mod macros;
mod util;

pub use util::{get_char_width, get_line_width, get_string_width, get_text_width};
