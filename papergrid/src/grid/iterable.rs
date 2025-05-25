//! The module contains a [`IterGrid`] structure.

use std::{
    borrow::{Borrow, Cow},
    cmp,
    collections::BTreeMap,
    fmt::{self, Write},
};

use crate::{
    ansi::{ANSIBuf, ANSIFmt},
    colors::Colors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Formatting, Indent, Offset,
        Position, Sides,
    },
    dimension::Dimension,
    records::{IntoRecords, Records},
    util::string::{count_lines, get_line_width, get_lines, get_text_width, Lines},
};

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IterGrid<R, D, G, C> {
    records: R,
    config: G,
    dimension: D,
    colors: C,
}

impl<R, D, G, C> IterGrid<R, D, G, C> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: G, dimension: D, colors: C) -> Self {
        IterGrid {
            records,
            config,
            dimension,
            colors,
        }
    }

    /// Builds a table.
    pub fn build<F>(self, mut f: F) -> fmt::Result
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
        D: Dimension,
        C: Colors,
        G: Borrow<SpannedConfig>,
        F: Write,
    {
        if self.records.count_columns() == 0 || self.records.hint_count_rows() == Some(0) {
            return Ok(());
        }

        let config = self.config.borrow();
        let ctx = GridCtx {
            cfg: config,
            colors: &self.colors,
            dims: &self.dimension,
        };

        print_grid(&mut f, self.records, &ctx)
    }

    /// Builds a table into string.
    ///
    /// Notice that it consumes self.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String
    where
        R: Records,
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
        D: Dimension,
        G: Borrow<SpannedConfig>,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranteed to never happen otherwise it's considered an stdlib error or impl error");
        buf
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridCtx<'a, D, C> {
    cfg: &'a SpannedConfig,
    colors: &'a C,
    dims: &'a D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowLine {
    row: usize,
    line: usize,
}

impl RowLine {
    fn new(row: usize, line: usize) -> Self {
        Self { row, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Height {
    value: usize,
    total: Option<usize>,
}

impl Height {
    fn new(value: usize, total: Option<usize>) -> Self {
        Self { value, total }
    }
}

fn print_grid<F, R, D, C>(f: &mut F, records: R, ctx: &GridCtx<'_, D, C>) -> fmt::Result
where
    F: Write,
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    // spanned version is a bit more complex and 'supposedly' slower,
    // because spans are considered to be not a general case we are having 2 versions
    let grid_has_spans = ctx.cfg.has_column_spans() || ctx.cfg.has_row_spans();
    if grid_has_spans {
        print_grid_spanned(f, records, ctx)
    } else {
        print_grid_general(f, records, ctx)
    }
}

fn print_grid_general<F, R, D, C>(f: &mut F, records: R, ctx: &GridCtx<'_, D, C>) -> fmt::Result
where
    F: Write,
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    let count_columns = records.count_columns();

    let mut totalw = None;
    let totalh = records
        .hint_count_rows()
        .map(|count_rows| total_height(ctx.cfg, ctx.dims, count_rows));

    let mut records_iter = records.iter_rows().into_iter();
    let mut next_columns = records_iter.next();

    if next_columns.is_none() {
        return Ok(());
    }

    if ctx.cfg.get_margin().top.size > 0 {
        totalw = Some(output_width(ctx.cfg, ctx.dims, count_columns));

        print_margin_top(f, ctx.cfg, totalw.unwrap())?;
        f.write_char('\n')?;
    }

    let mut row = 0;
    let mut line = 0;
    let mut is_prev_row_skipped = false;
    let mut buf = Vec::new();
    while let Some(columns) = next_columns {
        let columns = columns.into_iter();
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let height = ctx.dims.get_height(row);
        let count_rows = convert_count_rows(row, is_last_row);
        let has_horizontal = ctx.cfg.has_horizontal(row, count_rows);
        let shape = Position::new(count_rows, count_columns);
        let rline = RowLine::new(row, line);

        if row > 0 && !is_prev_row_skipped && (has_horizontal || height > 0) {
            f.write_char('\n')?;
        }

        if has_horizontal {
            print_horizontal_line(f, ctx, rline, shape, totalh)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        if height == 1 {
            print_single_line_columns(f, columns, ctx, rline, totalh, shape)?
        } else if height > 0 {
            buf.reserve(count_columns);

            collect_columns(&mut buf, columns, ctx, row, height);
            let height = Height::new(height, totalh);
            print_columns_lines(f, &mut buf, ctx.cfg, rline, height)?;

            buf.clear();
        }

        is_prev_row_skipped = height == 0 && !has_horizontal;
        line += height;
        row += 1;
    }

    if ctx.cfg.has_horizontal(row, row) {
        f.write_char('\n')?;
        let shape = Position::new(row, count_columns);
        let rline = RowLine::new(row, line);
        print_horizontal_line(f, ctx, rline, shape, totalh)?;
    }

    if ctx.cfg.get_margin().bottom.size > 0 {
        let totalw = totalw.unwrap_or_else(|| output_width(ctx.cfg, ctx.dims, count_columns));

        f.write_char('\n')?;
        print_margin_bottom(f, ctx.cfg, totalw)?;
    }

    Ok(())
}

fn output_width<D>(cfg: &SpannedConfig, d: D, count_columns: usize) -> usize
where
    D: Dimension,
{
    let margin = cfg.get_margin();
    total_width(cfg, &d, count_columns) + margin.left.size + margin.right.size
}

fn print_horizontal_line<F, D, C>(
    f: &mut F,
    ctx: &GridCtx<'_, D, C>,
    rline: RowLine,
    shape: Position,
    totalh: Option<usize>,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
{
    print_margin_left(f, ctx.cfg, rline.line, totalh)?;
    print_split_line(f, ctx.cfg, ctx.dims, rline.row, shape)?;
    print_margin_right(f, ctx.cfg, rline.line, totalh)?;
    Ok(())
}

fn print_single_line_columns<F, I, D, C>(
    f: &mut F,
    iter: I,
    ctx: &GridCtx<'_, D, C>,
    rline: RowLine,
    totalheight: Option<usize>,
    shape: Position,
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    print_margin_left(f, ctx.cfg, rline.line, totalheight)?;

    for (col, cell) in iter.enumerate() {
        let pos = Position::new(rline.row, col);
        let width = ctx.dims.get_width(col);
        let color = ctx.colors.get_color(pos);
        let text = cell.as_ref();
        print_vertical_char(f, ctx.cfg, pos, 0, 1, shape.col)?;
        print_single_line_column(f, text, ctx.cfg, width, color, pos)?;
    }

    let pos = Position::new(rline.row, shape.col);
    print_vertical_char(f, ctx.cfg, pos, 0, 1, shape.col)?;
    print_margin_right(f, ctx.cfg, rline.line, totalheight)?;

    Ok(())
}

fn print_single_line_column<F, C>(
    f: &mut F,
    text: &str,
    cfg: &SpannedConfig,
    width: usize,
    color: Option<&C>,
    pos: Position,
) -> fmt::Result
where
    F: Write,
    C: ANSIFmt,
{
    let pad = cfg.get_padding(pos);
    let pad_color = cfg.get_padding_color(pos);
    let fmt = cfg.get_formatting(pos);
    let space = cfg.get_justification(pos);
    let space_color = cfg.get_justification_color(pos);

    let (text, text_width) = if fmt.horizontal_trim && !text.is_empty() {
        let text = string_trim(text);
        let width = get_line_width(&text);

        (text, width)
    } else {
        let text = Cow::Borrowed(text);
        let width = get_text_width(&text);

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

fn print_columns_lines<T, F, C>(
    f: &mut F,
    buf: &mut [Cell<'_, T, C>],
    cfg: &SpannedConfig,
    rline: RowLine,
    height: Height,
) -> fmt::Result
where
    F: Write,
    C: ANSIFmt,
{
    let count_columns = buf.len();

    for i in 0..height.value {
        let exact_line = rline.line + i;

        print_margin_left(f, cfg, exact_line, height.total)?;

        for (col, cell) in buf.iter_mut().enumerate() {
            let pos = Position::new(rline.row, col);
            print_vertical_char(f, cfg, pos, i, height.value, count_columns)?;
            cell.display(f)?;
        }

        let pos = Position::new(rline.row, count_columns);
        print_vertical_char(f, cfg, pos, i, height.value, count_columns)?;

        print_margin_right(f, cfg, exact_line, height.total)?;

        if i + 1 != height.value {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn collect_columns<'a, I, D, C>(
    buf: &mut Vec<Cell<'a, I::Item, &'a C::Color>>,
    iter: I,
    ctx: &GridCtx<'a, D, C>,
    row: usize,
    height: usize,
) where
    I: Iterator,
    I::Item: AsRef<str>,
    C: Colors,
    D: Dimension,
{
    for (col, cell) in iter.enumerate() {
        let pos = Position::new(row, col);
        let width = ctx.dims.get_width(col);
        let color = ctx.colors.get_color(pos);
        let cell = Cell::new(cell, width, height, ctx.cfg, color, pos);
        buf.push(cell);
    }
}

fn print_split_line<F, D>(
    f: &mut F,
    cfg: &SpannedConfig,
    dimension: &D,
    row: usize,
    shape: Position,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
{
    let mut used_color = None;
    print_vertical_intersection(f, cfg, (row, 0).into(), shape, &mut used_color)?;

    for col in 0..shape.col {
        let width = dimension.get_width(col);

        // general case
        if width > 0 {
            let pos = (row, col).into();
            let main = cfg.get_horizontal(pos, shape.row);
            match main {
                Some(c) => {
                    let clr = cfg.get_horizontal_color(pos, shape.row);
                    prepare_coloring(f, clr, &mut used_color)?;
                    print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                }
                None => repeat_char(f, ' ', width)?,
            }
        }

        print_vertical_intersection(f, cfg, (row, col + 1).into(), shape, &mut used_color)?;
    }

    if let Some(clr) = used_color.take() {
        clr.fmt_ansi_suffix(f)?;
    }

    Ok(())
}

fn print_grid_spanned<F, R, D, C>(f: &mut F, records: R, ctx: &GridCtx<'_, D, C>) -> fmt::Result
where
    F: Write,
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    let count_columns = records.count_columns();

    let total_width = total_width(ctx.cfg, ctx.dims, count_columns);
    let margin = ctx.cfg.get_margin();
    let total_width_with_margin = total_width + margin.left.size + margin.right.size;

    let totalh = records
        .hint_count_rows()
        .map(|rows| total_height(ctx.cfg, ctx.dims, rows));

    if margin.top.size > 0 {
        print_margin_top(f, ctx.cfg, total_width_with_margin)?;
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

        let height = ctx.dims.get_height(row);
        let count_rows = convert_count_rows(row, is_last_row);
        let shape = Position::new(count_rows, count_columns);

        let has_horizontal = ctx.cfg.has_horizontal(row, count_rows);
        if need_new_line && (has_horizontal || height > 0) {
            f.write_char('\n')?;
            need_new_line = false;
        }

        if has_horizontal {
            print_margin_left(f, ctx.cfg, line, totalh)?;
            print_split_line_spanned(f, &mut buf, ctx.cfg, ctx.dims, row, shape)?;
            print_margin_right(f, ctx.cfg, line, totalh)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        let rline = RowLine::new(row, line);
        let height = Height::new(height, totalh);
        print_spanned_columns(f, columns, &mut buf, ctx, rline, height, shape)?;

        if has_horizontal || height.value > 0 {
            need_new_line = true;
        }

        line += height.value;
        row += 1;
    }

    if row > 0 {
        if ctx.cfg.has_horizontal(row, row) {
            f.write_char('\n')?;
            let shape = Position::new(row, count_columns);
            let rline = RowLine::new(row, line);
            print_horizontal_line(f, ctx, rline, shape, totalh)?;
        }

        if margin.bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, ctx.cfg, total_width_with_margin)?;
        }
    }

    Ok(())
}

fn print_split_line_spanned<S, F, D, C>(
    f: &mut F,
    buf: &mut BTreeMap<usize, Cell<'_, S, C>>,
    cfg: &SpannedConfig,
    dimension: &D,
    row: usize,
    shape: Position,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
    C: ANSIFmt,
{
    let mut used_color = None;
    print_vertical_intersection(f, cfg, (row, 0).into(), shape, &mut used_color)?;

    for col in 0..shape.col {
        let pos = (row, col).into();
        if cfg.is_cell_covered_by_both_spans(pos) {
            continue;
        }

        let width = dimension.get_width(col);
        let mut col = col;
        if cfg.is_cell_covered_by_row_span(pos) {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            prepare_coloring(f, None, &mut used_color)?;
            let cell = buf.get_mut(&col).unwrap();
            cell.display(f)?;

            // We need to use a correct right split char.
            let original_row = closest_visible_row(cfg, pos).unwrap();
            if let Some(span) = cfg.get_column_span((original_row, col).into()) {
                col += span - 1;
            }
        } else if width > 0 {
            // general case
            let main = cfg.get_horizontal(pos, shape.row);
            match main {
                Some(c) => {
                    let clr = cfg.get_horizontal_color(pos, shape.row);
                    prepare_coloring(f, clr, &mut used_color)?;
                    print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                }
                None => repeat_char(f, ' ', width)?,
            }
        }

        print_vertical_intersection(f, cfg, (row, col + 1).into(), shape, &mut used_color)?;
    }

    if let Some(clr) = used_color.take() {
        clr.fmt_ansi_suffix(f)?;
    }

    Ok(())
}

fn print_vertical_intersection<'a, F>(
    f: &mut F,
    cfg: &'a SpannedConfig,
    pos: Position,
    shape: Position,
    used_color: &mut Option<&'a ANSIBuf>,
) -> fmt::Result
where
    F: fmt::Write,
{
    if !cfg.has_vertical(pos.col, shape.col) {
        return Ok(());
    }

    match cfg.get_intersection(pos, shape.into()) {
        Some(c) => {
            let clr = cfg.get_intersection_color(pos, shape.into());
            prepare_coloring(f, clr, used_color)?;
            f.write_char(c)
        }
        None => Ok(()),
    }
}

fn print_spanned_columns<'a, F, I, D, C>(
    f: &mut F,
    iter: I,
    buf: &mut BTreeMap<usize, Cell<'a, I::Item, &'a C::Color>>,
    ctx: &GridCtx<'a, D, C>,
    rline: RowLine,
    height: Height,
    shape: Position,
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    if height.value == 0 {
        // it's possible that we dont show row but it contains an actual cell which will be
        // rendered after all cause it's a rowspanned

        let mut skip = 0;
        for (col, cell) in iter.enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if let Some(cell) = buf.get(&col) {
                skip = cell.colspan - 1;
                continue;
            }

            let pos = Position::new(rline.row, col);
            let rowspan = ctx.cfg.get_row_span(pos).unwrap_or(1);
            if rowspan < 2 {
                continue;
            }

            // FIXME: Do we need to recalcalate it?
            // Is height not enough?
            let height = if rowspan > 1 {
                range_height(ctx.cfg, ctx.dims, rline.row, rline.row + rowspan, shape.row)
            } else {
                height.value
            };

            let colspan = ctx.cfg.get_column_span(pos).unwrap_or(1);
            skip = colspan - 1;
            let width = if colspan > 1 {
                range_width(ctx.cfg, ctx.dims, col, col + colspan, shape.col)
            } else {
                ctx.dims.get_width(col)
            };

            let color = ctx.colors.get_color(pos);
            let mut cell = Cell::new(cell, width, height, ctx.cfg, color, pos);
            cell.rowspan = rowspan;
            cell.colspan = colspan;

            buf.insert(col, cell);
        }

        buf.retain(|_, cell| {
            cell.rowspan -= 1;
            cell.rowspan != 0
        });

        return Ok(());
    }

    let mut skip = 0;
    for (col, cell) in iter.enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if let Some(cell) = buf.get(&col) {
            skip = cell.colspan - 1;
            continue;
        }

        let pos = Position::new(rline.row, col);
        let colspan = ctx.cfg.get_column_span(pos).unwrap_or(1);
        skip = colspan - 1;

        let width = if colspan > 1 {
            range_width(ctx.cfg, ctx.dims, col, col + colspan, shape.col)
        } else {
            ctx.dims.get_width(col)
        };

        let rowspan = ctx.cfg.get_row_span(pos).unwrap_or(1);
        let height = if rowspan > 1 {
            range_height(ctx.cfg, ctx.dims, rline.row, rline.row + rowspan, shape.row)
        } else {
            height.value
        };

        let color = ctx.colors.get_color(pos);
        let mut cell = Cell::new(cell, width, height, ctx.cfg, color, pos);
        cell.rowspan = rowspan;
        cell.colspan = colspan;

        buf.insert(col, cell);
    }

    for i in 0..height.value {
        let exact_line = rline.line + i;
        let cell_line = i;

        print_margin_left(f, ctx.cfg, exact_line, height.total)?;

        for (&col, cell) in buf.iter_mut() {
            let pos = Position::new(rline.row, col);
            print_vertical_char(f, ctx.cfg, pos, cell_line, height.value, shape.col)?;
            cell.display(f)?;
        }

        let pos = Position::new(rline.row, shape.col);
        print_vertical_char(f, ctx.cfg, pos, cell_line, height.value, shape.col)?;

        print_margin_right(f, ctx.cfg, exact_line, height.total)?;

        if i + 1 != height.value {
            f.write_char('\n')?;
        }
    }

    buf.retain(|_, cell| {
        cell.rowspan -= 1;
        cell.rowspan != 0
    });

    Ok(())
}

fn print_horizontal_border<F>(
    f: &mut F,
    cfg: &SpannedConfig,
    pos: Position,
    width: usize,
    c: char,
    used_color: &Option<&ANSIBuf>,
) -> fmt::Result
where
    F: Write,
{
    if !cfg.is_overridden_horizontal(pos) {
        return repeat_char(f, c, width);
    }

    for i in 0..width {
        let c = cfg.lookup_horizontal_char(pos, i, width).unwrap_or(c);
        match cfg.lookup_horizontal_color(pos, i, width) {
            Some(color) => match used_color {
                Some(clr) => {
                    clr.fmt_ansi_suffix(f)?;
                    color.fmt_ansi_prefix(f)?;
                    f.write_char(c)?;
                    color.fmt_ansi_suffix(f)?;
                    clr.fmt_ansi_prefix(f)?;
                }
                None => {
                    color.fmt_ansi_prefix(f)?;
                    f.write_char(c)?;
                    color.fmt_ansi_suffix(f)?;
                }
            },
            _ => f.write_char(c)?,
        }
    }

    Ok(())
}

struct Cell<'a, T, C> {
    lines: LinesIter<T>,
    width: usize,
    indent_top: usize,
    indent_left: Option<usize>,
    alignh: AlignmentHorizontal,
    fmt: Formatting,
    pad: &'a Sides<Indent>,
    pad_color: &'a Sides<Option<ANSIBuf>>,
    color: Option<C>,
    justification: char,
    justification_color: Option<&'a ANSIBuf>,
    rowspan: usize,
    colspan: usize,
}

impl<'a, T, C> Cell<'a, T, C> {
    fn new(
        text: T,
        width: usize,
        height: usize,
        cfg: &'a SpannedConfig,
        color: Option<C>,
        pos: Position,
    ) -> Cell<'a, T, C>
    where
        T: AsRef<str>,
    {
        let fmt = cfg.get_formatting(pos);
        let pad = cfg.get_padding(pos);
        let pad_color = cfg.get_padding_color(pos);
        let alignh = *cfg.get_alignment_horizontal(pos);
        let alignv = *cfg.get_alignment_vertical(pos);
        let justification = cfg.get_justification(pos);
        let justification_color = cfg.get_justification_color(pos);

        let (count_lines, skip) = if fmt.vertical_trim {
            let (len, top, _) = count_empty_lines(text.as_ref());
            (len, top)
        } else {
            (count_lines(text.as_ref()), 0)
        };

        let indent_top = top_indent(pad, alignv, count_lines, height);

        let mut indent_left = None;
        if !fmt.allow_lines_alignment {
            let text_width = text_width(text.as_ref(), fmt.horizontal_trim);
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
            justification_color,
            colspan: 0,
            rowspan: 0,
        }
    }

    fn display<F>(&mut self, f: &mut F) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
    {
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

        let line_width = get_line_width(&line);
        let available_width = self.width - self.pad.left.size - self.pad.right.size;

        let (left, right) = if self.fmt.allow_lines_alignment {
            calculate_indent(self.alignh, line_width, available_width)
        } else {
            let left = self.indent_left.expect("must be here");
            (left, available_width - line_width - left)
        };

        print_padding(f, &self.pad.left, self.pad_color.left.as_ref())?;

        print_indent(f, self.justification, left, self.justification_color)?;
        print_text(f, &line, self.color.as_ref())?;
        print_indent(f, self.justification, right, self.justification_color)?;

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

fn print_text<F>(f: &mut F, text: &str, clr: Option<impl ANSIFmt>) -> fmt::Result
where
    F: Write,
{
    match clr {
        Some(color) => {
            color.fmt_ansi_prefix(f)?;
            f.write_str(text)?;
            color.fmt_ansi_suffix(f)
        }
        None => f.write_str(text),
    }
}

fn prepare_coloring<'a, F>(
    f: &mut F,
    clr: Option<&'a ANSIBuf>,
    used_color: &mut Option<&'a ANSIBuf>,
) -> fmt::Result
where
    F: Write,
{
    match clr {
        Some(clr) => match used_color.as_mut() {
            Some(used_clr) => {
                if **used_clr != *clr {
                    used_clr.fmt_ansi_suffix(f)?;
                    clr.fmt_ansi_prefix(f)?;
                    *used_clr = clr;
                }
            }
            None => {
                clr.fmt_ansi_prefix(f)?;
                *used_color = Some(clr);
            }
        },
        None => {
            if let Some(clr) = used_color.take() {
                clr.fmt_ansi_suffix(f)?
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

fn calculate_indent(alignment: AlignmentHorizontal, got: usize, max: usize) -> (usize, usize) {
    let diff = max - got;
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

fn repeat_char<F>(f: &mut F, c: char, n: usize) -> fmt::Result
where
    F: Write,
{
    for _ in 0..n {
        f.write_char(c)?;
    }

    Ok(())
}

fn print_vertical_char<F>(
    f: &mut F,
    cfg: &SpannedConfig,
    pos: Position,
    line: usize,
    count_lines: usize,
    count_columns: usize,
) -> fmt::Result
where
    F: Write,
{
    let symbol = match cfg.get_vertical(pos, count_columns) {
        Some(c) => c,
        None => return Ok(()),
    };

    let symbol = cfg
        .lookup_vertical_char(pos, line, count_lines)
        .unwrap_or(symbol);

    let color = cfg
        .get_vertical_color(pos, count_columns)
        .or_else(|| cfg.lookup_vertical_color(pos, line, count_lines));

    match color {
        Some(clr) => {
            clr.fmt_ansi_prefix(f)?;
            f.write_char(symbol)?;
            clr.fmt_ansi_suffix(f)?;
        }
        None => f.write_char(symbol)?,
    }

    Ok(())
}

fn print_margin_top<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
where
    F: Write,
{
    let indent = cfg.get_margin().top;
    let offset = cfg.get_margin_offset().top;
    let color = cfg.get_margin_color();
    let color = color.top;
    print_indent_lines(f, &indent, &offset, color, width)
}

fn print_margin_bottom<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
where
    F: Write,
{
    let indent = cfg.get_margin().bottom;
    let offset = cfg.get_margin_offset().bottom;
    let color = cfg.get_margin_color();
    let color = color.bottom;
    print_indent_lines(f, &indent, &offset, color, width)
}

fn print_margin_left<F>(
    f: &mut F,
    cfg: &SpannedConfig,
    line: usize,
    height: Option<usize>,
) -> fmt::Result
where
    F: Write,
{
    let indent = cfg.get_margin().left;
    let offset = cfg.get_margin_offset().left;
    let color = cfg.get_margin_color();
    let color = color.left;
    print_margin_vertical(f, indent, offset, color, line, height)
}

fn print_margin_right<F>(
    f: &mut F,
    cfg: &SpannedConfig,
    line: usize,
    height: Option<usize>,
) -> fmt::Result
where
    F: Write,
{
    let indent = cfg.get_margin().right;
    let offset = cfg.get_margin_offset().right;
    let color = cfg.get_margin_color();
    let color = color.right;
    print_margin_vertical(f, indent, offset, color, line, height)
}

fn print_margin_vertical<F>(
    f: &mut F,
    indent: Indent,
    offset: Offset,
    color: Option<&ANSIBuf>,
    line: usize,
    height: Option<usize>,
) -> fmt::Result
where
    F: Write,
{
    if indent.size == 0 {
        return Ok(());
    }

    match offset {
        Offset::Start(mut offset) => {
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

fn print_indent_lines<F>(
    f: &mut F,
    indent: &Indent,
    offset: &Offset,
    color: Option<&ANSIBuf>,
    width: usize,
) -> fmt::Result
where
    F: Write,
{
    if indent.size == 0 {
        return Ok(());
    }

    let (start_offset, end_offset) = match offset {
        Offset::Start(start) => (*start, 0),
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

fn print_padding<F>(f: &mut F, pad: &Indent, color: Option<&ANSIBuf>) -> fmt::Result
where
    F: Write,
{
    print_indent(f, pad.fill, pad.size, color)
}

fn print_padding_n<F>(f: &mut F, pad: &Indent, color: Option<&ANSIBuf>, n: usize) -> fmt::Result
where
    F: Write,
{
    print_indent(f, pad.fill, n, color)
}

fn print_indent<F>(f: &mut F, c: char, n: usize, color: Option<&ANSIBuf>) -> fmt::Result
where
    F: Write,
{
    if n == 0 {
        return Ok(());
    }

    match color {
        Some(color) => {
            color.fmt_ansi_prefix(f)?;
            repeat_char(f, c, n)?;
            color.fmt_ansi_suffix(f)
        }
        None => repeat_char(f, c, n),
    }
}

fn range_width<D>(cfg: &SpannedConfig, dims: D, start: usize, end: usize, max: usize) -> usize
where
    D: Dimension,
{
    let count_borders = count_verticals_in_range(cfg, start, end, max);
    let range_width = (start..end).map(|col| dims.get_width(col)).sum::<usize>();

    count_borders + range_width
}

fn range_height<D>(cfg: &SpannedConfig, dims: D, from: usize, end: usize, max: usize) -> usize
where
    D: Dimension,
{
    let count_borders = count_horizontals_in_range(cfg, from, end, max);
    let range_width = (from..end).map(|col| dims.get_height(col)).sum::<usize>();

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
            return Some(pos.row);
        }

        if pos.row == 0 {
            return None;
        }

        pos -= (1, 0);
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
    #[cfg(feature = "ansi")]
    {
        ansi_str::AnsiStr::ansi_trim(text)
    }

    #[cfg(not(feature = "ansi"))]
    {
        text.trim().into()
    }
}

fn total_width<D>(cfg: &SpannedConfig, dimension: &D, count_columns: usize) -> usize
where
    D: Dimension,
{
    (0..count_columns)
        .map(|i| dimension.get_width(i))
        .sum::<usize>()
        + cfg.count_vertical(count_columns)
}

fn total_height<D>(cfg: &SpannedConfig, dimension: &D, count_rows: usize) -> usize
where
    D: Dimension,
{
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

fn text_width(text: &str, trim: bool) -> usize {
    if trim {
        get_lines(text)
            .map(|line| get_line_width(line.trim()))
            .max()
            .unwrap_or(0)
    } else {
        get_text_width(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
