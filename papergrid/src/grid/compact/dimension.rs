//! The module contains a [`ExactDimension`] for [`CompactGrid`] height/width estimation.
//!
//! [`CompactGrid`]: crate::grid::compact::CompactGrid

use core::cmp::max;

use crate::{
    dimension::{Dimension, Estimate},
    records::Records,
    util::string::{count_lines, string_width_multiline},
};

use super::config::CompactConfig;

/// A [`Dimension`] implementation which calculates exact column/row width/height.
///
/// [`Grid`]: crate::grid::iterable::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExactDimension {
    height: usize,
    width: Vec<usize>,
}

impl ExactDimension {
    /// Calculates height of rows.
    pub fn height<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
        build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<R: Records>(records: R, cfg: &CompactConfig) -> Vec<usize> {
        build_width(records, cfg)
    }
}

impl Dimension for ExactDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, _: usize) -> usize {
        self.height
    }
}

impl Estimate<CompactConfig> for ExactDimension {
    fn estimate<R: Records>(&mut self, records: R, cfg: &CompactConfig) {
        self.width = Self::width(records, cfg);
        self.height = calc_height(cfg);
    }
}

fn calc_height(cfg: &CompactConfig) -> usize {
    1 + cfg.get_padding().top.size + cfg.get_padding().bottom.size
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
    let padding = cfg.get_padding();
    let count_lines = max(1, count_lines(cell));
    count_lines + padding.top.size + padding.bottom.size
}

fn get_cell_width(text: &str, cfg: &CompactConfig) -> usize {
    let pad = cfg.get_padding();
    let width = string_width_multiline(text);

    width + pad.left.size + pad.right.size
}
