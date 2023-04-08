use papergrid::records::vec_records::{CellInfo, VecRecords};

use crate::grid::{
    config::SpannedConfig,
    dimension::{Dimension, Estimate},
    records::Records,
};

/// PeekableDimension is a [`Dimension`] implementation for a [`Table`]
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, Clone)]
pub struct PeekableDimension {
    width: Vec<usize>,
    height: Vec<usize>,
}

impl PeekableDimension {
    /// Calculates height of rows.
    pub fn height<T: AsRef<str>>(
        records: &VecRecords<CellInfo<T>>,
        cfg: &SpannedConfig,
    ) -> Vec<usize> {
        estimation::build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<T: AsRef<str>>(
        records: &VecRecords<CellInfo<T>>,
        cfg: &SpannedConfig,
    ) -> Vec<usize> {
        estimation::build_width(records, cfg)
    }

    /// Return width and height lists.
    pub fn get_values(self) -> (Vec<usize>, Vec<usize>) {
        (self.width, self.height)
    }
}

impl Dimension for PeekableDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.height[row]
    }
}

impl<T> Estimate<&VecRecords<CellInfo<T>>, SpannedConfig> for PeekableDimension
where
    T: AsRef<str>,
{
    fn estimate(&mut self, records: &VecRecords<CellInfo<T>>, cfg: &SpannedConfig) {
        let (width, height) = estimation::build_dimensions(records, cfg);
        self.width = width;
        self.height = height;
    }
}

mod estimation {
    use core::cmp::{max, Ordering};
    use std::collections::HashMap;

    use papergrid::{
        config::Position,
        records::vec_records::{Cell, CellInfo, VecRecords},
    };

    use super::*;

    pub(super) fn build_dimensions<T: AsRef<str>>(
        records: &VecRecords<CellInfo<T>>,
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
                let pos = (row, col);
                if !cfg.is_cell_visible(pos) {
                    continue;
                }

                let height = cell.count_lines();
                let width = cell.width();

                let pad = cfg.get_padding(pos.into());
                let width = width + pad.left.size + pad.right.size;
                let height = height + pad.top.size + pad.bottom.size;

                match cfg.get_column_span(pos) {
                    Some(n) if n > 1 => {
                        let _ = vspans.insert(pos, (n, width));
                    }
                    _ => widths[col] = max(widths[col], width),
                }

                match cfg.get_row_span(pos) {
                    Some(n) if n > 1 => {
                        let _ = hspans.insert(pos, (n, height));
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

        let mut spans_ordered = spans
            .iter()
            .map(|(k, v)| ((k.0, k.1), *v))
            .collect::<Vec<_>>();
        spans_ordered.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
            Ordering::Equal => acol.cmp(bcol),
            ord => ord,
        });

        for ((row, _), (span, height)) in spans_ordered {
            adjust_row_range(cfg, height, len, row, row + span, heights);
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

    fn count_horizontal_borders(
        cfg: &SpannedConfig,
        len: usize,
        start: usize,
        end: usize,
    ) -> usize {
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
        let mut spans_ordered = spans
            .iter()
            .map(|(k, v)| ((k.0, k.1), *v))
            .collect::<Vec<_>>();
        spans_ordered.sort_unstable_by(|a, b| match a.1 .0.cmp(&b.1 .0) {
            Ordering::Equal => a.0.cmp(&b.0),
            o => o,
        });

        for ((_, col), (span, width)) in spans_ordered {
            adjust_column_range(cfg, width, len, col, col + span, widths);
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

    pub(super) fn build_height<T: AsRef<str>>(
        records: &VecRecords<CellInfo<T>>,
        cfg: &SpannedConfig,
    ) -> Vec<usize> {
        let mut heights = vec![];
        let mut hspans = HashMap::new();

        for (row, columns) in records.iter_rows().enumerate() {
            let mut row_height = 0;
            for (col, cell) in columns.iter().enumerate() {
                let pos = (row, col);
                if !cfg.is_cell_visible(pos) {
                    continue;
                }

                let height = cell.count_lines();
                match cfg.get_row_span(pos) {
                    Some(n) if n > 1 => {
                        let _ = hspans.insert(pos, (n, height));
                    }
                    _ => row_height = max(row_height, height),
                }
            }

            heights.push(row_height);
        }

        adjust_hspans(cfg, heights.len(), &hspans, &mut heights);

        heights
    }

    pub(super) fn build_width<T: AsRef<str>>(
        records: &VecRecords<CellInfo<T>>,
        cfg: &SpannedConfig,
    ) -> Vec<usize> {
        let count_columns = records.count_columns();

        let mut widths = vec![0; count_columns];
        let mut vspans = HashMap::new();

        for (row, columns) in records.iter_rows().enumerate() {
            for (col, cell) in columns.iter().enumerate() {
                let pos = (row, col);
                if !cfg.is_cell_visible(pos) {
                    continue;
                }

                let width = cell.width();
                match cfg.get_column_span(pos) {
                    Some(n) if n > 1 => {
                        let _ = vspans.insert(pos, (n, width));
                    }
                    _ => widths[col] = max(widths[col], width),
                }
            }
        }

        adjust_vspans(cfg, count_columns, &vspans, &mut widths);

        widths
    }
}
