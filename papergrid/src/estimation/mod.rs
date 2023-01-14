///! The module contains an [`Estimate`] trait and its implementations.
use crate::{records::Records, GridConfig};

pub mod height;
pub mod width;
mod width_func;

/// An Evaluator of an metric of a [`Grid`]
///
/// [`Grid`]: crate::Grid
pub trait Estimate {
    /// Estimates a metric.
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig);

    /// Gets a metric by index.
    fn get(&self, i: usize) -> Option<usize>;

    /// Calculates a sum of metrics.
    fn total(&self) -> Option<usize>;
}

pub trait ExactEstimate: Estimate {
    fn total_amount(&self) -> usize;
}
