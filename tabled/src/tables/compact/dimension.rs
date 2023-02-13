use crate::{
    grid::{
        compact::CompactConfig,
        dimension::{Dimension, Estimate},
        spanned::GridConfig,
    },
    records::Records,
};

#[derive(Debug, Clone, Copy)]
pub struct ConstantDimension<const COUNT_COLUMNS: usize> {
    width: Width<COUNT_COLUMNS>,
    height: usize,
}

impl<const COUNT_COLUMNS: usize> ConstantDimension<COUNT_COLUMNS> {
    pub fn new(width: Width<COUNT_COLUMNS>, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Width<const N: usize> {
    List([usize; N]),
    Const(usize),
}

impl<const COUNT_COLUMNS: usize> Dimension for ConstantDimension<COUNT_COLUMNS> {
    fn get_width(&self, column: usize) -> usize {
        match self.width {
            Width::List(list) => list[column],
            Width::Const(val) => val,
        }
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}

impl<const COUNT_COLUMNS: usize> Estimate<CompactConfig> for ConstantDimension<COUNT_COLUMNS> {
    fn estimate<R: Records>(&mut self, _: R, _: &CompactConfig) {}
}

impl<const COUNT_COLUMNS: usize> Estimate<GridConfig> for ConstantDimension<COUNT_COLUMNS> {
    fn estimate<R: Records>(&mut self, _: R, _: &GridConfig) {}
}
