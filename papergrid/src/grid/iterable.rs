//! The module contains a [`Grid`] structure.

use std::{
    borrow::{Borrow, Cow},
    cmp,
    collections::BTreeMap,
    fmt::{self, Write},
};

use crate::{
    color::{AnsiColor, Color},
    colors::Colors,
    config::{AlignmentHorizontal, AlignmentVertical, Indent, Position, Sides},
    dimension::Dimension,
    records::Records,
    util::string::{count_lines, get_lines, string_width, string_width_multiline, Lines},
};

use crate::config::spanned::{Formatting, Offset, SpannedConfig};

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct Grid<R, D, G, C> {
    records: R,
    config: G,
    dimension: D,
    colors: C,
}

impl<R, D, G, C> Grid<R, D, G, C> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, dimension: D, config: G, colors: C) -> Self {
        Grid {
            records,
            config,
            dimension,
            colors,
        }
    }
}

impl<R, D, G, C> Grid<R, D, G, C> {
    /// Builds a table.
    pub fn build<F>(self, mut f: F) -> fmt::Result
    where
        R: Records,
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
        R: Records,
        D: Dimension,
        G: Borrow<SpannedConfig>,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranteed to never happen otherwise it's considered an stdlib error or impl error");
        buf
    }
}

fn print_grid<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &SpannedConfig,
    dimension: &D,
    colors: &C,
) -> fmt::Result {
    // spanned version is a bit more complex and 'supposedly' slower,
    // because spans are considered to be not a general case we are having 2 versions
    let grid_has_spans = cfg.has_column_spans() || cfg.has_row_spans();
    if grid_has_spans {
        print_grid_spanned(f, records, cfg, dimension, colors)
    } else {
        print_grid_general(f, records, cfg, dimension, colors)
    }
}

fn print_grid_general<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &SpannedConfig,
    dims: &D,
    colors: &C,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let mut totalw = None;
    let totalh = records
        .hint_count_rows()
        .map(|count_rows| total_height(cfg, dims, count_rows));

    let mut records_iter = records.iter_rows().into_iter();
    let mut next_columns = records_iter.next();

    if next_columns.is_none() {
        return Ok(());
    }

    if cfg.get_margin().top.size > 0 {
        totalw = Some(output_width(cfg, dims, count_columns));

        print_margin_top(f, cfg, totalw.unwrap())?;
        f.write_char('\n')?;
    }

    let mut row = 0;
    let mut line = 0;
    let mut is_prev_row_skipped = false;
    let mut buf = None;
    while let Some(columns) = next_columns {
        let columns = columns.into_iter();
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let height = dims.get_height(row);
        let count_rows = convert_count_rows(row, is_last_row);
        let has_horizontal = cfg.has_horizontal(row, count_rows);
        let shape = (count_rows, count_columns);

        if row > 0 && !is_prev_row_skipped && (has_horizontal || height > 0) {
            f.write_char('\n')?;
        }

        if has_horizontal {
            print_horizontal_line(f, cfg, line, totalh, dims, row, shape)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        if height == 1 {
            print_single_line_columns(f, columns, cfg, colors, dims, row, line, totalh, shape)?
        } else if height > 0 {
            if buf.is_none() {
                buf = Some(Vec::with_capacity(count_columns));
            }

            let buf = buf.as_mut().unwrap();
            print_multiline_columns(
                f, columns, cfg, colors, dims, height, row, line, totalh, shape, buf,
            )?;

            buf.clear();
        }

        if height == 0 && !has_horizontal {
            is_prev_row_skipped = true;
        } else {
            is_prev_row_skipped = false;
        }

        line += height;
        row += 1;
    }

    if cfg.has_horizontal(row, row) {
        f.write_char('\n')?;
        let shape = (row, count_columns);
        print_horizontal_line(f, cfg, line, totalh, dims, row, shape)?;
    }

    {
        let margin = cfg.get_margin();
        if margin.bottom.size > 0 {
            let totalw = totalw.unwrap_or_else(|| output_width(cfg, dims, count_columns));

            f.write_char('\n')?;
            print_margin_bottom(f, cfg, totalw)?;
        }
    }

    Ok(())
}

fn output_width<D: Dimension>(cfg: &SpannedConfig, d: D, count_columns: usize) -> usize {
    let margin = cfg.get_margin();
    total_width(cfg, &d, count_columns) + margin.left.size + margin.right.size
}

#[allow(clippy::too_many_arguments)]
fn print_horizontal_line<F: Write, D: Dimension>(
    f: &mut F,
    cfg: &SpannedConfig,
    line: usize,
    totalh: Option<usize>,
    dimension: &D,
    row: usize,
    shape: (usize, usize),
) -> fmt::Result {
    print_margin_left(f, cfg, line, totalh)?;
    print_split_line(f, cfg, dimension, row, shape)?;
    print_margin_right(f, cfg, line, totalh)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_multiline_columns<'a, F, I, D, C>(
    f: &mut F,
    columns: I,
    cfg: &'a SpannedConfig,
    colors: &'a C,
    dimension: &D,
    height: usize,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
    buf: &mut Vec<Cell<I::Item, &'a C::Color>>,
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    collect_columns(buf, columns, cfg, colors, dimension, height, row);
    print_columns_lines(f, buf, height, cfg, line, row, totalh, shape)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_single_line_columns<F, I, D, C>(
    f: &mut F,
    columns: I,
    cfg: &SpannedConfig,
    colors: &C,
    dims: &D,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    print_margin_left(f, cfg, line, totalh)?;

    for (col, cell) in columns.enumerate() {
        let pos = (row, col);
        let width = dims.get_width(col);
        let color = colors.get_color(pos);
        print_vertical_char(f, cfg, pos, 0, 1, shape.1)?;
        print_single_line_column(f, cell.as_ref(), cfg, width, color, pos)?;
    }

    print_vertical_char(f, cfg, (row, shape.1), 0, 1, shape.1)?;

    print_margin_right(f, cfg, line, totalh)?;

    Ok(())
}

fn print_single_line_column<F: Write, C: Color>(
    f: &mut F,
    text: &str,
    cfg: &SpannedConfig,
    width: usize,
    color: Option<&C>,
    pos: Position,
) -> fmt::Result {
    let pos = pos.into();
    let pad = cfg.get_padding(pos);
    let pad_color = cfg.get_padding_color(pos);
    let fmt = cfg.get_formatting(pos);
    let space = cfg.get_justification(pos);
    let space_color = cfg.get_justification_color(pos);

    let (text, text_width) = if fmt.horizontal_trim && !text.is_empty() {
        let text = string_trim(text);
        let width = string_width(&text);

        (text, width)
    } else {
        let text = Cow::Borrowed(text);
        let width = string_width_multiline(&text);

        (text, width)
    };

    let alignment = *cfg.get_alignment_horizontal(pos);
    let available_width = width - pad.left.size - pad.right.size;
    let (left, right) = calculate_indent(alignment, text_width, available_width);

    print_padding(f, &pad.left, pad_color.left.as_ref())?;

    print_indent(f, space, left, space_color)?;
    print_text(f, &text, color)?;
    print_indent(f, space, right, space_color)?;

    print_padding(f, &pad.right, pad_color.right.as_ref())?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_columns_lines<T, F: Write, C: Color>(
    f: &mut F,
    buf: &mut [Cell<T, C>],
    height: usize,
    cfg: &SpannedConfig,
    line: usize,
    row: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> fmt::Result {
    for i in 0..height {
        let exact_line = line + i;

        print_margin_left(f, cfg, exact_line, totalh)?;

        for (col, cell) in buf.iter_mut().enumerate() {
            print_vertical_char(f, cfg, (row, col), i, height, shape.1)?;
            cell.display(f)?;
        }

        print_vertical_char(f, cfg, (row, shape.1), i, height, shape.1)?;

        print_margin_right(f, cfg, exact_line, totalh)?;

        if i + 1 != height {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn collect_columns<'a, I, D, C>(
    buf: &mut Vec<Cell<I::Item, &'a C::Color>>,
    iter: I,
    cfg: &SpannedConfig,
    colors: &'a C,
    dimension: &D,
    height: usize,
    row: usize,
) where
    I: Iterator,
    I::Item: AsRef<str>,
    C: Colors,
    D: Dimension,
{
    let iter = iter.enumerate().map(|(col, cell)| {
        let pos = (row, col);
        let width = dimension.get_width(col);
        let color = colors.get_color(pos);
        Cell::new(cell, width, height, cfg, color, pos)
    });

    buf.extend(iter);
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

fn print_grid_spanned<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &SpannedConfig,
    dims: &D,
    colors: &C,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let total_width = total_width(cfg, dims, count_columns);
    let margin = cfg.get_margin();
    let total_width_with_margin = total_width + margin.left.size + margin.right.size;

    let totalh = records
        .hint_count_rows()
        .map(|rows| total_height(cfg, dims, rows));

    if margin.top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut buf = BTreeMap::new();

    let mut records_iter = records.iter_rows().into_iter();
    let mut next_columns = records_iter.next();

    let mut need_new_line = false;
    let mut line = 0;
    let mut row = 0;
    while let Some(columns) = next_columns {
        let columns = columns.into_iter();
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let height = dims.get_height(row);
        let count_rows = convert_count_rows(row, is_last_row);
        let shape = (count_rows, count_columns);

        let has_horizontal = cfg.has_horizontal(row, count_rows);
        if need_new_line && (has_horizontal || height > 0) {
            f.write_char('\n')?;
            need_new_line = false;
        }

        if has_horizontal {
            print_margin_left(f, cfg, line, totalh)?;
            print_split_line_spanned(f, &mut buf, cfg, dims, row, shape)?;
            print_margin_right(f, cfg, line, totalh)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        print_spanned_columns(
            f, &mut buf, columns, cfg, colors, dims, height, row, line, totalh, shape,
        )?;

        if has_horizontal || height > 0 {
            need_new_line = true;
        }

        line += height;
        row += 1;
    }

    if row > 0 {
        if cfg.has_horizontal(row, row) {
            f.write_char('\n')?;
            let shape = (row, count_columns);
            print_horizontal_line(f, cfg, line, totalh, dims, row, shape)?;
        }

        if margin.bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, cfg, total_width_with_margin)?;
        }
    }

    Ok(())
}

fn print_split_line_spanned<S, F: Write, D: Dimension, C: Color>(
    f: &mut F,
    buf: &mut BTreeMap<usize, (Cell<S, C>, usize, usize)>,
    cfg: &SpannedConfig,
    dimension: &D,
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

        let width = dimension.get_width(col);
        let mut col = col;
        if cfg.is_cell_covered_by_row_span(pos) {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let (cell, _, _) = buf.get_mut(&col).unwrap();
            cell.display(f)?;

            // We need to use a correct right split char.
            let original_row = closest_visible_row(cfg, pos).unwrap();
            if let Some(span) = cfg.get_column_span((original_row, col)) {
                col += span - 1;
            }
        } else if width > 0 {
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn print_spanned_columns<'a, F, I, D, C>(
    f: &mut F,
    buf: &mut BTreeMap<usize, (Cell<I::Item, &'a C::Color>, usize, usize)>,
    iter: I,
    cfg: &SpannedConfig,
    colors: &'a C,
    dimension: &D,
    this_height: usize,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    if this_height == 0 {
        // it's possible that we dont show row but it contains an actual cell which will be
        // rendered after all cause it's a rowspanned

        let mut skip = 0;
        for (col, cell) in iter.enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if let Some((_, _, colspan)) = buf.get(&col) {
                skip = *colspan - 1;
                continue;
            }

            let pos = (row, col);
            let rowspan = cfg.get_row_span(pos).unwrap_or(1);
            if rowspan < 2 {
                continue;
            }

            let height = if rowspan > 1 {
                range_height(cfg, dimension, row, row + rowspan, shape.0)
            } else {
                this_height
            };

            let colspan = cfg.get_column_span(pos).unwrap_or(1);
            skip = colspan - 1;
            let width = if colspan > 1 {
                range_width(cfg, dimension, col, col + colspan, shape.1)
            } else {
                dimension.get_width(col)
            };

            let color = colors.get_color(pos);
            let cell = Cell::new(cell, width, height, cfg, color, pos);

            buf.insert(col, (cell, rowspan, colspan));
        }

        buf.retain(|_, (_, rowspan, _)| {
            *rowspan -= 1;
            *rowspan != 0
        });

        return Ok(());
    }

    let mut skip = 0;
    for (col, cell) in iter.enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if let Some((_, _, colspan)) = buf.get(&col) {
            skip = *colspan - 1;
            continue;
        }

        let pos = (row, col);
        let colspan = cfg.get_column_span(pos).unwrap_or(1);
        skip = colspan - 1;

        let width = if colspan > 1 {
            range_width(cfg, dimension, col, col + colspan, shape.1)
        } else {
            dimension.get_width(col)
        };

        let rowspan = cfg.get_row_span(pos).unwrap_or(1);
        let height = if rowspan > 1 {
            range_height(cfg, dimension, row, row + rowspan, shape.0)
        } else {
            this_height
        };

        let color = colors.get_color(pos);
        let cell = Cell::new(cell, width, height, cfg, color, pos);

        buf.insert(col, (cell, rowspan, colspan));
    }

    for i in 0..this_height {
        let exact_line = line + i;
        let cell_line = i;

        print_margin_left(f, cfg, exact_line, totalh)?;

        for (&col, (cell, _, _)) in buf.iter_mut() {
            print_vertical_char(f, cfg, (row, col), cell_line, this_height, shape.1)?;
            cell.display(f)?;
        }

        print_vertical_char(f, cfg, (row, shape.1), cell_line, this_height, shape.1)?;

        print_margin_right(f, cfg, exact_line, totalh)?;

        if i + 1 != this_height {
            f.write_char('\n')?;
        }
    }

    buf.retain(|_, (_, rowspan, _)| {
        *rowspan -= 1;
        *rowspan != 0
    });

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

struct Cell<T, C> {
    lines: LinesIter<T>,
    width: usize,
    indent_top: usize,
    indent_left: Option<usize>,
    alignh: AlignmentHorizontal,
    fmt: Formatting,
    pad: Sides<Indent>,
    pad_color: Sides<Option<AnsiColor<'static>>>,
    color: Option<C>,
    justification: (char, Option<AnsiColor<'static>>),
}

impl<T, C> Cell<T, C>
where
    T: AsRef<str>,
{
    fn new(
        text: T,
        width: usize,
        height: usize,
        cfg: &SpannedConfig,
        color: Option<C>,
        pos: Position,
    ) -> Cell<T, C> {
        let fmt = *cfg.get_formatting(pos.into());
        let pad = cfg.get_padding(pos.into());
        let pad_color = cfg.get_padding_color(pos.into()).clone();
        let alignh = *cfg.get_alignment_horizontal(pos.into());
        let alignv = *cfg.get_alignment_vertical(pos.into());
        let justification = (
            cfg.get_justification(pos.into()),
            cfg.get_justification_color(pos.into()).cloned(),
        );

        let (count_lines, skip) = if fmt.vertical_trim {
            let (len, top, _) = count_empty_lines(text.as_ref());
            (len, top)
        } else {
            (count_lines(text.as_ref()), 0)
        };

        let indent_top = top_indent(&pad, alignv, count_lines, height);

        let mut indent_left = None;
        if !fmt.allow_lines_alignment {
            let text_width = get_text_width(text.as_ref(), fmt.horizontal_trim);
            let available = width - pad.left.size - pad.right.size;
            indent_left = Some(calculate_indent(alignh, text_width, available).0);
        }

        let mut lines = LinesIter::new(text);
        for _ in 0..skip {
            let _ = lines.lines.next();
        }

        Self {
            lines,
            indent_left,
            indent_top,
            width,
            alignh,
            fmt,
            pad,
            pad_color,
            color,
            justification,
        }
    }
}

impl<T, C> Cell<T, C>
where
    C: Color,
{
    fn display<F: Write>(&mut self, f: &mut F) -> fmt::Result {
        if self.indent_top > 0 {
            self.indent_top -= 1;
            print_padding_n(f, &self.pad.top, self.pad_color.top.as_ref(), self.width)?;
            return Ok(());
        }

        let line = match self.lines.lines.next() {
            Some(line) => line,
            None => {
                let color = self.pad_color.bottom.as_ref();
                print_padding_n(f, &self.pad.bottom, color, self.width)?;
                return Ok(());
            }
        };

        let line = if self.fmt.horizontal_trim && !line.is_empty() {
            string_trim(&line)
        } else {
            line
        };

        let line_width = string_width(&line);
        let available_width = self.width - self.pad.left.size - self.pad.right.size;

        let (left, right) = if self.fmt.allow_lines_alignment {
            calculate_indent(self.alignh, line_width, available_width)
        } else {
            let left = self.indent_left.expect("must be here");
            (left, available_width - line_width - left)
        };

        let (justification, justification_color) =
            (self.justification.0, self.justification.1.as_ref());

        print_padding(f, &self.pad.left, self.pad_color.left.as_ref())?;

        print_indent(f, justification, left, justification_color)?;
        print_text(f, &line, self.color.as_ref())?;
        print_indent(f, justification, right, justification_color)?;

        print_padding(f, &self.pad.right, self.pad_color.right.as_ref())?;

        Ok(())
    }
}

struct LinesIter<C> {
    _cell: C,
    /// SAFETY: IT'S NOT SAFE TO KEEP THE 'static REFERENCES AROUND AS THEY ARE NOT 'static in reality AND WILL BE DROPPED
    _text: &'static str,
    /// SAFETY: IT'S NOT SAFE TO KEEP THE 'static REFERENCES AROUND AS THEY ARE NOT 'static in reality AND WILL BE DROPPED
    lines: Lines<'static>,
}

impl<C> LinesIter<C> {
    fn new(cell: C) -> Self
    where
        C: AsRef<str>,
    {
        // We want to not allocate a String/Vec.
        // It's currently not possible due to a lifetime issues. (It's known as self-referential struct)
        //
        // Here we change the lifetime of text.
        //
        // # Safety
        //
        // It must be safe because the referenced string and the references are dropped at the same time.
        // And the referenced String is guaranteed to not be changed.
        let text = cell.as_ref();
        let text = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(text.as_ptr(), text.len()))
        };

        let lines = get_lines(text);

        Self {
            _cell: cell,
            _text: text,
            lines,
        }
    }
}

fn print_text<F: Write>(f: &mut F, text: &str, clr: Option<impl Color>) -> fmt::Result {
    match clr {
        Some(color) => {
            color.fmt_prefix(f)?;
            f.write_str(text)?;
            color.fmt_suffix(f)
        }
        None => f.write_str(text),
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

fn top_indent(
    padding: &Sides<Indent>,
    alignment: AlignmentVertical,
    cell_height: usize,
    available: usize,
) -> usize {
    let height = available - padding.top.size;
    let indent = indent_from_top(alignment, height, cell_height);

    indent + padding.top.size
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
    height: Option<usize>,
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
    height: Option<usize>,
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
    height: Option<usize>,
) -> fmt::Result {
    if indent.size == 0 {
        return Ok(());
    }

    match offset {
        Offset::Begin(mut offset) => {
            if let Some(max) = height {
                offset = cmp::min(offset, max);
            }

            if line >= offset {
                print_indent(f, indent.fill, indent.size, color)?;
            } else {
                repeat_char(f, ' ', indent.size)?;
            }
        }
        Offset::End(mut offset) => {
            if let Some(max) = height {
                offset = cmp::min(offset, max);
                let pos = max - offset;

                if line >= pos {
                    repeat_char(f, ' ', indent.size)?;
                } else {
                    print_indent(f, indent.fill, indent.size, color)?;
                }
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

fn print_padding<F: Write>(f: &mut F, pad: &Indent, color: Option<&AnsiColor<'_>>) -> fmt::Result {
    print_indent(f, pad.fill, pad.size, color)
}

fn print_padding_n<F: Write>(
    f: &mut F,
    pad: &Indent,
    color: Option<&AnsiColor<'_>>,
    n: usize,
) -> fmt::Result {
    print_indent(f, pad.fill, n, color)
}

fn print_indent<F: Write>(
    f: &mut F,
    c: char,
    n: usize,
    color: Option<&AnsiColor<'_>>,
) -> fmt::Result {
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

fn range_width(
    cfg: &SpannedConfig,
    d: impl Dimension,
    start: usize,
    end: usize,
    max: usize,
) -> usize {
    let count_borders = count_verticals_in_range(cfg, start, end, max);
    let range_width = (start..end).map(|col| d.get_width(col)).sum::<usize>();

    count_borders + range_width
}

fn range_height(
    cfg: &SpannedConfig,
    d: impl Dimension,
    from: usize,
    end: usize,
    max: usize,
) -> usize {
    let count_borders = count_horizontals_in_range(cfg, from, end, max);
    let range_width = (from..end).map(|col| d.get_height(col)).sum::<usize>();

    count_borders + range_width
}

fn count_horizontals_in_range(cfg: &SpannedConfig, from: usize, end: usize, max: usize) -> usize {
    (from + 1..end)
        .map(|i| cfg.has_horizontal(i, max) as usize)
        .sum()
}

fn count_verticals_in_range(cfg: &SpannedConfig, start: usize, end: usize, max: usize) -> usize {
    (start..end)
        .skip(1)
        .map(|i| cfg.has_vertical(i, max) as usize)
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

fn convert_count_rows(row: usize, is_last: bool) -> usize {
    if is_last {
        row + 1
    } else {
        row + 2
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

fn count_empty_lines(cell: &str) -> (usize, usize, usize) {
    let mut len = 0;
    let mut top = 0;
    let mut bottom = 0;
    let mut top_check = true;

    for line in get_lines(cell) {
        let is_empty = line.trim().is_empty();
        if top_check {
            if is_empty {
                top += 1;
            } else {
                len = 1;
                top_check = false;
            }

            continue;
        }

        if is_empty {
            bottom += 1;
        } else {
            len += bottom + 1;
            bottom = 0;
        }
    }

    (len, top, bottom)
}

fn get_text_width(text: &str, trim: bool) -> usize {
    if trim {
        get_lines(text)
            .map(|line| string_width(line.trim()))
            .max()
            .unwrap_or(0)
    } else {
        string_width_multiline(text)
    }
}

#[cfg(test)]
mod tests {
    // use crate::util::string_width;

    use super::*;

    // #[test]
    // fn horizontal_alignment_test() {
    //     use std::fmt;

    //     struct F<'a>(&'a str, AlignmentHorizontal, usize);

    //     impl fmt::Display for F<'_> {
    //         fn fmt(&self, f: &mut impl fmt::Write) -> fmt::Result {
    //             let (left, right) = calculate_indent(self.1, string_width(self.0), self.2);
    //             print_text_formatted(f, &self.0, 4, Option::<&AnsiColor<'_>>::None)
    //         }
    //     }

    //     assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
    //     assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
    //     assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
    //     assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
    //     assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");

    //     #[cfg(feature = "color")]
    //     {
    //         use owo_colors::OwoColorize;
    //         let text = "Colored Text".red().to_string();
    //         assert_eq!(
    //             F(&text, AlignmentHorizontal::Center, 15).to_string(),
    //             format!(" {}  ", text)
    //         );
    //     }
    // }

    #[test]
    fn vertical_alignment_test() {
        use AlignmentVertical::*;

        assert_eq!(indent_from_top(Bottom, 1, 1), 0);
        assert_eq!(indent_from_top(Top, 1, 1), 0);
        assert_eq!(indent_from_top(Center, 1, 1), 0);
        assert_eq!(indent_from_top(Bottom, 3, 1), 2);
        assert_eq!(indent_from_top(Top, 3, 1), 0);
        assert_eq!(indent_from_top(Center, 3, 1), 1);
        assert_eq!(indent_from_top(Center, 4, 1), 1);
    }

    #[test]
    fn count_empty_lines_test() {
        assert_eq!(count_empty_lines("\n\nsome text\n\n\n"), (1, 2, 3));
        assert_eq!(count_empty_lines("\n\nsome\ntext\n\n\n"), (2, 2, 3));
        assert_eq!(count_empty_lines("\n\nsome\nsome\ntext\n\n\n"), (3, 2, 3));
        assert_eq!(count_empty_lines("\n\n\n\n"), (0, 5, 0));
    }
}
