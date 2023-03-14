//! The module contains an [`Dimension`] trait and its implementations.

use crate::records::Records;

/// Dimension of a [`Grid`]
///
/// It's a friend trait of [`Estimate`].
///
/// [`Grid`]: crate::grid::iterable::Grid
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
/// [`Grid`]: crate::grid::iterable::Grid
pub trait Estimate<Config> {
    /// Estimates a metric.
    fn estimate<R: Records>(&mut self, records: R, cfg: &Config);
}
