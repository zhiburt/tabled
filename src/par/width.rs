//! The module contains a [`WidthEstimator`] for [`Grid`] width estimation.
//!
//! [`Grid`]: crate::Grid

use std::cmp::Ordering;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use papergrid::{records::Records, width::CfgWidthFunction, Estimate, GridConfig, Position};

/// A [`Estimate`]or of a width for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WidthEstimator {
    widths: Vec<usize>,
}

impl<R> Estimate<R> for WidthEstimator
where
    R: Records + Sync,
{
    fn estimate(&mut self, records: R, cfg: &GridConfig) {
        let width_ctrl = CfgWidthFunction::from_cfg(cfg);
        self.widths = build_widths(&records, cfg, &width_ctrl);
    }

    fn get(&self, column: usize) -> Option<usize> {
        self.widths.get(column).copied()
    }

    fn total(&self) -> usize {
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

fn build_widths<R>(records: &R, cfg: &GridConfig, width_ctrl: &CfgWidthFunction) -> Vec<usize>
where
    R: Records + Sync,
{
    let shape = (records.count_rows(), records.count_columns());
    let mut widths = (0..records.count_columns())
        .into_par_iter()
        .map(|col| {
            (0..records.count_rows())
                .filter(|&row| is_simple_cell(cfg, (row, col), shape))
                .map(|row| get_cell_width(cfg, records, (row, col), width_ctrl))
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    adjust_spans(cfg, width_ctrl, records, &mut widths);

    widths
}

fn adjust_spans<R>(
    cfg: &GridConfig,
    width_ctrl: &CfgWidthFunction,
    records: &R,
    widths: &mut [usize],
) where
    R: Records,
{
    if !cfg.has_column_spans() {
        return;
    }

    // The overall width disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = cfg
        .iter_column_spans((records.count_rows(), records.count_columns()))
        .collect::<Vec<_>>();
    spans.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_range(cfg, width_ctrl, records, row, col, col + span, widths);
    }
}

fn adjust_range<R>(
    cfg: &GridConfig,
    width_ctrl: &CfgWidthFunction,
    records: &R,
    row: usize,
    start: usize,
    end: usize,
    widths: &mut [usize],
) where
    R: Records,
{
    let max_span_width = get_cell_width(cfg, records, (row, start), width_ctrl);
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

fn get_cell_width<R>(
    cfg: &GridConfig,
    records: &R,
    pos: Position,
    width_ctrl: &CfgWidthFunction,
) -> usize
where
    R: Records,
{
    let width = records.get_width(pos, width_ctrl);
    let padding = get_cell_padding(cfg, pos);
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
