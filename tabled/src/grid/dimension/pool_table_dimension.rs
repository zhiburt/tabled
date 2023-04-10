/// PoolTableDimension is a dimension resolve strategy for [`PoolTable`]
///
/// [`PoolTable`]: crate::tables::PoolTable
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PoolTableDimension {
    width: DimensionPriority,
    height: DimensionPriority,
}

impl PoolTableDimension {
    /// Creates a new object.
    pub fn new(width: DimensionPriority, height: DimensionPriority) -> Self {
        Self { width, height }
    }

    /// Return a width priority.
    pub fn width(&self) -> DimensionPriority {
        self.width
    }

    /// Return a height priority.
    pub fn height(&self) -> DimensionPriority {
        self.height
    }
}

/// A control of width/height logic for situations where we must increase some cell to align columns/row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DimensionPriority {
    /// Increase first cell width/height in a row/column.
    First,
    /// Increase last cell width/height in a row/column.
    Last,
    /// Increase cells width/height 1 by 1 in a row/column.
    List,
}
