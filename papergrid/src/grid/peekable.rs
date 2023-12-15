//! The module contains a [`PeekableGrid`] structure.

use core::borrow::Borrow;
use std::{
    borrow::Cow,
    cmp,
    fmt::{self, Write},
};

use crate::{
    ansi::{ANSIBuf, ANSIFmt},
    colors::Colors,
    config::{
        spanned::{Offset, SpannedConfig},
        Formatting,
    },
    config::{AlignmentHorizontal, AlignmentVertical, Entity, Indent, Position, Sides},
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

        let ctx = PrintCtx {
            cfg: self.config.borrow(),
            colors: &self.colors,
            dims: &self.dimension,
            records: &self.records,
        };

        print_grid(&mut f, ctx)
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

#[derive(Debug, Copy, Clone)]
struct PrintCtx<'a, R, D, C> {
    records: &'a R,
    cfg: &'a SpannedConfig,
    dims: &'a D,
    colors: &'a C,
}

fn print_grid<F, R, D, C>(f: &mut F, ctx: PrintCtx<'_, R, D, C>) -> fmt::Result
where
    F: Write,
    R: Records + PeekableRecords + ExactRecords,
    D: Dimension,
    C: Colors,
{
    let has_spans = ctx.cfg.has_column_spans() || ctx.cfg.has_row_spans();
    if has_spans {
        return grid_spanned::build_grid(f, ctx);
    }

    let is_basic = !ctx.cfg.has_border_colors()
        && !ctx.cfg.has_justification()
        && ctx.cfg.get_justification_color(Entity::Global).is_none()
        && !ctx.cfg.has_offset_chars()
        && !has_margin(ctx.cfg)
        && !has_padding_color(ctx.cfg)
        && ctx.colors.is_empty();

    if is_basic {
        grid_basic::build_grid(f, ctx)
    } else {
        grid_not_spanned::build_grid(f, ctx)
    }
}

fn has_margin(cfg: &SpannedConfig) -> bool {
    let margin = cfg.get_margin();
    margin.left.size > 0 || margin.right.size > 0 || margin.top.size > 0 || margin.bottom.size > 0
}

fn has_padding_color(cfg: &SpannedConfig) -> bool {
    let pad = cfg.get_padding_color(Entity::Global);
    let has_pad =
        pad.left.is_some() || pad.right.is_some() || pad.top.is_some() || pad.bottom.is_some();

    has_pad || cfg.has_padding_color()
}

mod grid_basic {
    use super::*;

    struct TextCfg {
        alignment: AlignmentHorizontal,
        formatting: Formatting,
        justification: char,
    }

    #[derive(Debug, Clone, Copy)]
    struct Shape {
        count_rows: usize,
        count_columns: usize,
    }

    struct HIndent {
        left: usize,
        right: usize,
    }

    pub(super) fn build_grid<F, R, D, C>(f: &mut F, ctx: PrintCtx<'_, R, D, C>) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
    {
        let shape = Shape {
            count_rows: ctx.records.count_rows(),
            count_columns: ctx.records.count_columns(),
        };

        let mut new_line = false;

        for row in 0..shape.count_rows {
            let height = ctx.dims.get_height(row);

            let has_horizontal = ctx.cfg.has_horizontal(row, shape.count_rows);

            if new_line && (has_horizontal || height > 0) {
                f.write_char('\n')?;
                new_line = false;
            }

            if has_horizontal {
                print_split_line(f, ctx.cfg, ctx.dims, row, shape)?;

                if height > 0 {
                    f.write_char('\n')?;
                } else {
                    new_line = true;
                }
            }

            if height > 0 {
                print_grid_line(f, &ctx, shape, height, row, 0)?;

                for i in 1..height {
                    f.write_char('\n')?;

                    print_grid_line(f, &ctx, shape, height, row, i)?;
                }

                new_line = true;
            }
        }

        if ctx.cfg.has_horizontal(shape.count_rows, shape.count_rows) {
            f.write_char('\n')?;
            print_split_line(f, ctx.cfg, ctx.dims, shape.count_rows, shape)?;
        }

        Ok(())
    }

    fn print_grid_line<F, R, D, C>(
        f: &mut F,
        ctx: &PrintCtx<'_, R, D, C>,
        shape: Shape,
        height: usize,
        row: usize,
        line: usize,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
    {
        for col in 0..shape.count_columns {
            let pos = (row, col);
            print_vertical_char(f, ctx.cfg, pos, shape.count_columns)?;
            print_cell_line(f, ctx, height, pos, line)?;
        }

        let pos = (row, shape.count_columns);
        print_vertical_char(f, ctx.cfg, pos, shape.count_columns)?;

        Ok(())
    }

    fn print_split_line<F, D>(
        f: &mut F,
        cfg: &SpannedConfig,
        dimension: &D,
        row: usize,
        shape: Shape,
    ) -> fmt::Result
    where
        F: Write,
        D: Dimension,
    {
        print_vertical_intersection(f, cfg, (row, 0), shape)?;

        for col in 0..shape.count_columns {
            let width = dimension.get_width(col);

            // general case
            if width > 0 {
                let pos = (row, col);
                let main = cfg.get_horizontal(pos, shape.count_rows);
                match main {
                    Some(c) => repeat_char(f, c, width)?,
                    None => repeat_char(f, ' ', width)?,
                }
            }

            let pos = (row, col + 1);
            print_vertical_intersection(f, cfg, pos, shape)?;
        }

        Ok(())
    }

    fn print_vertical_intersection<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        pos: Position,
        shape: Shape,
    ) -> fmt::Result
    where
        F: fmt::Write,
    {
        let intersection = match cfg.get_intersection(pos, (shape.count_rows, shape.count_columns))
        {
            Some(c) => c,
            None => return Ok(()),
        };

        // We need to make sure that we need to print it.
        // Specifically for cases where we have a limited amount of verticals.
        //
        // todo: Yes... this check very likely degrages performance a bit,
        //       Surely we need to rethink it.
        if !cfg.has_vertical(pos.1, shape.count_columns) {
            return Ok(());
        }

        f.write_char(intersection)?;

        Ok(())
    }

    fn print_vertical_char<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        pos: Position,
        count_columns: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
        let symbol = match cfg.get_vertical(pos, count_columns) {
            Some(c) => c,
            None => return Ok(()),
        };

        f.write_char(symbol)?;

        Ok(())
    }

    fn print_cell_line<F, R, D, C>(
        f: &mut F,
        ctx: &PrintCtx<'_, R, D, C>,
        height: usize,
        pos: Position,
        line: usize,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
    {
        let entity = Entity::from(pos);

        let width = ctx.dims.get_width(pos.1);

        let pad = ctx.cfg.get_padding(entity);
        let valignment = *ctx.cfg.get_alignment_vertical(entity);
        let text_cfg = TextCfg {
            alignment: *ctx.cfg.get_alignment_horizontal(entity),
            formatting: *ctx.cfg.get_formatting(entity),
            justification: ctx.cfg.get_justification(Entity::Global),
        };

        let mut cell_height = ctx.records.count_lines(pos);
        if text_cfg.formatting.vertical_trim {
            cell_height -= count_empty_lines_at_start(ctx.records, pos)
                + count_empty_lines_at_end(ctx.records, pos);
        }

        if cell_height > height {
            // it may happen if the height estimation decide so
            cell_height = height;
        }

        let indent = top_indent(&pad, valignment, cell_height, height);
        if indent > line {
            return repeat_char(f, pad.top.fill, width);
        }

        let mut index = line - indent;
        let cell_has_this_line = cell_height > index;
        if !cell_has_this_line {
            // happens when other cells have bigger height
            return repeat_char(f, pad.bottom.fill, width);
        }

        if text_cfg.formatting.vertical_trim {
            let empty_lines = count_empty_lines_at_start(ctx.records, pos);
            index += empty_lines;

            if index > ctx.records.count_lines(pos) {
                return repeat_char(f, pad.top.fill, width);
            }
        }

        let width = width - pad.left.size - pad.right.size;

        repeat_char(f, pad.left.fill, pad.left.size)?;
        print_line(f, ctx.records, pos, index, width, text_cfg)?;
        repeat_char(f, pad.right.fill, pad.right.size)?;

        Ok(())
    }

    fn print_line<F, R>(
        f: &mut F,
        records: &R,
        pos: Position,
        index: usize,
        available: usize,
        cfg: TextCfg,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords,
    {
        let line = records.get_line(pos, index);
        let (line, line_width) = if cfg.formatting.horizontal_trim {
            let line = string_trim(line);
            let width = string_width(&line);
            (line, width)
        } else {
            let width = records.get_line_width(pos, index);
            (Cow::Borrowed(line), width)
        };

        if cfg.formatting.allow_lines_alignment {
            let indent = calculate_indent(cfg.alignment, line_width, available);
            return print_text_padded(f, &line, cfg.justification, indent);
        }

        let cell_width = if cfg.formatting.horizontal_trim {
            (0..records.count_lines(pos))
                .map(|i| records.get_line(pos, i))
                .map(|line| string_width(line.trim()))
                .max()
                .unwrap_or_default()
        } else {
            records.get_width(pos)
        };

        let indent = calculate_indent(cfg.alignment, cell_width, available);
        print_text_padded(f, &line, cfg.justification, indent)?;

        // todo: remove me?
        let rest_width = cell_width - line_width;
        repeat_char(f, ' ', rest_width)?;

        Ok(())
    }

    fn print_text_padded<F>(f: &mut F, text: &str, space: char, indent: HIndent) -> fmt::Result
    where
        F: Write,
    {
        repeat_char(f, space, indent.left)?;
        f.write_str(text)?;
        repeat_char(f, space, indent.right)?;

        Ok(())
    }

    fn top_indent(
        pad: &Sides<Indent>,
        alignment: AlignmentVertical,
        height: usize,
        available: usize,
    ) -> usize {
        let available = available - pad.top.size;
        let indent = indent_from_top(alignment, available, height);

        indent + pad.top.size
    }

    fn indent_from_top(alignment: AlignmentVertical, available: usize, real: usize) -> usize {
        match alignment {
            AlignmentVertical::Top => 0,
            AlignmentVertical::Bottom => available - real,
            AlignmentVertical::Center => (available - real) / 2,
        }
    }

    fn calculate_indent(alignment: AlignmentHorizontal, width: usize, available: usize) -> HIndent {
        let diff = available - width;

        let (left, right) = match alignment {
            AlignmentHorizontal::Left => (0, diff),
            AlignmentHorizontal::Right => (diff, 0),
            AlignmentHorizontal::Center => {
                let left = diff / 2;
                let rest = diff - left;
                (left, rest)
            }
        };

        HIndent { left, right }
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
}

mod grid_not_spanned {
    use super::*;

    struct TextCfg<C, C1> {
        alignment: AlignmentHorizontal,
        formatting: Formatting,
        color: Option<C>,
        justification: Colored<char, C1>,
    }

    struct Colored<T, C> {
        data: T,
        color: Option<C>,
    }

    impl<T, C> Colored<T, C> {
        fn new(data: T, color: Option<C>) -> Self {
            Self { data, color }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Shape {
        count_rows: usize,
        count_columns: usize,
    }

    struct HIndent {
        left: usize,
        right: usize,
    }

    pub(super) fn build_grid<F, R, D, C>(f: &mut F, ctx: PrintCtx<'_, R, D, C>) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
        C: Colors,
    {
        let shape = Shape {
            count_rows: ctx.records.count_rows(),
            count_columns: ctx.records.count_columns(),
        };

        let total_width = total_width(ctx.cfg, ctx.dims, shape.count_columns);

        let margin = ctx.cfg.get_margin();
        let total_width_with_margin = total_width + margin.left.size + margin.right.size;

        let total_height = total_height(ctx.cfg, ctx.dims, shape.count_rows);

        if margin.top.size > 0 {
            print_margin_top(f, ctx.cfg, total_width_with_margin)?;
            f.write_char('\n')?;
        }

        let mut table_line = 0;
        let mut prev_empty_horizontal = false;
        for row in 0..shape.count_rows {
            let height = ctx.dims.get_height(row);

            if ctx.cfg.has_horizontal(row, shape.count_rows) {
                if prev_empty_horizontal {
                    f.write_char('\n')?;
                }

                print_margin_left(f, ctx.cfg, table_line, total_height)?;
                print_split_line(f, ctx.cfg, ctx.dims, row, shape)?;
                print_margin_right(f, ctx.cfg, table_line, total_height)?;

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
                print_margin_left(f, ctx.cfg, table_line, total_height)?;

                for col in 0..shape.count_columns {
                    print_vertical_char(f, ctx.cfg, (row, col), i, height, shape.count_columns)?;
                    print_cell_line(f, &ctx, height, (row, col), i)?;

                    let is_last_column = col + 1 == shape.count_columns;
                    if is_last_column {
                        let pos = (row, col + 1);
                        print_vertical_char(f, ctx.cfg, pos, i, height, shape.count_columns)?;
                    }
                }

                print_margin_right(f, ctx.cfg, table_line, total_height)?;

                let is_last_line = i + 1 == height;
                let is_last_row = row + 1 == shape.count_rows;
                if !(is_last_line && is_last_row) {
                    f.write_char('\n')?;
                }

                table_line += 1;
            }
        }

        if ctx.cfg.has_horizontal(shape.count_rows, shape.count_rows) {
            f.write_char('\n')?;
            print_margin_left(f, ctx.cfg, table_line, total_height)?;
            print_split_line(f, ctx.cfg, ctx.dims, shape.count_rows, shape)?;
            print_margin_right(f, ctx.cfg, table_line, total_height)?;
        }

        if margin.bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, ctx.cfg, total_width_with_margin)?;
        }

        Ok(())
    }

    fn print_split_line<F, D>(
        f: &mut F,
        cfg: &SpannedConfig,
        dimension: &D,
        row: usize,
        shape: Shape,
    ) -> fmt::Result
    where
        F: Write,
        D: Dimension,
    {
        let mut used_color = None;
        print_vertical_intersection(f, cfg, (row, 0), shape, &mut used_color)?;

        for col in 0..shape.count_columns {
            let width = dimension.get_width(col);

            // general case
            if width > 0 {
                let pos = (row, col);
                let main = cfg.get_horizontal(pos, shape.count_rows);
                match main {
                    Some(c) => {
                        let clr = cfg.get_horizontal_color(pos, shape.count_rows);
                        prepare_coloring(f, clr, &mut used_color)?;
                        print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                    }
                    None => repeat_char(f, ' ', width)?,
                }
            }

            let pos = (row, col + 1);
            print_vertical_intersection(f, cfg, pos, shape, &mut used_color)?;
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
        shape: Shape,
        used_color: &mut Option<&'a ANSIBuf>,
    ) -> fmt::Result
    where
        F: fmt::Write,
    {
        let intersection = match cfg.get_intersection(pos, (shape.count_rows, shape.count_columns))
        {
            Some(c) => c,
            None => return Ok(()),
        };

        // We need to make sure that we need to print it.
        // Specifically for cases where we have a limited amount of verticals.
        //
        // todo: Yes... this check very likely degrages performance a bit,
        //       Surely we need to rethink it.
        if !cfg.has_vertical(pos.1, shape.count_columns) {
            return Ok(());
        }

        let color = cfg.get_intersection_color(pos, (shape.count_rows, shape.count_columns));
        prepare_coloring(f, color, used_color)?;
        f.write_char(intersection)?;

        Ok(())
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

    fn print_cell_line<F, R, D, C>(
        f: &mut F,
        ctx: &PrintCtx<'_, R, D, C>,
        height: usize,
        pos: Position,
        line: usize,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        C: Colors,
        D: Dimension,
    {
        let entity = pos.into();

        let width = ctx.dims.get_width(pos.1);

        let formatting = ctx.cfg.get_formatting(entity);
        let text_cfg = TextCfg {
            alignment: *ctx.cfg.get_alignment_horizontal(entity),
            color: ctx.colors.get_color(pos),
            justification: Colored::new(
                ctx.cfg.get_justification(entity),
                ctx.cfg.get_justification_color(entity),
            ),
            formatting: *formatting,
        };

        let pad = ctx.cfg.get_padding(entity);
        let pad_color = ctx.cfg.get_padding_color(entity);
        let valignment = *ctx.cfg.get_alignment_vertical(entity);

        let mut cell_height = ctx.records.count_lines(pos);
        if formatting.vertical_trim {
            cell_height -= count_empty_lines_at_start(ctx.records, pos)
                + count_empty_lines_at_end(ctx.records, pos);
        }

        if cell_height > height {
            // it may happen if the height estimation decide so
            cell_height = height;
        }

        let indent = top_indent(&pad, valignment, cell_height, height);
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
            let empty_lines = count_empty_lines_at_start(ctx.records, pos);
            index += empty_lines;

            if index > ctx.records.count_lines(pos) {
                return print_indent(f, pad.top.fill, width, pad_color.top.as_ref());
            }
        }

        let width = width - pad.left.size - pad.right.size;

        print_indent(f, pad.left.fill, pad.left.size, pad_color.left.as_ref())?;
        print_line(f, ctx.records, pos, index, width, text_cfg)?;
        print_indent(f, pad.right.fill, pad.right.size, pad_color.right.as_ref())?;

        Ok(())
    }

    fn print_line<F, R, C>(
        f: &mut F,
        records: &R,
        pos: Position,
        index: usize,
        available: usize,
        cfg: TextCfg<C, &'_ ANSIBuf>,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords,
        C: ANSIFmt,
    {
        let line = records.get_line(pos, index);
        let (line, line_width) = if cfg.formatting.horizontal_trim {
            let line = string_trim(line);
            let width = string_width(&line);
            (line, width)
        } else {
            let width = records.get_line_width(pos, index);
            (Cow::Borrowed(line), width)
        };

        if cfg.formatting.allow_lines_alignment {
            let indent = calculate_indent(cfg.alignment, line_width, available);
            let text = Colored::new(line.as_ref(), cfg.color);
            return print_text_padded(f, text, cfg.justification, indent);
        }

        let cell_width = if cfg.formatting.horizontal_trim {
            (0..records.count_lines(pos))
                .map(|i| records.get_line(pos, i))
                .map(|line| string_width(line.trim()))
                .max()
                .unwrap_or_default()
        } else {
            records.get_width(pos)
        };

        let indent = calculate_indent(cfg.alignment, cell_width, available);
        let text = Colored::new(line.as_ref(), cfg.color);
        print_text_padded(f, text, cfg.justification, indent)?;

        // todo: remove me?
        let rest_width = cell_width - line_width;
        repeat_char(f, ' ', rest_width)?;

        Ok(())
    }

    fn print_text_padded<F, C, C1>(
        f: &mut F,
        text: Colored<&str, C>,
        justification: Colored<char, C1>,
        indent: HIndent,
    ) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
        C1: ANSIFmt,
    {
        print_indent2(f, &justification, indent.left)?;
        print_text2(f, text)?;
        print_indent2(f, &justification, indent.right)?;

        Ok(())
    }

    fn print_text2<F, C>(f: &mut F, text: Colored<&str, C>) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
    {
        match text.color {
            Some(color) => {
                color.fmt_ansi_prefix(f)?;
                f.write_str(text.data)?;
                color.fmt_ansi_suffix(f)
            }
            None => f.write_str(text.data),
        }
    }

    fn top_indent(
        pad: &Sides<Indent>,
        alignment: AlignmentVertical,
        height: usize,
        available: usize,
    ) -> usize {
        let available = available - pad.top.size;
        let indent = indent_from_top(alignment, available, height);

        indent + pad.top.size
    }

    fn indent_from_top(alignment: AlignmentVertical, available: usize, real: usize) -> usize {
        match alignment {
            AlignmentVertical::Top => 0,
            AlignmentVertical::Bottom => available - real,
            AlignmentVertical::Center => (available - real) / 2,
        }
    }

    fn calculate_indent(alignment: AlignmentHorizontal, width: usize, available: usize) -> HIndent {
        let diff = available - width;

        let (left, right) = match alignment {
            AlignmentHorizontal::Left => (0, diff),
            AlignmentHorizontal::Right => (diff, 0),
            AlignmentHorizontal::Center => {
                let left = diff / 2;
                let rest = diff - left;
                (left, rest)
            }
        };

        HIndent { left, right }
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

    fn print_margin_top<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().top;
        let offset = cfg.get_margin_offset().top;
        let color = cfg.get_margin_color();
        let color = color.top.as_ref();
        print_indent_lines(f, indent, offset, color, width)
    }

    fn print_margin_bottom<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().bottom;
        let offset = cfg.get_margin_offset().bottom;
        let color = cfg.get_margin_color();
        let color = color.bottom.as_ref();
        print_indent_lines(f, indent, offset, color, width)
    }

    fn print_margin_left<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().left;
        let offset = cfg.get_margin_offset().left;
        let color = cfg.get_margin_color();
        let color = color.left.as_ref();
        print_margin_vertical(f, indent, offset, color, line, height)
    }

    fn print_margin_right<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().right;
        let offset = cfg.get_margin_offset().right;
        let color = cfg.get_margin_color();
        let color = color.right.as_ref();
        print_margin_vertical(f, indent, offset, color, line, height)
    }

    fn print_margin_vertical<F>(
        f: &mut F,
        indent: Indent,
        offset: Offset,
        color: Option<&ANSIBuf>,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
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

    fn print_indent_lines<F>(
        f: &mut F,
        indent: Indent,
        offset: Offset,
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
            Offset::Begin(start) => (start, 0),
            Offset::End(end) => (0, end),
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

    fn print_indent<F, C>(f: &mut F, c: char, n: usize, color: Option<C>) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
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

    fn print_indent2<F, C>(f: &mut F, c: &Colored<char, C>, n: usize) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
    {
        if n == 0 {
            return Ok(());
        }

        match &c.color {
            Some(color) => {
                color.fmt_ansi_prefix(f)?;
                repeat_char(f, c.data, n)?;
                color.fmt_ansi_suffix(f)
            }
            None => repeat_char(f, c.data, n),
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
}

mod grid_spanned {
    use super::*;

    struct TextCfg<C, C1> {
        alignment: AlignmentHorizontal,
        formatting: Formatting,
        color: Option<C>,
        justification: Colored<char, C1>,
    }

    struct Colored<T, C> {
        data: T,
        color: Option<C>,
    }

    impl<T, C> Colored<T, C> {
        fn new(data: T, color: Option<C>) -> Self {
            Self { data, color }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Shape {
        count_rows: usize,
        count_columns: usize,
    }

    struct HIndent {
        left: usize,
        right: usize,
    }

    pub(super) fn build_grid<F, R, D, C>(f: &mut F, ctx: PrintCtx<'_, R, D, C>) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        D: Dimension,
        C: Colors,
    {
        let shape = Shape {
            count_rows: ctx.records.count_rows(),
            count_columns: ctx.records.count_columns(),
        };

        let total_width = total_width(ctx.cfg, ctx.dims, shape.count_columns);

        let margin = ctx.cfg.get_margin();
        let total_width_with_margin = total_width + margin.left.size + margin.right.size;

        let total_height = total_height(ctx.cfg, ctx.dims, shape.count_rows);

        if margin.top.size > 0 {
            print_margin_top(f, ctx.cfg, total_width_with_margin)?;
            f.write_char('\n')?;
        }

        let mut table_line = 0;
        let mut prev_empty_horizontal = false;
        for row in 0..shape.count_rows {
            let count_lines = ctx.dims.get_height(row);

            if ctx.cfg.has_horizontal(row, shape.count_rows) {
                if prev_empty_horizontal {
                    f.write_char('\n')?;
                }

                print_margin_left(f, ctx.cfg, table_line, total_height)?;
                print_split_line_spanned(f, &ctx, row, shape)?;
                print_margin_right(f, ctx.cfg, table_line, total_height)?;

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
                print_margin_left(f, ctx.cfg, table_line, total_height)?;

                for col in 0..shape.count_columns {
                    let pos = (row, col);

                    if ctx.cfg.is_cell_covered_by_both_spans(pos) {
                        continue;
                    }

                    if ctx.cfg.is_cell_covered_by_column_span(pos) {
                        let is_last_column = col + 1 == shape.count_columns;
                        if is_last_column {
                            let pos = (row, col + 1);
                            let count_columns = shape.count_columns;
                            print_vertical_char(f, ctx.cfg, pos, i, count_lines, count_columns)?;
                        }

                        continue;
                    }

                    print_vertical_char(f, ctx.cfg, pos, i, count_lines, shape.count_columns)?;

                    if ctx.cfg.is_cell_covered_by_row_span(pos) {
                        // means it's part of other a spanned cell
                        // so. we just need to use line from other cell.
                        let original_row = closest_visible_row(ctx.cfg, pos).unwrap();

                        // considering that the content will be printed instead horizontal lines so we can skip some lines.
                        let mut skip_lines = (original_row..row)
                            .map(|i| ctx.dims.get_height(i))
                            .sum::<usize>();

                        skip_lines += (original_row + 1..=row)
                            .map(|row| ctx.cfg.has_horizontal(row, shape.count_rows) as usize)
                            .sum::<usize>();

                        let line = i + skip_lines;
                        let pos = (original_row, col);

                        let width = get_cell_width(ctx.cfg, ctx.dims, pos, shape.count_columns);
                        let height = get_cell_height(ctx.cfg, ctx.dims, pos, shape.count_rows);

                        print_cell_line(f, &ctx, width, height, pos, line)?;
                    } else {
                        let width = get_cell_width(ctx.cfg, ctx.dims, pos, shape.count_columns);
                        let height = get_cell_height(ctx.cfg, ctx.dims, pos, shape.count_rows);
                        print_cell_line(f, &ctx, width, height, pos, i)?;
                    }

                    let is_last_column = col + 1 == shape.count_columns;
                    if is_last_column {
                        let pos = (row, col + 1);
                        print_vertical_char(f, ctx.cfg, pos, i, count_lines, shape.count_columns)?;
                    }
                }

                print_margin_right(f, ctx.cfg, table_line, total_height)?;

                let is_last_line = i + 1 == count_lines;
                let is_last_row = row + 1 == shape.count_rows;
                if !(is_last_line && is_last_row) {
                    f.write_char('\n')?;
                }

                table_line += 1;
            }
        }

        if ctx.cfg.has_horizontal(shape.count_rows, shape.count_rows) {
            f.write_char('\n')?;
            print_margin_left(f, ctx.cfg, table_line, total_height)?;
            print_split_line(f, ctx.cfg, ctx.dims, shape.count_rows, shape)?;
            print_margin_right(f, ctx.cfg, table_line, total_height)?;
        }

        if margin.bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, ctx.cfg, total_width_with_margin)?;
        }

        Ok(())
    }

    fn print_split_line_spanned<F, R, D, C>(
        f: &mut F,
        ctx: &PrintCtx<'_, R, D, C>,
        row: usize,
        shape: Shape,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + ExactRecords + PeekableRecords,
        D: Dimension,
        C: Colors,
    {
        let mut used_color = None;

        let pos = (row, 0);
        print_vertical_intersection(f, ctx.cfg, pos, shape, &mut used_color)?;

        for col in 0..shape.count_columns {
            let pos = (row, col);
            if ctx.cfg.is_cell_covered_by_both_spans(pos) {
                continue;
            }

            if ctx.cfg.is_cell_covered_by_row_span(pos) {
                // means it's part of other a spanned cell
                // so. we just need to use line from other cell.

                let original_row = closest_visible_row(ctx.cfg, pos).unwrap();

                // considering that the content will be printed instead horizontal lines so we can skip some lines.
                let mut skip_lines = (original_row..row)
                    .map(|i| ctx.dims.get_height(i))
                    .sum::<usize>();

                // skip horizontal lines
                if row > 0 {
                    skip_lines += (original_row..row - 1)
                        .map(|row| ctx.cfg.has_horizontal(row + 1, shape.count_rows) as usize)
                        .sum::<usize>();
                }

                let pos = (original_row, col);
                let height = get_cell_height(ctx.cfg, ctx.dims, pos, shape.count_rows);
                let width = get_cell_width(ctx.cfg, ctx.dims, pos, shape.count_columns);
                let line = skip_lines;

                print_cell_line(f, ctx, width, height, pos, line)?;

                // We need to use a correct right split char.
                let mut col = col;
                if let Some(span) = ctx.cfg.get_column_span(pos) {
                    col += span - 1;
                }

                let pos = (row, col + 1);
                print_vertical_intersection(f, ctx.cfg, pos, shape, &mut used_color)?;

                continue;
            }

            let width = ctx.dims.get_width(col);
            if width > 0 {
                // general case
                let main = ctx.cfg.get_horizontal(pos, shape.count_rows);
                match main {
                    Some(c) => {
                        let clr = ctx.cfg.get_horizontal_color(pos, shape.count_rows);
                        prepare_coloring(f, clr, &mut used_color)?;
                        print_horizontal_border(f, ctx.cfg, pos, width, c, &used_color)?;
                    }
                    None => repeat_char(f, ' ', width)?,
                }
            }

            let pos = (row, col + 1);
            print_vertical_intersection(f, ctx.cfg, pos, shape, &mut used_color)?;
        }

        if let Some(clr) = used_color {
            clr.fmt_ansi_suffix(f)?;
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

    fn print_vertical_intersection<'a, F>(
        f: &mut F,
        cfg: &'a SpannedConfig,
        pos: Position,
        shape: Shape,
        used_color: &mut Option<&'a ANSIBuf>,
    ) -> fmt::Result
    where
        F: fmt::Write,
    {
        let intersection = match cfg.get_intersection(pos, (shape.count_rows, shape.count_columns))
        {
            Some(c) => c,
            None => return Ok(()),
        };

        // We need to make sure that we need to print it.
        // Specifically for cases where we have a limited amount of verticals.
        //
        // todo: Yes... this check very likely degrages performance a bit,
        //       Surely we need to rethink it.
        if !cfg.has_vertical(pos.1, shape.count_columns) {
            return Ok(());
        }

        let color = cfg.get_intersection_color(pos, (shape.count_rows, shape.count_columns));
        prepare_coloring(f, color, used_color)?;
        f.write_char(intersection)?;

        Ok(())
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

    fn print_split_line<F, D>(
        f: &mut F,
        cfg: &SpannedConfig,
        dimension: &D,
        row: usize,
        shape: Shape,
    ) -> fmt::Result
    where
        F: Write,
        D: Dimension,
    {
        let mut used_color = None;
        print_vertical_intersection(f, cfg, (row, 0), shape, &mut used_color)?;

        for col in 0..shape.count_columns {
            let width = dimension.get_width(col);

            // general case
            if width > 0 {
                let pos = (row, col);
                let main = cfg.get_horizontal(pos, shape.count_rows);
                match main {
                    Some(c) => {
                        let clr = cfg.get_horizontal_color(pos, shape.count_rows);
                        prepare_coloring(f, clr, &mut used_color)?;
                        print_horizontal_border(f, cfg, pos, width, c, &used_color)?;
                    }
                    None => repeat_char(f, ' ', width)?,
                }
            }

            let pos = (row, col + 1);
            print_vertical_intersection(f, cfg, pos, shape, &mut used_color)?;
        }

        if let Some(clr) = used_color.take() {
            clr.fmt_ansi_suffix(f)?;
        }

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

    fn print_cell_line<F, R, D, C>(
        f: &mut F,
        ctx: &PrintCtx<'_, R, D, C>,
        width: usize,
        height: usize,
        pos: Position,
        line: usize,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords + ExactRecords,
        C: Colors,
    {
        let entity = pos.into();

        let mut cell_height = ctx.records.count_lines(pos);
        let formatting = ctx.cfg.get_formatting(entity);
        if formatting.vertical_trim {
            cell_height -= count_empty_lines_at_start(ctx.records, pos)
                + count_empty_lines_at_end(ctx.records, pos);
        }

        if cell_height > height {
            // it may happen if the height estimation decide so
            cell_height = height;
        }

        let pad = ctx.cfg.get_padding(entity);
        let pad_color = ctx.cfg.get_padding_color(entity);
        let alignment = ctx.cfg.get_alignment_vertical(entity);
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
            let empty_lines = count_empty_lines_at_start(ctx.records, pos);
            index += empty_lines;

            if index > ctx.records.count_lines(pos) {
                return print_indent(f, pad.top.fill, width, pad_color.top.as_ref());
            }
        }

        print_indent(f, pad.left.fill, pad.left.size, pad_color.left.as_ref())?;

        let width = width - pad.left.size - pad.right.size;

        let line_cfg = TextCfg {
            alignment: *ctx.cfg.get_alignment_horizontal(entity),
            formatting: *formatting,
            color: ctx.colors.get_color(pos),
            justification: Colored::new(
                ctx.cfg.get_justification(entity),
                ctx.cfg.get_justification_color(entity),
            ),
        };

        print_line(f, ctx.records, pos, index, width, line_cfg)?;

        print_indent(f, pad.right.fill, pad.right.size, pad_color.right.as_ref())?;

        Ok(())
    }

    fn print_line<F, R, C, C1>(
        f: &mut F,
        records: &R,
        pos: Position,
        index: usize,
        available: usize,
        text_cfg: TextCfg<C, C1>,
    ) -> fmt::Result
    where
        F: Write,
        R: Records + PeekableRecords,
        C: ANSIFmt,
        C1: ANSIFmt,
    {
        let line = records.get_line(pos, index);
        let (line, line_width) = if text_cfg.formatting.horizontal_trim {
            let line = string_trim(line);
            let width = string_width(&line);
            (line, width)
        } else {
            let width = records.get_line_width(pos, index);
            (Cow::Borrowed(line), width)
        };

        if text_cfg.formatting.allow_lines_alignment {
            let indent = calculate_indent(text_cfg.alignment, line_width, available);
            let text = Colored::new(line.as_ref(), text_cfg.color);
            return print_text_with_pad(f, text, text_cfg.justification, indent);
        }

        let cell_width = if text_cfg.formatting.horizontal_trim {
            (0..records.count_lines(pos))
                .map(|i| records.get_line(pos, i))
                .map(|line| string_width(line.trim()))
                .max()
                .unwrap_or_default()
        } else {
            records.get_width(pos)
        };

        let indent = calculate_indent(text_cfg.alignment, cell_width, available);
        let text = Colored::new(line.as_ref(), text_cfg.color);
        print_text_with_pad(f, text, text_cfg.justification, indent)?;

        // todo: remove me?
        let rest_width = cell_width - line_width;
        repeat_char(f, ' ', rest_width)?;

        Ok(())
    }

    fn print_text_with_pad<F, C, C1>(
        f: &mut F,
        text: Colored<&str, C>,
        space: Colored<char, C1>,
        indent: HIndent,
    ) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
        C1: ANSIFmt,
    {
        print_indent(f, space.data, indent.left, space.color.as_ref())?;
        print_text(f, text.data, text.color)?;
        print_indent(f, space.data, indent.right, space.color.as_ref())?;
        Ok(())
    }

    fn print_text<F, C>(f: &mut F, text: &str, clr: Option<C>) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
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

    fn calculate_indent(alignment: AlignmentHorizontal, width: usize, available: usize) -> HIndent {
        let diff = available - width;

        let (left, right) = match alignment {
            AlignmentHorizontal::Left => (0, diff),
            AlignmentHorizontal::Right => (diff, 0),
            AlignmentHorizontal::Center => {
                let left = diff / 2;
                let rest = diff - left;
                (left, rest)
            }
        };

        HIndent { left, right }
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

    fn print_margin_top<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().top;
        let offset = cfg.get_margin_offset().top;
        let color = cfg.get_margin_color();
        let color = color.top.as_ref();
        print_indent_lines(f, &indent, &offset, color, width)
    }

    fn print_margin_bottom<F>(f: &mut F, cfg: &SpannedConfig, width: usize) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().bottom;
        let offset = cfg.get_margin_offset().bottom;
        let color = cfg.get_margin_color();
        let color = color.bottom.as_ref();
        print_indent_lines(f, &indent, &offset, color, width)
    }

    fn print_margin_left<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().left;
        let offset = cfg.get_margin_offset().left;
        let color = cfg.get_margin_color();
        let color = color.left.as_ref();
        print_margin_vertical(f, indent, offset, color, line, height)
    }

    fn print_margin_right<F>(
        f: &mut F,
        cfg: &SpannedConfig,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
        let indent = cfg.get_margin().right;
        let offset = cfg.get_margin_offset().right;
        let color = cfg.get_margin_color();
        let color = color.right.as_ref();
        print_margin_vertical(f, indent, offset, color, line, height)
    }

    fn print_margin_vertical<F>(
        f: &mut F,
        indent: Indent,
        offset: Offset,
        color: Option<&ANSIBuf>,
        line: usize,
        height: usize,
    ) -> fmt::Result
    where
        F: Write,
    {
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

    fn print_indent<F, C>(f: &mut F, c: char, n: usize, color: Option<C>) -> fmt::Result
    where
        F: Write,
        C: ANSIFmt,
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

    fn get_cell_width<D>(cfg: &SpannedConfig, dims: &D, pos: Position, max: usize) -> usize
    where
        D: Dimension,
    {
        match cfg.get_column_span(pos) {
            Some(span) => {
                let start = pos.1;
                let end = start + span;
                range_width(dims, start, end) + count_verticals_range(cfg, start, end, max)
            }
            None => dims.get_width(pos.1),
        }
    }

    fn range_width<D>(dims: &D, start: usize, end: usize) -> usize
    where
        D: Dimension,
    {
        (start..end).map(|col| dims.get_width(col)).sum::<usize>()
    }

    fn count_verticals_range(cfg: &SpannedConfig, start: usize, end: usize, max: usize) -> usize {
        (start + 1..end)
            .map(|i| cfg.has_vertical(i, max) as usize)
            .sum()
    }

    fn get_cell_height<D>(cfg: &SpannedConfig, dims: &D, pos: Position, max: usize) -> usize
    where
        D: Dimension,
    {
        match cfg.get_row_span(pos) {
            Some(span) => {
                let start = pos.0;
                let end = pos.0 + span;
                range_height(dims, start, end) + count_horizontals_range(cfg, start, end, max)
            }
            None => dims.get_height(pos.0),
        }
    }

    fn range_height<D>(dims: &D, start: usize, end: usize) -> usize
    where
        D: Dimension,
    {
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
        #[cfg(feature = "ansi")]
        {
            ansi_str::AnsiStr::ansi_trim(text)
        }

        #[cfg(not(feature = "ansi"))]
        {
            text.trim().into()
        }
    }
}
