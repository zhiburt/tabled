//! The module contains a [`WidthEstimator`] for [`Grid`] width estimation.
//!
//! [`Grid`]: crate::Grid

use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

use crate::{
    records::{RecordCell, Records},
    GridConfig, Position,
};

use super::{Estimate, ExactEstimate};

pub use super::width_func::{CfgWidthFunction, WidthFunc};

/// A [`Estimate`]or of a width for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WidthEstimator {
    widths: Vec<usize>,
}

impl Estimate for WidthEstimator {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        let width_ctrl = CfgWidthFunction::from_cfg(cfg);
        self.widths = build_widths(&records, cfg, &width_ctrl);
    }

    fn get(&self, column: usize) -> Option<usize> {
        self.widths.get(column).copied()
    }

    fn total(&self) -> Option<usize> {
        Some(self.widths.iter().sum())
    }
}

impl ExactEstimate for WidthEstimator {
    fn total_amount(&self) -> usize {
        self.widths.iter().sum()
    }
}

impl From<Vec<usize>> for WidthEstimator {
    fn from(widths: Vec<usize>) -> Self {
        Self { widths }
    }
}

impl From<WidthEstimator> for Vec<usize> {
    fn from(val: WidthEstimator) -> Self {
        val.widths
    }
}

fn build_widths<R: Records>(
    records: &R,
    cfg: &GridConfig,
    width_ctrl: &CfgWidthFunction,
) -> Vec<usize> {
    let count_columns = records.count_columns();
    let mut widths = vec![0; count_columns];

    let mut spans = HashMap::new();

    for (row, columns) in records.iter_rows().enumerate() {
        for (col, cell) in columns.enumerate() {
            if !is_simple_cell(cfg, (row, col), (usize::MAX, count_columns)) {
                let is_spanned = cfg
                    .get_column_span((row, col), (usize::MAX, count_columns))
                    .is_some();
                if is_spanned {
                    let width = get_cell_width(&cell, cfg, (row, col), width_ctrl);
                    spans.insert((row, col), width);
                }

                continue;
            }

            let width = get_cell_width(&cell, cfg, (row, col), width_ctrl);
            widths[col] = max(widths[col], width);
        }
    }

    adjust_spans(cfg, &spans, count_columns, &mut widths);

    widths
}

fn adjust_spans(
    cfg: &GridConfig,
    span_list: &HashMap<Position, usize>,
    count_columns: usize,
    widths: &mut [usize],
) {
    if !cfg.has_column_spans() {
        return;
    }

    // The overall width disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = cfg
        .iter_column_spans((usize::MAX, count_columns))
        .collect::<Vec<_>>();
    spans.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_range(cfg, span_list, row, col, col + span, widths);
    }
}

fn adjust_range(
    cfg: &GridConfig,
    span_list: &HashMap<Position, usize>,
    row: usize,
    start: usize,
    end: usize,
    widths: &mut [usize],
) {
    let max_span_width = *span_list.get(&(row, start)).expect("must be there");
    let range_width = range_width(cfg, start, end, widths);

    if range_width >= max_span_width {
        return;
    }

    inc_range_width(widths, max_span_width - range_width, start, end);
}

fn inc_range_width(widths: &mut [usize], size: usize, start: usize, end: usize) {
    if widths.is_empty() {
        return;
    }

    let span = end - start;
    let one = size / span;
    let rest = size - span * one;

    let mut i = start;
    while i < end {
        if i == start {
            widths[i] += one + rest;
        } else {
            widths[i] += one;
        }

        i += 1;
    }
}

fn is_simple_cell(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    cfg.is_cell_visible(pos, shape) && matches!(cfg.get_column_span(pos, shape), None | Some(1))
}

fn get_cell_width<C>(
    cell: C,
    cfg: &GridConfig,
    pos: Position,
    width_ctrl: &CfgWidthFunction,
) -> usize
where
    C: RecordCell,
{
    let padding = get_cell_padding(cfg, pos); // todo: remove it...
    let width = cell.get_width(width_ctrl);
    width + padding
}

fn get_cell_padding(cfg: &GridConfig, pos: Position) -> usize {
    let padding = cfg.get_padding(pos.into());
    padding.left.size + padding.right.size
}

fn range_width(grid: &GridConfig, start: usize, end: usize, widths: &[usize]) -> usize {
    let count_borders = count_borders_in_range(grid, start, end, widths.len());
    let range_width = widths[start..end].iter().sum::<usize>();
    count_borders + range_width
}

fn count_borders_in_range(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, count_columns))
        .count()
}
