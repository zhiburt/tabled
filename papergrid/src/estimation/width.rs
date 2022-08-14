use std::cmp::Ordering;

use crate::{
    grid::GridConfig,
    records::Records,
    util::{string_width_multiline_tab, string_width_tab},
    Position,
};

use super::Estimate;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WidthEstimator {
    widths: Vec<usize>,
}

impl<R> Estimate<R> for WidthEstimator
where
    R: Records,
{
    fn estimate(&mut self, records: R, cfg: &GridConfig) {
        self.widths = build_widths(&records, cfg);
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

pub trait WidthFunc {
    fn width(&self, text: &str) -> usize;
    fn width_multiline(&self, text: &str) -> usize;
}

impl<W> WidthFunc for &W
where
    W: WidthFunc,
{
    fn width(&self, text: &str) -> usize {
        W::width(self, text)
    }

    fn width_multiline(&self, text: &str) -> usize {
        W::width_multiline(self, text)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CfgWidthFunction {
    tab_width: usize,
}

impl CfgWidthFunction {
    pub fn from_cfg(cfg: &GridConfig) -> Self {
        Self::new(cfg.get_tab_width())
    }

    pub fn new(tab_size: usize) -> Self {
        Self {
            tab_width: tab_size,
        }
    }
}

impl WidthFunc for CfgWidthFunction {
    fn width(&self, text: &str) -> usize {
        string_width_tab(text, self.tab_width)
    }

    fn width_multiline(&self, text: &str) -> usize {
        string_width_multiline_tab(text, self.tab_width)
    }
}

fn build_widths<R>(records: &R, cfg: &GridConfig) -> Vec<usize>
where
    R: Records,
{
    let mut widths = vec![0; records.count_columns()];
    for (col, column) in widths.iter_mut().enumerate() {
        let max = (0..records.count_rows())
            .filter(|&row| is_simple_cell(cfg, (row, col)))
            .map(|row| get_cell_width(cfg, records, (row, col)))
            .max()
            .unwrap_or(0);

        *column = max;
    }

    adjust_spans(cfg, records, &mut widths);

    widths
}

fn adjust_spans<R>(cfg: &GridConfig, records: &R, widths: &mut [usize])
where
    R: Records,
{
    if !cfg.has_column_spans() {
        return;
    }

    // The overall width disctribution will be different depend on the order.
    //
    // We sort spans in order to prioritize the smaller spans first.
    let mut spans = cfg.iter_column_spans().collect::<Vec<_>>();
    spans.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });

    // todo: the order is matter here; we need to figure out what is correct.
    for ((row, col), span) in spans {
        adjust_range(cfg, records, row, col, col + span, widths);
    }
}

fn adjust_range<R>(
    cfg: &GridConfig,
    records: &R,
    row: usize,
    start: usize,
    end: usize,
    widths: &mut [usize],
) where
    R: Records,
{
    let max_span_width = get_cell_width(cfg, records, (row, start));
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

fn is_simple_cell(cfg: &GridConfig, pos: Position) -> bool {
    cfg.is_cell_visible(pos) && matches!(cfg.get_column_span(pos), None | Some(1))
}

fn get_cell_width<R>(cfg: &GridConfig, records: &R, pos: Position) -> usize
where
    R: Records,
{
    let width = records.get_width(pos, CfgWidthFunction::from_cfg(cfg));
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
