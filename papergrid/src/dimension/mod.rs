//! The module contains an [`Dimension`] trait and its implementations.

use crate::{records::Records, GridConfig};

mod exact_dimension;

pub use crate::dimension::exact_dimension::ExactDimension;

/// Dimension of a [`Grid`]
///
/// It's a friend trait of [`Estimate`].
///
/// [`Grid`]: crate::Grid
pub trait Dimension {
    /// Get a column width by index.
    fn get_width(&self, column: usize) -> usize;

    /// Get a row height by index.
    fn get_height(&self, row: usize) -> usize;
}

impl<T> Dimension for &T
where
    T: Dimension,
{
    fn get_height(&self, row: usize) -> usize {
        T::get_height(self, row)
    }

    fn get_width(&self, column: usize) -> usize {
        T::get_width(self, column)
    }
}

/// Dimension estimation of a [`Grid`]
///
/// It's a friend trait of [`Dimension`].
///
/// [`Grid`]: crate::Grid
pub trait Estimate {
    /// Estimates a metric.
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig);
}

impl<T> Estimate for &mut T
where
    T: Estimate,
{
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        T::estimate(self, records, cfg)
    }
}
