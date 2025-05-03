//! The module contains an [`Dimension`] trait and its implementations.

#[cfg(feature = "std")]
pub mod compact;
#[cfg(feature = "std")]
pub mod iterable;
#[cfg(feature = "std")]
pub mod peekable;

/// Dimension of a grid.
///
/// It's a friend trait of [`Estimate`].
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

/// Dimension estimation of a grid.
///
/// It's a friend trait of [`Dimension`].
pub trait Estimate<R, C> {
    /// Estimates a metric.
    fn estimate(&mut self, records: R, config: &C);
}

impl<T, R, C> Estimate<R, C> for &mut T
where
    T: Estimate<R, C>,
{
    fn estimate(&mut self, records: R, config: &C) {
        T::estimate(self, records, config)
    }
}
