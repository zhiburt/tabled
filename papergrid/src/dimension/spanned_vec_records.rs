//! The module contains a [`SpannedVecRecordsDimension`] for [`Grid`] height/width estimation.
//!
//! [`Grid`]: crate::grid::iterable::Grid

use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

use crate::{
    config::Position,
    dimension::{Dimension, Estimate},
    records::{
        vec_records::{Cell, VecRecords},
        Records,
    },
};

use crate::config::spanned::SpannedConfig;

/// A [`Dimension`] implementation which calculates exact column/row width/height for [`VecRecords`].
///
/// It is a specialization of [`SpannedGridDimension`] for [`VecRecords`].
///
/// [`SpannedGridDimension`]: crate::dimension::spanned::SpannedGridDimension
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SpannedVecRecordsDimension {
    height: Vec<usize>,
    width: Vec<usize>,
}

impl SpannedVecRecordsDimension {
    /// Calculates height of rows.
    pub fn height<T: Cell + AsRef<str>>(
        records: &VecRecords<T>,
        cfg: &SpannedConfig,
    ) -> Vec<usize> {
        build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<T: Cell + AsRef<str>>(records: &VecRecords<T>, cfg: &SpannedConfig) -> Vec<usize> {
        build_width(records, cfg)
    }

    /// Return width and height lists.
    pub fn get_values(self) -> (Vec<usize>, Vec<usize>) {
        (self.width, self.height)
    }
}

impl Dimension for SpannedVecRecordsDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.height[row]
    }
}

impl<T> Estimate<&VecRecords<T>, SpannedConfig> for SpannedVecRecordsDimension
where
    T: Cell + AsRef<str>,
{
    fn estimate(&mut self, records: &VecRecords<T>, cfg: &SpannedConfig) {
        let (width, height) = build_dimensions(records, cfg);
        self.width = width;
        self.height = height;
    }
}

fn build_dimensions<T: Cell + AsRef<str>>(
    records: &VecRecords<T>,
    cfg: &SpannedConfig,
) -> (Vec<usize>, Vec<usize>) {
    let count_columns = records.count_columns();

    let mut widths = vec![0; count_columns];
    let mut heights = vec![];

    let mut vspans = HashMap::new();
    let mut hspans = HashMap::new();

    for (row, columns) in records.iter_rows().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let width = cell.width();
            let height = cell.count_lines();
            let pad = cfg.get_padding(pos);
            let width = width + pad.left.size + pad.right.size;
            let height = height + pad.top.size + pad.bottom.size;

            match cfg.get_column_span(pos) {
                Some(n) if n > 1 => {
                    vspans.insert(pos, (n, width));
                }
                _ => widths[col] = max(widths[col], width),
            }

            match cfg.get_row_span(pos) {
                Some(n) if n > 1 => {
                    hspans.insert(pos, (n, height));
                }
                _ => row_height = max(row_height, height),
            }
        }

        heights.push(row_height);
    }

    let count_rows = heights.len();

    adjust_vspans(cfg, count_columns, &vspans, &mut widths);
    adjust_hspans(cfg, count_rows, &hspans, &mut heights);

    (widths, heights)
}

fn adjust_hspans(
    cfg: &SpannedConfig,
    len: usize,
    spans: &HashMap<Position, (usize, usize)>,
    heights: &mut [usize],
) {
    if spans.is_empty() {
        return;
    }

    let mut spans_ordered = spans.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
    spans_ordered.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
        Ordering::Equal => acol.cmp(bcol),
        ord => ord,
    });

    for (pos, (span, height)) in spans_ordered {
        adjust_row_range(cfg, height, len, pos.row(), pos.row() + span, heights);
    }
}

fn adjust_row_range(
    cfg: &SpannedConfig,
    max_span_height: usize,
    len: usize,
    start: usize,
    end: usize,
    heights: &mut [usize],
) {
    let range_height = range_height(cfg, len, start, end, heights);
    if range_height >= max_span_height {
        return;
    }

    inc_range(heights, max_span_height - range_height, start, end);
}

fn range_height(
    cfg: &SpannedConfig,
    len: usize,
    start: usize,
    end: usize,
    heights: &[usize],
) -> usize {
    let count_borders = count_horizontal_borders(cfg, len, start, end);
    let range_height = heights[start..end].iter().sum::<usize>();
    count_borders + range_height
}

fn count_horizontal_borders(cfg: &SpannedConfig, len: usize, start: usize, end: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_horizontal(i, len))
        .count()
}

fn inc_range(list: &mut [usize], size: usize, start: usize, end: usize) {
    if list.is_empty() {
        return;
    }

    let span = end - start;
    let one = size / span;
    let rest = size - span * one;

    let mut i = start;
    while i < end {
        if i == start {
            list[i] += one + rest;
        } else {
            list[i] += one;
        }

        i += 1;
    }
}

fn adjust_vspans(
    cfg: &SpannedConfig,
    len: usize,
    spans: &HashMap<Position, (usize, usize)>,
    widths: &mut [usize],
) {
    if spans.is_empty() {
        return;
    }

    // The overall width distribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans_ordered = spans.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
    spans_ordered.sort_unstable_by(|a, b| match a.1 .0.cmp(&b.1 .0) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });

    for (pos, (span, width)) in spans_ordered {
        adjust_column_range(cfg, width, len, pos.col(), pos.col() + span, widths);
    }
}

fn adjust_column_range(
    cfg: &SpannedConfig,
    max_span_width: usize,
    len: usize,
    start: usize,
    end: usize,
    widths: &mut [usize],
) {
    let range_width = range_width(cfg, len, start, end, widths);
    if range_width >= max_span_width {
        return;
    }

    inc_range(widths, max_span_width - range_width, start, end);
}

fn get_cell_padding_horizontal(cfg: &SpannedConfig, pos: Position) -> usize {
    let padding = cfg.get_padding(pos);
    padding.left.size + padding.right.size
}

fn get_cell_vertical_padding(cfg: &SpannedConfig, pos: Position) -> usize {
    let padding = cfg.get_padding(pos);
    padding.top.size + padding.bottom.size
}

fn range_width(
    cfg: &SpannedConfig,
    len: usize,
    start: usize,
    end: usize,
    widths: &[usize],
) -> usize {
    let count_borders = count_vertical_borders(cfg, len, start, end);
    let range_width = widths[start..end].iter().sum::<usize>();
    count_borders + range_width
}

fn count_vertical_borders(cfg: &SpannedConfig, len: usize, start: usize, end: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, len))
        .count()
}

fn build_height<T: Cell + AsRef<str>>(records: &VecRecords<T>, cfg: &SpannedConfig) -> Vec<usize> {
    let mut heights = vec![];
    let mut hspans = HashMap::new();

    for (row, columns) in records.iter_rows().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let height = cell.count_lines() + get_cell_vertical_padding(cfg, pos);
            match cfg.get_row_span(pos) {
                Some(n) if n > 1 => {
                    hspans.insert(pos, (n, height));
                }
                _ => row_height = max(row_height, height),
            }
        }

        heights.push(row_height);
    }

    adjust_hspans(cfg, heights.len(), &hspans, &mut heights);

    heights
}

fn build_width<T>(records: &VecRecords<T>, cfg: &SpannedConfig) -> Vec<usize>
where
    T: Cell + AsRef<str>,
{
    let count_columns = records.count_columns();

    let mut widths = vec![0; count_columns];
    let mut vspans = HashMap::new();

    for (row, columns) in records.iter_rows().enumerate() {
        for (col, cell) in columns.iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let width = cell.width() + get_cell_padding_horizontal(cfg, pos);
            match cfg.get_column_span(pos) {
                Some(n) if n > 1 => {
                    vspans.insert(pos, (n, width));
                }
                _ => widths[col] = max(widths[col], width),
            }
        }
    }

    adjust_vspans(cfg, count_columns, &vspans, &mut widths);

    widths
}
