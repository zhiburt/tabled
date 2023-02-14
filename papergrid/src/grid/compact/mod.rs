//! The module provides a limited grid backend which can work in a environments like `[no_std]`.
//!
//! It has less configuration settings compared to [`Grid`].
//! But it assumed to have a better performance.
//!
//! [`Grid`]: crate::grid::spanned::Grid

mod config;
mod dimension;
mod grid;

pub use config::CompactConfig;
pub use dimension::ExactDimension;
pub use grid::CompactGrid;
