#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![deny(unused_must_use)]

//! Papergrid is a library for generating text-based tables.
//!
//! It has relatively low level API.
//! If you're interested in a more friendly one take a look at [`tabled`](https://github.com/zhiburt/tabled).
//!
//! # Example
//!
//! ```
//! use papergrid::{
//!     records::iter_records::IterRecords,
//!     dimension::{Dimension, ExactDimension},
//!     Borders, Grid, GridConfig,
//! };
//!
//! // Creating a borders structure of a grid.
//! let borders = Borders {
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
//!     vertical: Some('|'),
//!     vertical_left: Some('|'),
//!     vertical_right: Some('|'),
//!     intersection: Some('+'),
//! };
//!
//! // Creating a grid config.
//! let mut cfg = GridConfig::default();
//! cfg.set_borders(borders);
//!
//! // Creating an actual data for grid.
//! let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
//! let records = IterRecords::new(records, 2);
//!
//! // Estimate grid dimension.
//! let mut dimension = ExactDimension::default();
//! dimension.estimate(&records, &cfg);
//!
//! // Creating a grid.
//! let grid = Grid::new(&records, &cfg, &dimension).to_string();
//!
//! assert_eq!(
//!     grid,
//!     concat!(
//!         "+-----+-----+\n",
//!         "|Hello|World|\n",
//!         "+-----+-----+\n",
//!         "|Hi   |World|\n",
//!         "+-----+-----+",
//!     ),
//! );
//! ```

mod grid;

pub mod color;
pub mod colors;
pub mod config;
pub mod grid_projection;
pub mod dimension;
pub mod records;
pub mod util;

pub use self::grid::Grid;
pub use dimension::{Dimension, ExactDimension};
pub use config::GridConfig;
