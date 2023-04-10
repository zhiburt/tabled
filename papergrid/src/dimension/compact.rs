//! The module contains a [`CompactGridDimension`] for [`CompactGrid`] height/width estimation.
//!
//! [`CompactGrid`]: crate::grid::compact::CompactGrid

use core::cmp::max;

use crate::{
    dimension::{Dimension, Estimate},
    records::Records,
    util::string::{count_lines, string_width_multiline},
};

use crate::config::compact::CompactConfig;

/// A [`Dimension`] implementation which calculates exact column/row width/height.
///
/// [`Grid`]: crate::grid::iterable::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompactGridDimension {
    height: usize,
    width: Vec<usize>,
}

impl CompactGridDimension {
    /// Calculates height of rows.
    pub fn height<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
        build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
        build_width(records, cfg)
    }

    /// Calculates dimensions of columns.
    pub fn dimension<R: Records>(records: R, cfg: &CompactConfig) -> (Vec<usize>, Vec<usize>) {
        build_dims(records, cfg)
    }
}

impl Dimension for CompactGridDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}

impl<R> Estimate<R, CompactConfig> for CompactGridDimension
where
    R: Records,
{
    fn estimate(&mut self, records: R, cfg: &CompactConfig) {
        self.width = build_width(records, cfg);
        let pad = cfg.get_padding();
        self.height = 1 + pad.top.size + pad.bottom.size;
    }
}

fn build_dims<R: Records>(records: R, cfg: &CompactConfig) -> (Vec<usize>, Vec<usize>) {
    let mut heights = vec![];
    let mut widths = vec![0; records.count_columns()];

    for columns in records.iter_rows() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let height = get_cell_height(cell.as_ref(), cfg);
            let width = get_cell_width(cell.as_ref(), cfg);
            row_height = max(row_height, height);
            widths[col] = max(widths[col], width)
        }

        heights.push(row_height);
    }

    (widths, heights)
}

fn build_height<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
    let mut heights = vec![];

    for columns in records.iter_rows() {
        let mut row_height = 0;
        for cell in columns.into_iter() {
            let height = get_cell_height(cell.as_ref(), cfg);
            row_height = max(row_height, height);
        }

        heights.push(row_height);
    }

    heights
}

fn build_width<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
    let mut widths = vec![0; records.count_columns()];
    for columns in records.iter_rows() {
        for (col, cell) in columns.into_iter().enumerate() {
            let width = get_cell_width(cell.as_ref(), cfg);
            widths[col] = max(widths[col], width);
        }
    }

    widths
}

fn get_cell_height(cell: &str, cfg: &CompactConfig) -> usize {
    let count_lines = max(1, count_lines(cell));
    let pad = cfg.get_padding();

    count_lines + pad.top.size + pad.bottom.size
}

fn get_cell_width(text: &str, cfg: &CompactConfig) -> usize {
    let width = string_width_multiline(text);
    let pad = cfg.get_padding();

    width + pad.left.size + pad.right.size
}
