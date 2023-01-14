//! The module contains a [`HeightEstimator`] for [`Grid`] height estimation.
//!
//! [`Grid`]: crate::Grid

use std::{
    cmp::{self, max, Ordering},
    collections::HashMap,
};

use crate::{
    records::{RecordCell, Records},
    Entity, GridConfig, Position,
};

use super::Estimate;

/// A [`Estimate`]or of a height for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeightEstimator {
    heights: Vec<usize>,
}

impl Estimate for HeightEstimator {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        self.heights = build_heights(&records, cfg);
    }

    fn get(&self, column: usize) -> Option<usize> {
        self.heights.get(column).copied()
    }

    fn total(&self) -> Option<usize> {
        Some(self.heights.iter().sum())
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

fn build_heights<R: Records>(records: &R, cfg: &GridConfig) -> Vec<usize> {
    let mut heights = vec![0; records.hint_rows().unwrap_or(0)];

    let shape = (usize::MAX, records.count_columns());
    let mut span_list = HashMap::new();

    for (row, columns) in records.iter_rows().enumerate() {
        let mut max = 0;
        for (col, cell) in columns.enumerate() {
            if !is_simple_cell(cfg, (row, col), shape) {
                let is_spanned = cfg.get_row_span((row, col), shape).is_some();
                if is_spanned {
                    let height = cell_height(&cell, cfg, (row, col));
                    span_list.insert((row, col), height);
                }
                continue;
            }

            let height = cell_height(&cell, cfg, (row, col));
            max = cmp::max(max, height);
        }

        heights.push(max);
    }

    adjust_spans(cfg, &span_list, shape.1, &mut heights);

    heights
}

fn adjust_spans(
    cfg: &GridConfig,
    span_list: &HashMap<Position, usize>,
    count_columns: usize,
    heights: &mut [usize],
) {
    if span_list.is_empty() {
        return;
    }

    // The overall height disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = cfg
        .iter_row_spans((usize::MAX, count_columns))
        .collect::<Vec<_>>();
    spans.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
        Ordering::Equal => acol.cmp(bcol),
        ord => ord,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_range(span_list, cfg, col, row, row + span, heights);
    }
}

fn adjust_range(
    span_list: &HashMap<Position, usize>,
    cfg: &GridConfig,
    col: usize,
    start: usize,
    end: usize,
    heights: &mut [usize],
) {
    let max_span_height = *span_list.get(&(start, col)).expect("must be there");
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

fn cell_height<C>(cell: &C, cfg: &GridConfig, pos: Position) -> usize
where
    C: RecordCell,
{
    let count_lines = max(1, cell.count_lines());
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
