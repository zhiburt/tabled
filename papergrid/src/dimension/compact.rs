//! The module contains a [`CompactGridDimension`] for [`CompactGrid`] height/width estimation.
//!
//! [`CompactGrid`]: crate::grid::compact::CompactGrid

use core::cmp::max;

use crate::{
    dimension::{Dimension, Estimate},
    records::{IntoRecords, Records},
    util::string::{count_lines, get_text_width},
};

use crate::config::compact::CompactConfig;

/// A [`Dimension`] implementation which calculates exact column/row width/height.
///
/// [`Grid`]: crate::grid::iterable::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompactGridDimension {
    width: Vec<usize>,
}

impl CompactGridDimension {
    /// Calculates height of rows.
    pub fn height<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        build_width(records, cfg)
    }

    /// Calculates dimensions of columns.
    pub fn dimension<R>(records: R, cfg: &CompactConfig) -> (Vec<usize>, Vec<usize>)
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        build_dimension(records, cfg)
    }
}

impl Dimension for CompactGridDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, _: usize) -> usize {
        1
    }
}

impl<R> Estimate<R, CompactConfig> for CompactGridDimension
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn estimate(&mut self, records: R, cfg: &CompactConfig) {
        self.width = build_width(records, cfg);
    }
}

fn build_dimension<R>(records: R, cfg: &CompactConfig) -> (Vec<usize>, Vec<usize>)
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
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

fn build_height<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
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

fn build_width<R>(records: R, cfg: &CompactConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
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
    let width = get_text_width(text);
    let pad = cfg.get_padding();

    width + pad.left.size + pad.right.size
}
