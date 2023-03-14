//! The module provides a grid backend [`Grid`] for pretty print tables.
//!
//! If you know your data at compile time and you do no need a reach set of configuration,
//! you might better use [`CompactGrid`] cause it is considered more performant.
//!
//! [`CompactGrid`]: crate::grid::compact::CompactGrid

pub mod config;
pub mod dimension;
mod grid;

pub use config::GridConfig;
pub use dimension::ExactDimension;
pub use grid::Grid;
