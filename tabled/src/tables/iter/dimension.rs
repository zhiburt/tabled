use crate::{
    grid::{
        compact::CompactConfig,
        dimension::{Dimension, Estimate},
        spanned::GridConfig,
    },
    records::Records,
};

#[derive(Debug, Clone)]
pub struct IterTableDimension {
    width: ExactList,
    height: ExactList,
}

impl IterTableDimension {
    pub fn new(width: ExactList, height: ExactList) -> Self {
        Self { width, height }
    }
}

impl From<IterTableDimension> for (ExactList, ExactList) {
    fn from(value: IterTableDimension) -> Self {
        (value.width, value.height)
    }
}

impl Dimension for IterTableDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width.get(column)
    }

    fn get_height(&self, row: usize) -> usize {
        self.height.get(row)
    }
}

impl Estimate<CompactConfig> for IterTableDimension {
    fn estimate<R: Records>(&mut self, _: R, _: &CompactConfig) {}
}

impl Estimate<GridConfig> for IterTableDimension {
    fn estimate<R: Records>(&mut self, _: R, _: &GridConfig) {}
}

/// A dimension value.
#[derive(Debug, Clone)]
pub enum ExactList {
    /// Const width value.
    Exact(usize),
    /// A list of width values for columns.
    List(Vec<usize>),
}

impl ExactList {
    /// Get a width by column.
    pub fn get(&self, col: usize) -> usize {
        match self {
            ExactList::Exact(val) => *val,
            ExactList::List(cols) => cols[col],
        }
    }
}
