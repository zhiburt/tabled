//! The module contains a [`PeekableGrid`] structure.

use core::borrow::Borrow;
use std::{
    borrow::Cow,
    cmp,
    fmt::{self, Write},
};

use crate::{
    color::{AnsiColor, Color},
    colors::Colors,
    config::spanned::{Formatting, Offset, SpannedConfig},
    config::{AlignmentHorizontal, AlignmentVertical, Indent, Position, Sides},
    dimension::Dimension,
    records::{ExactRecords, PeekableRecords, Records},
    util::string::string_width,
};

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct PeekableGrid<R, G, D, C> {
    records: R,
    config: G,
    dimension: D,
    colors: C,
}

impl<R, G, D, C> PeekableGrid<R, G, D, C> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: G, dimension: D, colors: C) -> Self {
        PeekableGrid {
            records,
            config,
            dimension,
            colors,
        }
    }
}

impl<R, G, D, C> PeekableGrid<R, G, D, C> {
    /// Builds a table.
    pub fn build<F>(self, mut f: F) -> fmt::Result
    where
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
        C: Colors,
        G: Borrow<SpannedConfig>,
        F: Write,
    {
        if self.records.count_columns() == 0 || self.records.hint_count_rows() == Some(0) {
            return Ok(());
        }

        let config = self.config.borrow();
        print_grid(&mut f, self.records, config, &self.dimension, &self.colors)
    }

    /// Builds a table into string.
    ///
    /// Notice that it consumes self.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String
    where
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
        G: Borrow<SpannedConfig>,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranteed to never happen otherwise it's considered an stdlib error or impl error");
        buf
    }
}

fn print_grid<F: Write, R: Records + PeekableRecords + ExactRecords, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &SpannedConfig,
    dimension: &D,
    colors: &C,
) -> fmt::Result {
    if cfg.has_column_spans() || cfg.has_row_spans() {
        build_grid_spanned(f, &records, cfg, dimension, colors)
    } else {
        build_grid(f, &records, cfg, dimension, colors)
    }
}

fn build_grid<F: Write, R: Records + PeekableRecords + ExactRecords, D: Dimension, C: Colors>(
    f: &mut F,
    records: &R,
    cfg: &SpannedConfig,
    dimension: &D,
    colors: &C,
) -> fmt::Result {
    let shape = (records.count_rows(), records.count_columns());

    let total_width = total_width(cfg, dimension, shape.1);
    let total_width_with_margin =
        total_width + cfg.get_margin().left.size + cfg.get_margin().right.size;

    let total_height = total_height(cfg, dimension, shape.0);

    if cfg.get_margin().top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut table_line = 0;
    let mut prev_empty_horizontal = false;
    for row in 0..shape.0 {
        let height = dimension.get_height(row);

        if cfg.has_horizontal(row, shape.0) {
            if prev_empty_horizontal {
                f.write_char('\n')?;
            }

            print_margin_left(f, cfg, table_line, total_height)?;
            print_split_line(f, cfg, dimension, row, shape)?;
            print_margin_right(f, cfg, table_line, total_height)?;

            if height > 0 {
                f.write_char('\n')?;
                prev_empty_horizontal = false;
            } else {
                prev_empty_horizontal = true;
            }

            table_line += 1;
        } else if height > 0 && prev_empty_horizontal {
            f.write_char('\n')?;
            prev_empty_horizontal = false;
        }

        for i in 0..height {
            print_margin_left(f, cfg, table_line, total_height)?;

            for col in 0..records.count_columns() {
                print_vertical_char(f, cfg, (row, col), i, height, shape.1)?;

                let width = dimension.get_width(col);
                print_cell_line(f, records, cfg, colors, width, height, (row, col), i)?;

                let is_last_column = col + 1 == records.count_columns();
                if is_last_column {
                    print_vertical_char(f, cfg, (row, col + 1), i, height, shape.1)?;
                }
            }

            print_margin_right(f, cfg, table_line, total_height)?;

            let is_last_line = i + 1 == height;
            let is_last_row = row + 1 == records.count_rows();
            if !(is_last_line && is_last_row) {
                f.write_char('\n')?;
            }

            table_line += 1;
        }
    }

    if cfg.has_horizontal(shape.0, shape.0) {
        f.write_char('\n')?;
        print_margin_left(f, cfg, table_line, total_height)?;
        print_split_line(f, cfg, dimension, records.count_rows(), shape)?;
        print_margin_right(f, cfg, table_line, total_height)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;
        print_margin_bottom(f, cfg, total_width_with_margin)?;
    }

    Ok(())
}

fn print_split_line<F: Write, D: Dimension>(
    f: &mut F,
    cfg: &SpannedConfig,
    dimension: &D,
    row: usize,
    shape: (usize, usize),
) -> fmt::Result {
    let mut used_color = None;
    print_vertical_intersection(f, cfg, (row, 0), shape, &mut used_color)?;

    for col in 0..shape.1 {
        let width = dimension.get_width(col);

        // general case
        if width > 0 {
            let pos = (row, col);
            let main = cfg.get_horizontal(pos, shape.0);
            match main {
                Some(c) => {
                    let clr = cfg.get_horizontal_color(pos, shape.0);
                    prepare_coloring(f, clr, &mut used_color)?;
                    print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                }
                None => repeat_char(f, ' ', width)?,
            }
        }

        print_vertical_intersection(f, cfg, (row, col + 1), shape, &mut used_color)?;
    }

    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_vertical_intersection<'a, F: fmt::Write>(
    f: &mut F,
    cfg: &'a SpannedConfig,
    pos: Position,
    shape: (usize, usize),
    used_color: &mut Option<&'a AnsiColor<'static>>,
) -> fmt::Result {
    match cfg.get_intersection(pos, shape) {
        Some(c) => {
            let clr = cfg.get_intersection_color(pos, shape);
            prepare_coloring(f, clr, used_color)?;
            f.write_char(c)
        }
        None => Ok(()),
    }
}

fn prepare_coloring<'a, 'b, F: Write>(
    f: &mut F,
    clr: Option<&'a AnsiColor<'b>>,
    used_color: &mut Option<&'a AnsiColor<'b>>,
) -> fmt::Result {
    match clr {
        Some(clr) => match used_color.as_mut() {
            Some(used_clr) => {
                if **used_clr != *clr {
                    used_clr.fmt_suffix(f)?;
                    clr.fmt_prefix(f)?;
                    *used_clr = clr;
                }
            }
            None => {
                clr.fmt_prefix(f)?;
                *used_color = Some(clr);
            }
        },
        None => {
            if let Some(clr) = used_color.take() {
                clr.fmt_suffix(f)?
            }
        }
    }

    Ok(())
}

fn print_vertical_char<F: Write>(
    f: &mut F,
    cfg: &SpannedConfig,
    pos: Position,
    line: usize,
    count_lines: usize,
    count_columns: usize,
) -> fmt::Result {
    let symbol = match cfg.get_vertical(pos, count_columns) {
        Some(c) => c,
        None => return Ok(()),
    };

    let symbol = cfg
        .is_overridden_vertical(pos)
        .then(|| cfg.lookup_vertical_char(pos, line, count_lines))
        .flatten()
        .unwrap_or(symbol);

    match cfg.get_vertical_color(pos, count_columns) {
        Some(clr) => {
            clr.fmt_prefix(f)?;
            f.write_char(symbol)?;
            clr.fmt_suffix(f)?;
        }
        None => f.write_char(symbol)?,
    }

    Ok(())
}

fn build_grid_spanned<
    F: Write,
    R: Records + PeekableRecords + ExactRecords,
    D: Dimension,
    C: Colors,
>(
    f: &mut F,
    records: &R,
    cfg: &SpannedConfig,
    dims: &D,
    colors: &C,
) -> fmt::Result {
    let shape = (records.count_rows(), records.count_columns());

    let total_width = total_width(cfg, dims, shape.1);
    let total_width_with_margin =
        total_width + cfg.get_margin().left.size + cfg.get_margin().right.size;

    let total_height = total_height(cfg, dims, shape.0);

    if cfg.get_margin().top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut table_line = 0;
    let mut prev_empty_horizontal = false;
    for row in 0..records.count_rows() {
        let count_lines = dims.get_height(row);

        if cfg.has_horizontal(row, shape.0) {
            if prev_empty_horizontal {
                f.write_char('\n')?;
            }

            print_margin_left(f, cfg, table_line, total_height)?;
            print_split_line_spanned(f, records, cfg, dims, colors, row, shape)?;
            print_margin_right(f, cfg, table_line, total_height)?;

            if count_lines > 0 {
                f.write_char('\n')?;
                prev_empty_horizontal = false;
            } else {
                prev_empty_horizontal = true;
            }

            table_line += 1;
        } else if count_lines > 0 && prev_empty_horizontal {
            f.write_char('\n')?;
            prev_empty_horizontal = false;
        }

        for i in 0..count_lines {
            print_margin_left(f, cfg, table_line, total_height)?;

            for col in 0..records.count_columns() {
                if cfg.is_cell_covered_by_both_spans((row, col)) {
                    continue;
                }

                if cfg.is_cell_covered_by_column_span((row, col)) {
                    let is_last_column = col + 1 == records.count_columns();
                    if is_last_column {
                        print_vertical_char(f, cfg, (row, col + 1), i, count_lines, shape.1)?;
                    }

                    continue;
                }

                print_vertical_char(f, cfg, (row, col), i, count_lines, shape.1)?;

                if cfg.is_cell_covered_by_row_span((row, col)) {
                    // means it's part of other a spanned cell
                    // so. we just need to use line from other cell.
                    let original_row = closest_visible_row(cfg, (row, col)).unwrap();

                    // considering that the content will be printed instead horizontal lines so we can skip some lines.
                    let mut skip_lines = (original_row..row)
                        .map(|i| dims.get_height(i))
                        .sum::<usize>();

                    skip_lines += (original_row + 1..=row)
                        .map(|row| cfg.has_horizontal(row, shape.0) as usize)
                        .sum::<usize>();

                    let line = i + skip_lines;
                    let pos = (original_row, col);

                    let width = get_cell_width(cfg, dims, pos, shape.1);
                    let height = get_cell_height(cfg, dims, pos, shape.0);

                    print_cell_line(f, records, cfg, colors, width, height, pos, line)?;
                } else {
                    let width = get_cell_width(cfg, dims, (row, col), shape.1);
                    let height = get_cell_height(cfg, dims, (row, col), shape.0);
                    print_cell_line(f, records, cfg, colors, width, height, (row, col), i)?;
                }

                let is_last_column = col + 1 == records.count_columns();
                if is_last_column {
                    print_vertical_char(f, cfg, (row, col + 1), i, count_lines, shape.1)?;
                }
            }

            print_margin_right(f, cfg, table_line, total_height)?;

            let is_last_line = i + 1 == count_lines;
            let is_last_row = row + 1 == records.count_rows();
            if !(is_last_line && is_last_row) {
                f.write_char('\n')?;
            }

            table_line += 1;
        }
    }

    if cfg.has_horizontal(shape.0, shape.0) {
        f.write_char('\n')?;
        print_margin_left(f, cfg, table_line, total_height)?;
        print_split_line(f, cfg, dims, records.count_rows(), shape)?;
        print_margin_right(f, cfg, table_line, total_height)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;
        print_margin_bottom(f, cfg, total_width_with_margin)?;
    }

    Ok(())
}

fn print_split_line_spanned<
    F: Write,
    R: Records + ExactRecords + PeekableRecords,
    D: Dimension,
    C: Colors,
>(
    f: &mut F,
    records: &R,
    cfg: &SpannedConfig,
    dims: &D,
    colors: &C,
    row: usize,
    shape: (usize, usize),
) -> fmt::Result {
    let mut used_color = None;
    print_vertical_intersection(f, cfg, (row, 0), shape, &mut used_color)?;

    for col in 0..shape.1 {
        let pos = (row, col);
        if cfg.is_cell_covered_by_both_spans(pos) {
            continue;
        }

        if cfg.is_cell_covered_by_row_span(pos) {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let original_row = closest_visible_row(cfg, (row, col)).unwrap();

            // considering that the content will be printed instead horizontal lines so we can skip some lines.
            let mut skip_lines = (original_row..row)
                .map(|i| dims.get_height(i))
                .sum::<usize>();

            // skip horizontal lines
            if row > 0 {
                skip_lines += (original_row..row - 1)
                    .map(|row| cfg.has_horizontal(row + 1, shape.0) as usize)
                    .sum::<usize>();
            }

            let pos = (original_row, col);
            let height = get_cell_height(cfg, dims, pos, shape.0);
            let width = get_cell_width(cfg, dims, pos, shape.1);
            let line = skip_lines;

            print_cell_line(f, records, cfg, colors, width, height, pos, line)?;

            // We need to use a correct right split char.
            let mut col = col;
            if let Some(span) = cfg.get_column_span(pos) {
                col += span - 1;
            }

            print_vertical_intersection(f, cfg, (row, col + 1), shape, &mut used_color)?;

            continue;
        }

        let width = dims.get_width(col);
        if width > 0 {
            // general case
            let main = cfg.get_horizontal(pos, shape.0);
            match main {
                Some(c) => {
                    let clr = cfg.get_horizontal_color(pos, shape.0);
                    prepare_coloring(f, clr, &mut used_color)?;
                    print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                }
                None => repeat_char(f, ' ', width)?,
            }
        }

        print_vertical_intersection(f, cfg, (row, col + 1), shape, &mut used_color)?;
    }

    if let Some(clr) = used_color {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_horizontal_border<F: Write>(
    f: &mut F,
    cfg: &SpannedConfig,
    pos: Position,
    width: usize,
    c: char,
    used_color: &Option<&AnsiColor<'static>>,
) -> fmt::Result {
    if !cfg.is_overridden_horizontal(pos) {
        return repeat_char(f, c, width);
    }

    for i in 0..width {
        let c = cfg.lookup_horizontal_char(pos, i, width).unwrap_or(c);
        match cfg.lookup_horizontal_color(pos, i, width) {
            Some(color) => match used_color {
                Some(clr) => {
                    clr.fmt_suffix(f)?;
                    color.fmt_prefix(f)?;
                    f.write_char(c)?;
                    color.fmt_suffix(f)?;
                    clr.fmt_prefix(f)?;
                }
                None => {
                    color.fmt_prefix(f)?;
                    f.write_char(c)?;
                    color.fmt_suffix(f)?;
                }
            },
            _ => f.write_char(c)?,
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_cell_line<F: Write, R: Records + PeekableRecords + ExactRecords, C: Colors>(
    f: &mut F,
    records: &R,
    cfg: &SpannedConfig,
    colors: &C,
    width: usize,
    height: usize,
    pos: Position,
    line: usize,
) -> fmt::Result {
    let entity = pos.into();

    let mut cell_height = records.count_lines(pos);
    let formatting = *cfg.get_formatting(entity);
    if formatting.vertical_trim {
        cell_height -=
            count_empty_lines_at_start(records, pos) + count_empty_lines_at_end(records, pos);
    }

    if cell_height > height {
        // it may happen if the height estimation decide so
        cell_height = height;
    }

    let pad = cfg.get_padding(entity);
    let pad_color = cfg.get_padding_color(entity);
    let alignment = cfg.get_alignment_vertical(entity);
    let indent = top_indent(&pad, *alignment, cell_height, height);
    if indent > line {
        return print_indent(f, pad.top.fill, width, pad_color.top.as_ref());
    }

    let mut index = line - indent;
    let cell_has_this_line = cell_height > index;
    if !cell_has_this_line {
        // happens when other cells have bigger height
        return print_indent(f, pad.bottom.fill, width, pad_color.bottom.as_ref());
    }

    if formatting.vertical_trim {
        let empty_lines = count_empty_lines_at_start(records, pos);
        index += empty_lines;

        if index > records.count_lines(pos) {
            return print_indent(f, pad.top.fill, width, pad_color.top.as_ref());
        }
    }

    print_indent(f, pad.left.fill, pad.left.size, pad_color.left.as_ref())?;

    let width = width - pad.left.size - pad.right.size;
    let alignment = *cfg.get_alignment_horizontal(entity);
    let justification = (
        cfg.get_justification(entity),
        cfg.get_justification_color(entity),
    );
    let color = colors.get_color(pos);
    print_line(
        f,
        records,
        pos,
        index,
        alignment,
        formatting,
        color,
        justification,
        width,
    )?;

    print_indent(f, pad.right.fill, pad.right.size, pad_color.right.as_ref())?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_line<F: Write, R: Records + PeekableRecords, C: Color>(
    f: &mut F,
    records: &R,
    pos: Position,
    index: usize,
    alignment: AlignmentHorizontal,
    formatting: Formatting,
    color: Option<C>,
    justification: (char, Option<&AnsiColor<'_>>),
    available: usize,
) -> fmt::Result {
    let line = records.get_line(pos, index);
    let (line, line_width) = if formatting.horizontal_trim {
        let line = string_trim(line);
        let width = string_width(&line);
        (line, width)
    } else {
        let width = records.get_line_width(pos, index);
        (Cow::Borrowed(line), width)
    };

    if formatting.allow_lines_alignment {
        let (left, right) = calculate_indent(alignment, line_width, available);
        return print_text_with_pad(f, &line, color, justification, left, right);
    }

    let cell_width = if formatting.horizontal_trim {
        (0..records.count_lines(pos))
            .map(|i| records.get_line(pos, i))
            .map(|line| string_width(line.trim()))
            .max()
            .unwrap_or_default()
    } else {
        records.get_width(pos)
    };

    let (left, right) = calculate_indent(alignment, cell_width, available);
    print_text_with_pad(f, &line, color, justification, left, right)?;

    // todo: remove me
    let rest_width = cell_width - line_width;
    repeat_char(f, ' ', rest_width)?;

    Ok(())
}

fn print_text_with_pad<F: Write, C: Color>(
    f: &mut F,
    text: &str,
    color: Option<C>,
    justification: (char, Option<&AnsiColor<'_>>),
    left: usize,
    right: usize,
) -> fmt::Result {
    print_indent(f, justification.0, left, justification.1)?;
    print_text(f, text, color)?;
    print_indent(f, justification.0, right, justification.1)?;
    Ok(())
}

fn print_text<F: Write, C: Color>(f: &mut F, text: &str, clr: Option<C>) -> fmt::Result {
    match clr {
        Some(color) => {
            color.fmt_prefix(f)?;
            f.write_str(text)?;
            color.fmt_suffix(f)
        }
        None => f.write_str(text),
    }
}

fn top_indent(
    pad: &Sides<Indent>,
    alignment: AlignmentVertical,
    cell_height: usize,
    available: usize,
) -> usize {
    let height = available - pad.top.size;
    let indent = indent_from_top(alignment, height, cell_height);

    indent + pad.top.size
}

fn indent_from_top(alignment: AlignmentVertical, available: usize, real: usize) -> usize {
    match alignment {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - real,
        AlignmentVertical::Center => (available - real) / 2,
    }
}

fn calculate_indent(
    alignment: AlignmentHorizontal,
    text_width: usize,
    available: usize,
) -> (usize, usize) {
    let diff = available - text_width;
    match alignment {
        AlignmentHorizontal::Left => (0, diff),
        AlignmentHorizontal::Right => (diff, 0),
        AlignmentHorizontal::Center => {
            let left = diff / 2;
            let rest = diff - left;
            (left, rest)
        }
    }
}

fn repeat_char<F: Write>(f: &mut F, c: char, n: usize) -> fmt::Result {
    for _ in 0..n {
        f.write_char(c)?;
    }

    Ok(())
}

fn count_empty_lines_at_end<R>(records: &R, pos: Position) -> usize
where
    R: Records + PeekableRecords,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .rev()
        .take_while(|l| l.trim().is_empty())
        .count()
}

fn count_empty_lines_at_start<R>(records: &R, pos: Position) -> usize
where
    R: Records + PeekableRecords,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .take_while(|s| s.trim().is_empty())
        .count()
}

fn total_width<D: Dimension>(cfg: &SpannedConfig, dimension: &D, count_columns: usize) -> usize {
    (0..count_columns)
        .map(|i| dimension.get_width(i))
        .sum::<usize>()
        + cfg.count_vertical(count_columns)
}

fn total_height<D: Dimension>(cfg: &SpannedConfig, dimension: &D, count_rows: usize) -> usize {
    (0..count_rows)
        .map(|i| dimension.get_height(i))
        .sum::<usize>()
        + cfg.count_horizontal(count_rows)
}

fn print_margin_top<F: Write>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result {
    let indent = cfg.get_margin().top;
    let offset = cfg.get_margin_offset().top;
    let color = cfg.get_margin_color();
    let color = color.top.as_ref();
    print_indent_lines(f, &indent, &offset, color, width)
}

fn print_margin_bottom<F: Write>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result {
    let indent = cfg.get_margin().bottom;
    let offset = cfg.get_margin_offset().bottom;
    let color = cfg.get_margin_color();
    let color = color.bottom.as_ref();
    print_indent_lines(f, &indent, &offset, color, width)
}

fn print_margin_left<F: Write>(
    f: &mut F,
    cfg: &SpannedConfig,
    line: usize,
    height: usize,
) -> fmt::Result {
    let indent = cfg.get_margin().left;
    let offset = cfg.get_margin_offset().left;
    let color = cfg.get_margin_color();
    let color = color.left.as_ref();
    print_margin_vertical(f, indent, offset, color, line, height)
}

fn print_margin_right<F: Write>(
    f: &mut F,
    cfg: &SpannedConfig,
    line: usize,
    height: usize,
) -> fmt::Result {
    let indent = cfg.get_margin().right;
    let offset = cfg.get_margin_offset().right;
    let color = cfg.get_margin_color();
    let color = color.right.as_ref();
    print_margin_vertical(f, indent, offset, color, line, height)
}

fn print_margin_vertical<F: Write>(
    f: &mut F,
    indent: Indent,
    offset: Offset,
    color: Option<&AnsiColor<'_>>,
    line: usize,
    height: usize,
) -> fmt::Result {
    if indent.size == 0 {
        return Ok(());
    }

    match offset {
        Offset::Begin(offset) => {
            let offset = cmp::min(offset, height);
            if line >= offset {
                print_indent(f, indent.fill, indent.size, color)?;
            } else {
                repeat_char(f, ' ', indent.size)?;
            }
        }
        Offset::End(offset) => {
            let offset = cmp::min(offset, height);
            let pos = height - offset;

            if line >= pos {
                repeat_char(f, ' ', indent.size)?;
            } else {
                print_indent(f, indent.fill, indent.size, color)?;
            }
        }
    }

    Ok(())
}

fn print_indent_lines<F: Write>(
    f: &mut F,
    indent: &Indent,
    offset: &Offset,
    color: Option<&AnsiColor<'_>>,
    width: usize,
) -> fmt::Result {
    if indent.size == 0 {
        return Ok(());
    }

    let (start_offset, end_offset) = match offset {
        Offset::Begin(start) => (*start, 0),
        Offset::End(end) => (0, *end),
    };

    let start_offset = std::cmp::min(start_offset, width);
    let end_offset = std::cmp::min(end_offset, width);
    let indent_size = width - start_offset - end_offset;

    for i in 0..indent.size {
        if start_offset > 0 {
            repeat_char(f, ' ', start_offset)?;
        }

        if indent_size > 0 {
            print_indent(f, indent.fill, indent_size, color)?;
        }

        if end_offset > 0 {
            repeat_char(f, ' ', end_offset)?;
        }

        if i + 1 != indent.size {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn print_indent<F: Write, C: Color>(f: &mut F, c: char, n: usize, color: Option<C>) -> fmt::Result {
    if n == 0 {
        return Ok(());
    }

    match color {
        Some(color) => {
            color.fmt_prefix(f)?;
            repeat_char(f, c, n)?;
            color.fmt_suffix(f)
        }
        None => repeat_char(f, c, n),
    }
}

fn get_cell_width<D: Dimension>(cfg: &SpannedConfig, dims: &D, pos: Position, max: usize) -> usize {
    match cfg.get_column_span(pos) {
        Some(span) => {
            let start = pos.1;
            let end = pos.1 + span;
            range_width(dims, start, end) + count_verticals_range(cfg, start, end, max)
        }
        None => dims.get_width(pos.1),
    }
}

fn range_width<D: Dimension>(dims: &D, start: usize, end: usize) -> usize {
    (start..end).map(|col| dims.get_width(col)).sum::<usize>()
}

fn count_verticals_range(cfg: &SpannedConfig, start: usize, end: usize, max: usize) -> usize {
    (start + 1..end)
        .map(|i| cfg.has_vertical(i, max) as usize)
        .sum()
}

fn get_cell_height<D: Dimension>(
    cfg: &SpannedConfig,
    dims: &D,
    pos: Position,
    max: usize,
) -> usize {
    match cfg.get_row_span(pos) {
        Some(span) => {
            let start = pos.0;
            let end = pos.0 + span;
            range_height(dims, start, end) + count_horizontals_range(cfg, start, end, max)
        }
        None => dims.get_height(pos.0),
    }
}

fn range_height<D: Dimension>(dims: &D, start: usize, end: usize) -> usize {
    (start..end).map(|col| dims.get_height(col)).sum::<usize>()
}

fn count_horizontals_range(cfg: &SpannedConfig, start: usize, end: usize, max: usize) -> usize {
    (start + 1..end)
        .map(|i| cfg.has_horizontal(i, max) as usize)
        .sum()
}

fn closest_visible_row(cfg: &SpannedConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.0);
        }

        if pos.0 == 0 {
            return None;
        }

        pos.0 -= 1;
    }
}

/// Trims a string.
fn string_trim(text: &str) -> Cow<'_, str> {
    #[cfg(feature = "color")]
    {
        ansi_str::AnsiStr::ansi_trim(text)
    }

    #[cfg(not(feature = "color"))]
    {
        text.trim().into()
    }
}
