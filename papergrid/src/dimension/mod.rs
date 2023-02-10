//! The module contains an [`Dimension`] trait and its implementations.

use crate::{records::Records, GridConfig};

mod exact_dimension;

pub use crate::dimension::exact_dimension::ExactDimension;

/// Dimension of a [`Grid`]
///
/// [`Grid`]: crate::Grid
pub trait Dimension {
    /// Estimates a metric.
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig);

    /// Get a column width by index.
    fn get_width(&self, column: usize) -> usize;

    /// Get a row height by index.
    fn get_height(&self, row: usize) -> usize;
}
