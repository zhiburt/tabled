use crate::grid::dimension::{Dimension, Estimate};

/// A constant dimension.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroDimension {}

impl ZeroDimension {
    /// Creates new dimension object.
    pub fn new() -> Self {
        Self {}
    }
}

impl Dimension for ZeroDimension {
    fn get_width(&self, _: usize) -> usize {
        0
    }

    fn get_height(&self, _: usize) -> usize {
        0
    }
}

impl<R, C> Estimate<R, C> for ZeroDimension {
    fn estimate(&mut self, _: R, _: &C) {}
}
