use std::{
    borrow::Cow,
    ops::{Index, IndexMut},
};

use tabled::{
    grid::config::Position,
    grid::util::string::string_width_multiline,
    settings::{object::SegmentAll, Alignment, Modify},
    Table, Tabled,
};

#[cfg(feature = "macros")]
mod macros;

#[cfg(feature = "table")]
mod matrix;

#[cfg(feature = "table")]
pub use matrix::Matrix;

pub fn is_lines_equal(s: &str, width: usize) -> bool {
    string_width_multiline(s) == width
}
