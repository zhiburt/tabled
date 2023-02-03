//! The module contains a [`HeightEstimator`] for [`Grid`] height estimation.
//!
//! [`Grid`]: crate::Grid

use std::{
    cmp::{self, max, Ordering},
    collections::HashMap,
};

use crate::{
    config::{Entity, GridConfig, Position},
    dimension::width_func::{CfgWidthFunc, WidthFunc},
    grid_projection::GridProjection,
    records::Records,
    util::count_lines,
};

use super::Dimension;

/// A [`Estimate`]or of a height for a [`Grid`].
///
/// [`Grid`]: crate::Grid
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExactDimension {
    height: Vec<usize>,
    width: Vec<usize>,
}

impl ExactDimension {
    pub fn height<R: Records>(records: R, cfg: &GridConfig) -> Vec<usize> {
        build_height(records, cfg)
    }

    pub fn width<R: Records>(records: R, cfg: &GridConfig) -> Vec<usize> {
        build_width(records, cfg)
    }
}

impl Dimension for ExactDimension {
    fn estimate<R: Records>(&mut self, records: R, cfg: &GridConfig) {
        let (width, height) = build_dimensions(records, cfg);
        self.width = width;
        self.height = height;
    }

    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.height[row]
    }
}

impl Into<(Vec<usize>, Vec<usize>)> for ExactDimension {
    fn into(self) -> (Vec<usize>, Vec<usize>) {
        (self.width, self.height)
    }
}

fn build_dimensions<R: Records>(records: R, cfg: &GridConfig) -> (Vec<usize>, Vec<usize>) {
    let count_columns = records.count_columns();
    let shape = (usize::MAX, count_columns);

    let mut widths = vec![0; count_columns];
    let mut heights = vec![];

    let mut vspans = HashMap::new();
    let mut hspans = HashMap::new();

    let wctrl = CfgWidthFunc::from_cfg(cfg);
    let gp = GridProjection::with_shape(cfg, shape);

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col);

            if !gp.is_cell_visible(pos) {
                continue;
            }

            let cell = cell.as_ref();

            let width = get_cell_width(cell, cfg, pos, &wctrl);

            let vspan = gp.get_span_column(pos);
            let has_vspan = !matches!(vspan, None | Some(1));
            if has_vspan {
                vspans.insert(pos, width);
            } else {
                widths[col] = max(widths[col], width);
            }

            let height = get_cell_height(cell, cfg, pos);

            let hspan = gp.get_span_row(pos);
            let has_hspan = !matches!(hspan, None | Some(1));
            if has_hspan {
                hspans.insert(pos, height);
            } else {
                row_height = cmp::max(row_height, height);
            }
        }

        heights.push(row_height);
    }

    // recreate projection cause we know the exact number of rows
    let gp = GridProjection::with_shape(cfg, (heights.len(), count_columns));

    adjust_vspans(&gp, &vspans, &mut widths);
    adjust_hspans(&gp, &hspans, &mut heights);

    (widths, heights)
}

fn adjust_hspans(
    gp: &GridProjection<'_>,
    span_list: &HashMap<Position, usize>,
    heights: &mut [usize],
) {
    if span_list.is_empty() {
        return;
    }

    // The overall height disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    //
    // todo: we actually have a span list already.... so we could keep order from the begining
    let mut spans = gp.iter_span_rows().collect::<Vec<_>>();
    spans.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
        Ordering::Equal => acol.cmp(bcol),
        ord => ord,
    });

    for ((row, col), span) in spans {
        adjust_row_range(span_list, gp, col, row, row + span, heights);
    }
}

fn adjust_row_range(
    span_list: &HashMap<Position, usize>,
    gp: &GridProjection<'_>,
    col: usize,
    start: usize,
    end: usize,
    heights: &mut [usize],
) {
    let max_span_height = *span_list.get(&(start, col)).expect("must be there");
    let range_height = range_height(gp, start, end, heights);
    if range_height >= max_span_height {
        return;
    }

    inc_range(heights, max_span_height - range_height, start, end);
}

fn range_height(gp: &GridProjection<'_>, start: usize, end: usize, heights: &[usize]) -> usize {
    let count_borders = count_horizontal_borders(gp, start, end);
    let range_height = heights[start..end].iter().sum::<usize>();
    count_borders + range_height
}

fn count_horizontal_borders(gp: &GridProjection<'_>, start: usize, end: usize) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| gp.has_horizontal(i))
        .count()
}

fn get_cell_height(cell: &str, cfg: &GridConfig, pos: Position) -> usize {
    let count_lines = max(1, count_lines(cell));
    let padding = cfg.get_padding(Entity::Cell(pos.0, pos.1));
    count_lines + padding.top.size + padding.bottom.size
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
    gp: &GridProjection<'_>,
    span_list: &HashMap<Position, usize>,
    widths: &mut [usize],
) {
    if !gp.has_span_columns() {
        return;
    }

    // The overall width disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = gp.iter_span_columns().collect::<Vec<_>>();
    spans.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_column_range(gp, span_list, row, col, col + span, widths);
    }
}

fn adjust_column_range(
    gp: &GridProjection<'_>,
    span_list: &HashMap<Position, usize>,
    row: usize,
    start: usize,
    end: usize,
    widths: &mut [usize],
) {
    let max_span_width = *span_list.get(&(row, start)).expect("must be there");
    let range_width = range_width(gp, start, end, widths);

    if range_width >= max_span_width {
        return;
    }

    inc_range(widths, max_span_width - range_width, start, end);
}

fn get_cell_width(text: &str, cfg: &GridConfig, pos: Position, wctrl: &CfgWidthFunc) -> usize {
    let padding = get_cell_padding(cfg, pos); // todo: remove it...
    let width = wctrl.width_multiline(text);
    width + padding
}

fn get_cell_padding(cfg: &GridConfig, pos: Position) -> usize {
    let padding = cfg.get_padding(pos.into());
    padding.left.size + padding.right.size
}

fn range_width(gp: &GridProjection<'_>, start: usize, end: usize, widths: &[usize]) -> usize {
    let count_borders = count_vertical_borders(gp, start, end);
    let range_width = widths[start..end].iter().sum::<usize>();
    count_borders + range_width
}

fn count_vertical_borders(gp: &GridProjection<'_>, start: usize, end: usize) -> usize {
    (start..end).skip(1).filter(|&i| gp.has_vertical(i)).count()
}

fn build_height<R: Records>(records: R, cfg: &GridConfig) -> Vec<usize> {
    let count_columns = records.count_columns();
    let shape = (usize::MAX, count_columns);
    let gp = GridProjection::with_shape(cfg, shape);

    let mut heights = vec![];
    let mut hspans = HashMap::new();

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col);

            if !gp.is_cell_visible(pos) {
                continue;
            }

            let height = get_cell_height(cell.as_ref(), cfg, pos);

            let has_hspan = !matches!(gp.get_span_row(pos), None | Some(1));
            if has_hspan {
                hspans.insert(pos, height);
            } else {
                row_height = cmp::max(row_height, height);
            }
        }

        heights.push(row_height);
    }

    // recreate projection cause we know the exact number of rows
    let gp = GridProjection::with_shape(cfg, (heights.len(), count_columns));

    adjust_hspans(&gp, &hspans, &mut heights);

    heights
}

fn build_width<R: Records>(records: R, cfg: &GridConfig) -> Vec<usize> {
    let count_columns = records.count_columns();
    let shape = (usize::MAX, count_columns);
    let gp = GridProjection::with_shape(cfg, shape);

    let wctrl = CfgWidthFunc::from_cfg(cfg);

    let mut widths = vec![0; count_columns];
    let mut vspans = HashMap::new();

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col);
            if !gp.is_cell_visible(pos) {
                continue;
            }

            let width = get_cell_width(cell.as_ref(), cfg, pos, &wctrl);

            let has_vspan = !matches!(gp.get_span_column(pos), None | Some(1));
            if has_vspan {
                vspans.insert(pos, width);
            } else {
                widths[col] = max(widths[col], width);
            }
        }
    }

    adjust_vspans(&gp, &vspans, &mut widths);

    widths
}
