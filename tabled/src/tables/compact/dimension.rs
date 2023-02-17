//! Module contains a dimension estimator for [`CompactTable`]
//!
//! [`CompactTable`]: crate::tables::compact::CompactTable

use crate::{
    grid::{
        compact::CompactConfig,
        dimension::{Dimension, Estimate},
    },
    records::Records,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
use crate::grid::spanned::GridConfig;

/// A constant size dimension or a value dimension.
#[derive(Debug, Clone, Copy)]
pub struct ConstantDimension<const ROWS: usize, const COLUMNS: usize> {
    width: ConstSize<COLUMNS>,
    height: ConstSize<ROWS>,
}

impl<const ROWS: usize, const COLUMNS: usize> ConstantDimension<ROWS, COLUMNS> {
    /// Returns a new dimension object with a given estimates.
    pub fn new(width: ConstSize<COLUMNS>, height: ConstSize<ROWS>) -> Self {
        Self { width, height }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Dimension for ConstantDimension<ROWS, COLUMNS> {
    fn get_width(&self, column: usize) -> usize {
        match self.width {
            ConstSize::List(list) => list[column],
            ConstSize::Value(val) => val,
        }
    }

    fn get_height(&self, row: usize) -> usize {
        match self.height {
            ConstSize::List(list) => list[row],
            ConstSize::Value(val) => val,
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Estimate<CompactConfig>
    for ConstantDimension<ROWS, COLUMNS>
{
    fn estimate<R: Records>(&mut self, _: R, _: &CompactConfig) {}
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<const ROWS: usize, const COLUMNS: usize> Estimate<GridConfig>
    for ConstantDimension<ROWS, COLUMNS>
{
    fn estimate<R: Records>(&mut self, _: R, _: &GridConfig) {}
}

/// Const size represents either a const array values or a single value which responsible for the whole list.
#[derive(Debug, Clone, Copy)]
pub enum ConstSize<const N: usize> {
    /// A constant array of estimates.
    List([usize; N]),
    /// A value which act as a single estimate for all entries.
    Value(usize),
}

impl From<usize> for ConstSize<0> {
    fn from(value: usize) -> Self {
        ConstSize::Value(value)
    }
}

impl<const N: usize> From<[usize; N]> for ConstSize<N> {
    fn from(value: [usize; N]) -> Self {
        ConstSize::List(value)
    }
}
