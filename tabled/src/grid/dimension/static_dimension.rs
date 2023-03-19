use crate::grid::dimension::{Dimension, Estimate};

/// A constant dimension.
#[derive(Debug, Clone)]
pub struct StaticDimension {
    width: DimensionValue,
    height: DimensionValue,
}

impl StaticDimension {
    /// Creates a constant dimension.
    pub fn new(width: DimensionValue, height: DimensionValue) -> Self {
        Self { width, height }
    }
}

impl From<StaticDimension> for (DimensionValue, DimensionValue) {
    fn from(value: StaticDimension) -> Self {
        (value.width, value.height)
    }
}

impl Dimension for StaticDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width.get(column)
    }

    fn get_height(&self, row: usize) -> usize {
        self.height.get(row)
    }
}

impl<R, C> Estimate<R, C> for StaticDimension {
    fn estimate(&mut self, _: R, _: &C) {}
}

/// A dimension value.
#[derive(Debug, Clone)]
pub enum DimensionValue {
    /// Const width value.
    Exact(usize),
    /// A list of width values for columns.
    List(Vec<usize>),
    /// A list of width values for columns and a value for the rest.
    Partial(Vec<usize>, usize),
}

impl DimensionValue {
    /// Get a width by column.
    pub fn get(&self, col: usize) -> usize {
        match self {
            DimensionValue::Exact(val) => *val,
            DimensionValue::List(cols) => cols[col],
            DimensionValue::Partial(cols, val) => {
                if cols.len() > col {
                    cols[col]
                } else {
                    *val
                }
            }
        }
    }
}
