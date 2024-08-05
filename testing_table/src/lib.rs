//! Crate provides a convinient functions for work with tables.
//!
//! It was developed as a sub-project of [`tabled`].
//!
//! [`tabled`]: https://github.com/zhiburt/tabled

mod macros;
mod util;

pub use util::{get_char_width, get_line_width, get_string_width, get_text_width};
