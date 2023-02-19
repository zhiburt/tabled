use crate::{
    grid::{
        compact::CompactConfig,
        dimension::{Dimension, Estimate},
        spanned::GridConfig,
    },
    records::into_records::truncate_records::Width,
    records::Records,
};

#[derive(Debug)]
pub struct IterTableDimension<'a> {
    width: Width<'a>,
    height: usize,
}

impl<'a> IterTableDimension<'a> {
    pub fn new(width: Width<'a>, height: usize) -> Self {
        Self { width, height }
    }
}

impl Dimension for IterTableDimension<'_> {
    fn get_width(&self, column: usize) -> usize {
        self.width.get(column)
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}

impl Estimate<CompactConfig> for IterTableDimension<'_> {
    fn estimate<R: Records>(&mut self, _: R, _: &CompactConfig) {}
}

impl Estimate<GridConfig> for IterTableDimension<'_> {
    fn estimate<R: Records>(&mut self, _: R, _: &GridConfig) {}
}
