#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![allow(clippy::uninlined_format_args)]
#![deny(unused_must_use)]

//! Papergrid is a library for generating text-based tables.
//!
//! It has relatively low level API.
//! If you're interested in a more friendly one take a look at [`tabled`](https://github.com/zhiburt/tabled).
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use papergrid::{
//!     records::IterRecords,
//!     dimension::{Estimate},
//!     grid::spanned::{config::{Borders, GridConfig}, Grid, ExactDimension},
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
//!     vertical: Some('|'),
//!     left: Some('|'),
//!     right: Some('|'),
//!     intersection: Some('+'),
//!     left_intersection: Some('+'),
//!     right_intersection: Some('+'),
//! };
//!
//! // Creating a grid config.
//! let mut cfg = GridConfig::default();
//! cfg.set_borders(borders);
//!
//! // Creating an actual data for grid.
//! let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
//! let records = IterRecords::new(records, 2, None);
//!
//! // Estimate grid dimension.
//! let mut dimension = ExactDimension::default();
//! dimension.estimate(&records, &cfg);
//!
//! // Creating a grid.
//! let grid = Grid::new(&records, &dimension, &cfg).to_string();
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

pub mod color;
pub mod colors;
pub mod config;
pub mod dimension;
pub mod grid;
pub mod records;
pub mod util;
