#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```
//! use papergrid::{Grid, Entity, Borders};
//!
//! let data = vec![
//!     vec![String::from("0-0"), String::from("0-1")],
//!     vec![String::from("1-0"), String::from("1-1")],
//! ];
//!
//! let mut grid = Grid::new(data, 2, 2);
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
mod estimation;
mod grid;

mod color;

pub mod records;

pub use estimation::height;
pub use estimation::width;
pub use estimation::Estimate;

pub use border::Border;
pub use borders::{Borders, Line};
pub use entity::{Entity, EntityIterator};
pub use grid::{
    AlignmentHorizontal, AlignmentVertical, Formatting, Grid, GridConfig, Indent, Margin, Padding,
    Position,
};

#[cfg(feature = "color")]
pub use crate::{
    color::{ansi_color::AnsiColor, Color},
    grid::{MarginColor, PaddingColor},
};

pub mod util;
