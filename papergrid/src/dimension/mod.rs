//! The module contains an [`Dimension`] trait and its implementations.

#[cfg(feature = "std")]
pub mod compact;
#[cfg(feature = "std")]
pub mod spanned;
#[cfg(feature = "std")]
pub mod spanned_vec_records;

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
pub trait Estimate<R, C> {
    /// Estimates a metric.
    fn estimate(&mut self, records: R, config: &C);
}
