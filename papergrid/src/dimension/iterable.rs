//! The module contains a [`IterGridDimension`].

use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

use crate::{
    config::{spanned::SpannedConfig, Position},
    dimension::{Dimension, Estimate},
    records::{IntoRecords, Records},
    util::string::{count_lines, get_text_dimension, get_text_width},
};

/// A [`Dimension`] implementation which calculates exact column/row width/height.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IterGridDimension {
    height: Vec<usize>,
    width: Vec<usize>,
}

impl IterGridDimension {
    /// Calculates height of rows.
    pub fn height<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        build_height(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        build_width(records, cfg)
    }

    /// Calculates width of columns.
    pub fn width_total<R>(records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        get_width_total(records, cfg)
    }

    /// Calculates height of rows.
    pub fn height_total<R>(records: R, cfg: &SpannedConfig) -> usize
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
    {
        get_height_total(records, cfg)
    }

    /// Return width and height lists.
    pub fn get_values(self) -> (Vec<usize>, Vec<usize>) {
        (self.width, self.height)
    }
}

impl Dimension for IterGridDimension {
    fn get_width(&self, column: usize) -> usize {
        self.width[column]
    }

    fn get_height(&self, row: usize) -> usize {
        self.height[row]
    }
}

impl<R> Estimate<R, SpannedConfig> for IterGridDimension
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn estimate(&mut self, records: R, cfg: &SpannedConfig) {
        let (width, height) = build_dimensions(records, cfg);
        self.width = width;
        self.height = height;
    }
}

fn build_dimensions<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, Vec<usize>)
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    if cfg.has_row_spans() || cfg.has_column_spans() {
        build_dimensions_spanned(records, cfg)
    } else {
        build_dimensions_basic(records, cfg)
    }
}

fn build_dimensions_basic<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, Vec<usize>)
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_columns = records.count_columns();

    let mut widths = vec![0; count_columns];
    let mut heights = vec![];

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let (height, width) = get_text_dimension(cell.as_ref());

            let pad = cfg.get_padding(Position::new(row, col));
            let width = width + pad.left.size + pad.right.size;
            let height = height + pad.top.size + pad.bottom.size;

            widths[col] = max(widths[col], width);
            row_height = max(row_height, height);
        }

        heights.push(row_height);
    }

    (widths, heights)
}

fn build_dimensions_spanned<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, Vec<usize>)
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_columns = records.count_columns();

    let mut widths = vec![0; count_columns];
    let mut heights = vec![];

    let mut vspans = HashMap::new();
    let mut hspans = HashMap::new();

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let text = cell.as_ref();
            let (height, width) = get_text_dimension(text);

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

    let mut spans_ordered = spans.iter().map(|(k, v)| (k, *v)).collect::<Vec<_>>();
    spans_ordered.sort_unstable_by(|(arow, acol), (brow, bcol)| match arow.cmp(brow) {
        Ordering::Equal => acol.cmp(bcol),
        ord => ord,
    });

    for (pos, (span, height)) in spans_ordered {
        adjust_row_range(cfg, height, len, pos.row, pos.row + span, heights);
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

fn get_cell_height(cell: &str, cfg: &SpannedConfig, pos: Position) -> usize {
    let count_lines = max(1, count_lines(cell));
    let padding = cfg.get_padding(pos);
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
    let mut spans_ordered = spans.iter().map(|(k, v)| (k, *v)).collect::<Vec<_>>();
    spans_ordered.sort_unstable_by(|a, b| match a.1 .0.cmp(&b.1 .0) {
        Ordering::Equal => a.0.cmp(b.0),
        o => o,
    });

    for (pos, (span, width)) in spans_ordered {
        adjust_column_range(cfg, width, len, pos.col, pos.col + span, widths);
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

fn get_cell_width(text: &str, cfg: &SpannedConfig, pos: Position) -> usize {
    let padding = get_cell_padding(cfg, pos);
    let width = get_text_width(text);
    width + padding
}

fn get_cell_padding(cfg: &SpannedConfig, pos: Position) -> usize {
    let padding = cfg.get_padding(pos);
    padding.left.size + padding.right.size
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

fn build_height<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    if cfg.has_row_spans() {
        build_height_spanned(records, cfg)
    } else {
        build_height_basic(records, cfg)
    }
}

fn build_height_basic<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let mut heights = vec![];

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col).into();
            let height = get_cell_height(cell.as_ref(), cfg, pos);
            row_height = max(row_height, height);
        }

        heights.push(row_height);
    }

    heights
}

fn build_height_spanned<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let mut heights = vec![];
    let mut hspans = HashMap::new();

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        let mut row_height = 0;
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let height = get_cell_height(cell.as_ref(), cfg, pos);
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

fn build_width<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    if cfg.has_column_spans() {
        build_width_spanned(records, cfg)
    } else {
        build_width_basic(records, cfg)
    }
}

fn build_width_basic<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_columns = records.count_columns();
    let mut widths = vec![0; count_columns];

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col).into();
            let width = get_cell_width(cell.as_ref(), cfg, pos);
            widths[col] = max(widths[col], width);
        }
    }

    widths
}

fn build_width_spanned<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_columns = records.count_columns();

    let mut widths = vec![0; count_columns];
    let mut vspans = HashMap::new();

    for (row, columns) in records.iter_rows().into_iter().enumerate() {
        for (col, cell) in columns.into_iter().enumerate() {
            let pos = (row, col).into();
            if !cfg.is_cell_visible(pos) {
                continue;
            }

            let width = get_cell_width(cell.as_ref(), cfg, pos);
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

fn get_width_total<R>(records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let widths = build_width(records, cfg);
    let count_columns = widths.len();

    let total = widths.into_iter().sum::<usize>();
    let count_verticals = cfg.count_vertical(count_columns);

    total + count_verticals
}

fn get_height_total<R>(records: R, cfg: &SpannedConfig) -> usize
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let heights = build_height(records, cfg);
    let count_rows = heights.len();

    let total = heights.into_iter().sum::<usize>();
    let count_horizontals = cfg.count_horizontal(count_rows);

    total + count_horizontals
}
