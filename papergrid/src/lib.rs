#![warn(rust_2018_idioms, missing_debug_implementations, unreachable_pub)]
#![deny(unused_must_use)]

//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//! use papergrid::{Grid, Entity, Borders, Settings};
//!
//! let mut grid = Grid::new(2, 2);
//! grid.set_borders(Borders {
//!     top: Some('-'),
//!     top_left: Some('+'),
//!     top_right: Some('+'),
//!     top_intersection: Some('+'),
//!     bottom: Some('-'),
//!     bottom_left: Some('+'),
//!     bottom_right: Some('+'),
//!     bottom_intersection: Some('+'),
//!     horizontal: Some('-'),
//!     horizontal_left: Some('+'),
//!     horizontal_right: Some('+'),
//!     vertical_left: Some('|'),
//!     vertical_right: Some('|'),
//!     vertical_intersection: Some('|'),
//!     intersection: Some('+'),
//! });
//!
//! grid.set(Entity::Cell(0, 0), Settings::new().text("0-0"));
//! grid.set(Entity::Cell(0, 1), Settings::new().text("0-1"));
//! grid.set(Entity::Cell(1, 0), Settings::new().text("1-0"));
//! grid.set(Entity::Cell(1, 1), Settings::new().text("1-1"));
//!
//! assert_eq!(
//!     grid.to_string(),
//!     concat!(
//!         "+---+---+\n",
//!         "|0-0|0-1|\n",
//!         "+---+---+\n",
//!         "|1-0|1-1|\n",
//!         "+---+---+",
//!     )
//! );
//! ```

mod border;
mod borders;
mod entity;
mod entity_map;
mod grid;
#[cfg(feature = "color")]
mod symbol;

pub use border::Border;
pub use borders::{Borders, Line};
pub use entity::{Entity, EntityIterator};
pub use grid::{
    AlignmentHorizontal, AlignmentVertical, Formatting, Grid, Indent, Margin, Padding, Position,
    Settings, Style,
};

#[cfg(feature = "color")]
pub use grid::Color;
#[cfg(feature = "color")]
pub use symbol::Symbol;

pub mod util {
    pub use crate::grid::{
        count_borders_in_range, count_lines, cut_str, string_split_at_length, string_width,
        string_width_multiline,
    };
}
