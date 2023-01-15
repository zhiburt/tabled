//! The module contains a [`HeightEstimator`] for [`Grid`] height estimation.
//!
//! [`Grid`]: crate::Grid

use std::cmp::{max, Ordering};

use crate::{records::Records, Entity, GridConfig, Position};

use super::Estimate;

/// A [`Estimate`]or of a height for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeightEstimator {
    heights: Vec<usize>,
}

impl<R> Estimate<R> for HeightEstimator
where
    R: Records,
{
    fn estimate(&mut self, records: R, cfg: &GridConfig) {
        self.heights = build_heights(&records, cfg);
    }

    fn get(&self, column: usize) -> Option<usize> {
        self.heights.get(column).copied()
    }

    fn total(&self) -> usize {
        self.heights.iter().sum()
    }
}

impl From<Vec<usize>> for HeightEstimator {
    fn from(heights: Vec<usize>) -> Self {
        Self { heights }
    }
}

impl From<HeightEstimator> for Vec<usize> {
    fn from(val: HeightEstimator) -> Self {
        val.heights
    }
}

fn build_heights<R>(records: &R, cfg: &GridConfig) -> Vec<usize>
where
    R: Records,
{
    let shape = (records.count_rows(), records.count_columns());
    let mut heights = vec![0; records.count_rows()];
    for (row, height) in heights.iter_mut().enumerate() {
        let max = (0..records.count_columns())
            .filter(|&col| is_simple_cell(cfg, (row, col), shape))
            .map(|col| cell_height(records, cfg, (row, col)))
            .max()
            .unwrap_or_default();

        *height = max;
    }

    adjust_spans(records, cfg, &mut heights);

    heights
}

fn adjust_spans<R>(records: &R, cfg: &GridConfig, heights: &mut [usize])
where
    R: Records,
{
    if !cfg.has_row_spans() {
        return;
    }

    // The overall height disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = cfg
        .iter_row_spans((records.count_rows(), records.count_columns()))
        .collect::<Vec<_>>();
    spans.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
        Ordering::Equal => acol.cmp(bcol),
        ord => ord,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_range(records, cfg, col, row, row + span, heights);
    }
}

fn adjust_range<R>(
    records: &R,
    cfg: &GridConfig,
    col: usize,
    start: usize,
    end: usize,
    heights: &mut [usize],
) where
    R: Records,
{
    let max_span_height = cell_height(records, cfg, (start, col));
    let range_height = range_height(cfg, start, end, heights);

    if range_height >= max_span_height {
        return;
    }

    inc_range_height(heights, max_span_height - range_height, start, end);
}

fn is_simple_cell(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    cfg.is_cell_visible(pos, shape) && matches!(cfg.get_row_span(pos, shape), None | Some(1))
}

fn range_height(grid: &GridConfig, start: usize, end: usize, heights: &[usize]) -> usize {
    let count_borders = count_borders_in_range(grid, start, end, heights.len());
    let range_height = heights[start..end].iter().sum::<usize>();
    count_borders + range_height
}

fn count_borders_in_range(cfg: &GridConfig, start: usize, end: usize, count_rows: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_horizontal(i, count_rows))
        .count()
}

fn cell_height<R>(records: &R, cfg: &GridConfig, pos: Position) -> usize
where
    R: Records,
{
    let count_lines = max(1, records.count_lines(pos));
    let padding = cfg.get_padding(Entity::Cell(pos.0, pos.1));
    count_lines + padding.top.size + padding.bottom.size
}

fn inc_range_height(heights: &mut [usize], size: usize, start: usize, end: usize) {
    if heights.is_empty() {
        return;
    }

    let span = end - start;
    let one = size / span;
    let rest = size - span * one;

    let mut i = start;
    while i < end {
        if i == start {
            heights[i] += one + rest;
        } else {
            heights[i] += one;
        }

        i += 1;
    }
}
