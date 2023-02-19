//! The module provides a limited grid backend which can work in a environments like `[no_std]`.
//!
//! It has less configuration settings compared to [`Grid`].
//! But it assumed to have a better performance.
//!
//! [`Grid`]: crate::grid::spanned::Grid
mod config;
mod grid;

#[cfg(feature = "std")]
mod dimension;

pub use config::CompactConfig;
pub use grid::CompactGrid;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use dimension::ExactDimension;
