//! Module contains a dimension estimator for [`CompactTable`]
//!
//! [`CompactTable`]: crate::tables::CompactTable

use crate::grid::dimension::{Dimension, Estimate};

/// A constant size dimension or a value dimension.
#[derive(Debug, Clone, Copy)]
pub struct ConstDimension<const COLUMNS: usize, const ROWS: usize> {
    height: ConstSize<ROWS>,
    width: ConstSize<COLUMNS>,
}

impl<const COLUMNS: usize, const ROWS: usize> ConstDimension<COLUMNS, ROWS> {
    /// Returns a new dimension object with a given estimates.
    pub const fn new(width: ConstSize<COLUMNS>, height: ConstSize<ROWS>) -> Self {
        Self { width, height }
    }
}

impl<const COLUMNS: usize, const ROWS: usize> Dimension for ConstDimension<COLUMNS, ROWS> {
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

impl<const COLUMNS: usize, const ROWS: usize> From<ConstDimension<COLUMNS, ROWS>>
    for (ConstSize<COLUMNS>, ConstSize<ROWS>)
{
    fn from(value: ConstDimension<COLUMNS, ROWS>) -> Self {
        (value.width, value.height)
    }
}

impl<R, D, const COLUMNS: usize, const ROWS: usize> Estimate<R, D>
    for ConstDimension<COLUMNS, ROWS>
{
    fn estimate(&mut self, _: R, _: &D) {}
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
