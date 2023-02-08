//! The module contains a [`Grid`] structure.

use std::{
    borrow::Cow,
    cmp,
    collections::BTreeMap,
    fmt::{self, Display, Write},
};

use crate::{
    color::{AnsiColor, Color},
    colors::{self, Colors},
    config::{
        AlignmentHorizontal, AlignmentVertical, EntityMap, Formatting, GridConfig, Indent, Offset,
        Padding, PaddingColor, Position,
    },
    dimension::Dimension,
    grid_projection::GridProjection,
    records::Records,
    util::string::{
        count_lines, get_lines, string_width, string_width_multiline_tab, string_width_tab, Lines,
    },
};

const DEFAULT_SPACE_CHAR: char = ' ';
const DEFAULT_BORDER_HORIZONTAL_CHAR: char = ' ';

type ColorsMap = EntityMap<Option<AnsiColor<'static>>>;

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct Grid<'a, R, D, C = ColorsMap> {
    records: R,
    config: &'a GridConfig,
    dimension: &'a D,
    colors: Option<C>,
}

impl<'a, R, D> Grid<'a, R, D, ColorsMap> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: &'a GridConfig, dimension: &'a D) -> Self {
        Grid {
            records,
            config,
            dimension,
            colors: None,
        }
    }
}

impl<'a, R, D, C> Grid<'a, R, D, C> {
    pub fn set_colors<Colors: colors::Colors>(self, colors: Colors) -> Grid<'a, R, D, Colors> {
        Grid {
            records: self.records,
            config: self.config,
            dimension: self.dimension,
            colors: Some(colors),
        }
    }

    pub fn build<F: Write>(self, mut f: F) -> fmt::Result
    where
        R: Records,
        D: Dimension,
        C: Colors,
    {
        if self.records.count_columns() == 0 {
            return Ok(());
        }

        print_grid(
            &mut f,
            self.records,
            self.config,
            self.dimension,
            self.colors.as_ref(),
        )
    }

    pub fn to_string(self) -> String
    where
        R: Records,
        D: Dimension,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranted to never happen otherwise it's considered an stdlib erorr or impl error");
        buf
    }
}

impl<R, D, C> Display for Grid<'_, R, D, C>
where
    for<'a> &'a R: Records,
    D: Dimension,
    C: Colors,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let records = &self.records;
        if records.count_columns() == 0 {
            return Ok(());
        }

        print_grid(
            f,
            records,
            self.config,
            self.dimension,
            self.colors.as_ref(),
        )
    }
}

fn print_grid<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &GridConfig,
    dimension: &D,
    colors: Option<&C>,
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
    cfg: &GridConfig,
    dims: &D,
    colors: Option<&C>,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let total_width = total_width(cfg, dims, count_columns);

    let margin = cfg.get_margin();
    let total_width_with_margin = total_width + margin.left.size + margin.right.size;

    let totalh = records
        .hint_count_rows()
        .map(|count_rows| total_height(cfg, dims, count_rows));

    if cfg.get_margin().top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut rbuf = Vec::with_capacity(count_columns);

    let mut records_iter = records.iter_rows().into_iter();
    let mut next_columns = records_iter.next();

    let mut row = 0;
    let mut line = 0;
    let mut is_prev_row_skipped = false;
    while let Some(columns) = next_columns {
        let columns = columns.into_iter();
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let height = dims.get_height(row);
        let count_rows = convert_count_rows(row, is_last_row);
        let has_horizontal = has_horizontal(cfg, row, count_rows);

        if row > 0 && !is_prev_row_skipped && (has_horizontal || height > 0) {
            f.write_char('\n')?;
        }

        if has_horizontal {
            let shape = (count_rows, count_columns);
            print_horizontal_line(f, cfg, line, totalh, dims, row, total_width, shape)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        let shape = (count_rows, count_columns);
        match height {
            0 => {}
            1 => {
                print_single_line_columns(f, columns, cfg, colors, dims, row, line, totalh, shape)?
            }
            _ => {
                print_multiline_columns(
                    f, &mut rbuf, columns, cfg, colors, dims, height, row, line, totalh, shape,
                )?;
            }
        }

        if height == 0 && !has_horizontal {
            is_prev_row_skipped = true;
        } else {
            is_prev_row_skipped = false;
        }

        line += height;
        row += 1;
    }

    if row > 0 {
        if has_horizontal(cfg, row, row) {
            f.write_char('\n')?;
            let shape = (row, count_columns);
            print_horizontal_line(f, cfg, line, totalh, dims, row, total_width, shape)?;
        }

        if cfg.get_margin().bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, cfg, total_width_with_margin)?;
        }
    }

    Ok(())
}

fn has_horizontal(cfg: &GridConfig, row: usize, count_rows: usize) -> bool {
    GridProjection::new(cfg)
        .count_rows(count_rows)
        .has_horizontal(row)
}

fn total_width<D: Dimension>(cfg: &GridConfig, dims: &D, count_columns: usize) -> usize {
    GridProjection::new(cfg)
        .count_columns(count_columns)
        .total_width(dims)
}

fn total_height<D: Dimension>(cfg: &GridConfig, dims: &D, count_rows: usize) -> usize {
    GridProjection::new(cfg)
        .count_rows(count_rows)
        .total_height(dims)
}

fn print_horizontal_line<F: Write, D: Dimension>(
    f: &mut F,
    cfg: &GridConfig,
    line: usize,
    totalh: Option<usize>,
    dimension: &D,
    row: usize,
    total_width: usize,
    shape: (usize, usize),
) -> Result<(), fmt::Error> {
    print_margin_left(f, cfg, line, totalh)?;
    print_split_line(f, cfg, dimension, row, total_width, shape)?;
    print_margin_right(f, cfg, line, totalh)?;
    Ok(())
}

type CLines<'a, S, C> = CellLines<'a, S, <C as Colors>::Color>;

fn print_multiline_columns<'a, 'b, F, I, D, C>(
    f: &mut F,
    buf: &mut Vec<CLines<'a, I::Item, C>>,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    dimension: &D,
    height: usize,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    collect_columns(buf, columns, cfg, colors, dimension, height, row);
    print_columns_lines(f, buf, height, cfg, line, row, totalh, shape)?;

    buf.clear();

    Ok(())
}

fn print_single_line_columns<'a, F, I, D, C>(
    f: &mut F,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    dims: &D,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    print_margin_left(f, cfg, line, totalh)?;

    for (col, cell) in columns.enumerate() {
        print_vertical_char(f, cfg, (row, col), 0, 1, shape)?;

        let width = dims.get_width(col);
        let color = colors.and_then(|c| c.get_color((row, col)));
        print_single_line_column(f, cell.as_ref(), cfg, width, color, (row, col))?;
    }

    print_vertical_char(f, cfg, (row, shape.1), 0, 1, shape)?;

    print_margin_right(f, cfg, line, totalh)?;

    Ok(())
}

fn print_single_line_column<F: Write, C: Color>(
    f: &mut F,
    text: &str,
    cfg: &GridConfig,
    width: usize,
    color: Option<&C>,
    pos: Position,
) -> Result<(), fmt::Error> {
    let pos = pos.into();
    let pad = cfg.get_padding(pos);
    let pad_color = cfg.get_padding_color(pos);

    let formatting = cfg.get_formatting(pos);

    let (text, text_width) = if formatting.horizontal_trim && !text.is_empty() {
        let text = string_trim(text);
        let width = string_width_tab(&text, cfg.get_tab_width());

        (text, width)
    } else {
        let text = Cow::Borrowed(text);
        let width = string_width_multiline_tab(&text, cfg.get_tab_width());

        (text, width)
    };

    let alignment = *cfg.get_alignment_horizontal(pos);
    let available_width = width - pad.left.size - pad.right.size;
    let (left, right) = calculate_indent(alignment, text_width, available_width);

    print_indent(f, pad.left.fill, pad.left.size, &pad_color.left)?;

    repeat_char(f, DEFAULT_SPACE_CHAR, left)?;
    print_text(f, &text, cfg.get_tab_width(), color)?;
    repeat_char(f, DEFAULT_SPACE_CHAR, right)?;

    print_indent(f, pad.right.fill, pad.right.size, &pad_color.right)?;

    Ok(())
}

fn print_columns_lines<S, F: Write, C: Color>(
    f: &mut F,
    columns: &mut [CellLines<'_, S, C>],
    height: usize,
    cfg: &GridConfig,
    line: usize,
    row: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error> {
    for i in 0..height {
        let exact_line = line + i;

        print_margin_left(f, cfg, exact_line, totalh)?;

        for (col, cell) in columns.iter_mut().enumerate() {
            print_vertical_char(f, cfg, (row, col), i, height, shape)?;
            cell.display(f, cfg.get_tab_width())?;
        }

        print_vertical_char(f, cfg, (row, shape.1), i, height, shape)?;

        print_margin_right(f, cfg, exact_line, totalh)?;

        if i + 1 != height {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn collect_columns<'a, I, D, C>(
    row_columns: &mut Vec<CLines<'a, I::Item, C>>,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    dimension: &D,

    height: usize,
    row: usize,
) where
    I: Iterator,
    I::Item: AsRef<str>,
    C: Colors,
    D: Dimension,
{
    let iter = columns.enumerate().map(|(col, cell)| {
        let width = dimension.get_width(col);
        let pos = (row, col).into();

        CellLines::new(
            cell,
            width,
            height,
            cfg.get_formatting(pos),
            cfg.get_padding(pos),
            cfg.get_padding_color(pos),
            *cfg.get_alignment_horizontal(pos),
            *cfg.get_alignment_vertical(pos),
            cfg.get_tab_width(),
            colors.and_then(|c| c.get_color((row, col))),
        )
    });

    row_columns.extend(iter);
}

fn print_split_line<F: Write, D: Dimension>(
    f: &mut F,
    cfg: &GridConfig,
    dimension: &D,
    row: usize,
    total_width: usize,
    shape: (usize, usize),
) -> fmt::Result {
    let mut override_text = cfg
        .get_split_line_text(row)
        .and_then(|text| get_lines(text).next())
        .unwrap_or_default()
        .into_owned();
    let override_text_offset = cfg.get_split_line_offset(row).unwrap_or(Offset::Begin(0));
    let override_text_pos = offset_start_pos(override_text_offset, total_width);

    let gp = GridProjection::with_shape(cfg, shape);

    let mut used_color = None;

    let mut i = 0;
    for col in 0..shape.1 {
        if col == 0 {
            let left = gp.get_intersection((row, col));
            if let Some(c) = left {
                if i >= override_text_pos && !override_text.is_empty() {
                    let (c, rest) = spplit_str_at(&override_text, 1);
                    f.write_str(&c)?;
                    override_text = rest.into_owned();
                    if string_width(&override_text) == 0 {
                        override_text = String::new()
                    }
                } else {
                    let clr = gp.get_intersection_color((row, col));
                    if let Some(clr) = clr {
                        clr.fmt_prefix(f)?;
                        used_color = Some(clr);
                    }

                    f.write_char(c)?;
                    i += 1;
                }
            }
        }

        let mut width = dimension.get_width(col);

        // a situation when need to partially print split
        if i < override_text_pos && i + width >= override_text_pos {
            let available = override_text_pos - i;
            width -= available;
            i += available;
            let width = available;

            let main = gp.get_horizontal((row, col));
            match main {
                Some(c) => {
                    let clr = gp.get_horizontal_color((row, col));
                    prepare_coloring(f, clr, &mut used_color)?;

                    print_horizontal_border(f, cfg, (row, col), width, c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }
        }

        if i >= override_text_pos && !override_text.is_empty() {
            let text_width = string_width_tab(&override_text, cfg.get_tab_width());
            let print_width = cmp::min(text_width, width);
            let (c, rest) = spplit_str_at(&override_text, print_width);
            f.write_str(&c)?;
            override_text = rest.into_owned();
            if string_width(&override_text) == 0 {
                override_text = String::new()
            }

            width -= print_width;
        }

        // general case
        if width > 0 {
            let main = gp.get_horizontal((row, col));
            match main {
                Some(c) => {
                    let clr = gp.get_horizontal_color((row, col));
                    prepare_coloring(f, clr, &mut used_color)?;

                    print_horizontal_border(f, cfg, (row, col), width, c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }

            i += width;
        }

        let right = gp.get_intersection((row, col + 1));
        if let Some(c) = right {
            if i >= override_text_pos && !override_text.is_empty() {
                let (c, rest) = spplit_str_at(&override_text, 1);
                f.write_str(&c)?;
                override_text = rest.into_owned();
                if string_width(&override_text) == 0 {
                    override_text = String::new()
                }
            } else {
                let clr = gp.get_intersection_color((row, col + 1));
                prepare_coloring(f, clr, &mut used_color)?;

                f.write_char(c)?;
                i += 1;
            }
        }
    }

    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_grid_spanned<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &GridConfig,
    dims: &D,
    colors: Option<&C>,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let total_width = total_width(cfg, dims, count_columns);
    let margin = cfg.get_margin();
    let total_width_with_margin = total_width + margin.left.size + margin.right.size;

    let totalh = records
        .hint_count_rows()
        .map(|rows| total_height(cfg, dims, rows));

    if cfg.get_margin().top.size > 0 {
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

        let has_horizontal = has_horizontal(cfg, row, count_rows);
        if need_new_line && (has_horizontal || height > 0) {
            f.write_char('\n')?;
            need_new_line = false;
        }

        if has_horizontal {
            print_margin_left(f, cfg, line, totalh)?;
            print_split_line_spanned(f, &mut buf, cfg, dims, total_width, row, shape)?;
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
        if has_horizontal(cfg, row, row) {
            f.write_char('\n')?;
            let shape = (row, count_columns);
            print_horizontal_line(f, cfg, line, totalh, dims, row, total_width, shape)?;
        }

        if cfg.get_margin().bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, cfg, total_width_with_margin)?;
        }
    }

    Ok(())
}

fn print_split_line_spanned<S, F: Write, D: Dimension, C: Color>(
    f: &mut F,
    columns: &mut BTreeMap<usize, (CellLines<'_, S, C>, usize, usize)>,
    cfg: &GridConfig,
    dimension: &D,
    total_width: usize,
    row: usize,
    shape: (usize, usize),
) -> fmt::Result {
    let mut override_text = cfg
        .get_split_line_text(row)
        .and_then(|text| get_lines(text).next())
        .unwrap_or_default()
        .into_owned();
    let override_text_offset = cfg.get_split_line_offset(row).unwrap_or(Offset::Begin(0));
    let override_text_pos = offset_start_pos(override_text_offset, total_width);

    let gp = GridProjection::with_shape(cfg, shape);

    let mut used_color = None;

    let mut i = 0;
    for col in 0..shape.1 {
        if col == 0 {
            let left = gp.get_intersection((row, col));
            if let Some(c) = left {
                if i >= override_text_pos && !override_text.is_empty() {
                    let (c, rest) = spplit_str_at(&override_text, 1);
                    f.write_str(&c)?;
                    override_text = rest.into_owned();
                    if string_width(&override_text) == 0 {
                        override_text = String::new()
                    }
                } else {
                    let clr = gp.get_intersection_color((row, col));
                    if let Some(clr) = clr {
                        clr.fmt_prefix(f)?;
                        used_color = Some(clr);
                    }

                    f.write_char(c)?;
                    i += 1;
                }
            }
        }

        if gp.is_cell_covered_by_both_spans((row, col)) {
            continue;
        }

        let is_spanned_split_line_part = gp.is_cell_covered_by_row_span((row, col));

        let mut width = dimension.get_width(col);

        let mut col = col;
        if is_spanned_split_line_part {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let (cell, _, _) = columns.get_mut(&col).unwrap();
            cell.display(f, cfg.get_tab_width())?;

            // We need to use a correct right split char.
            let original_row = closest_visible_row(cfg, (row, col), shape).unwrap();
            if let Some(span) = gp.get_span_column((original_row, col)) {
                col += span - 1;
            }
        } else if width > 0 {
            // a situation when need to partially print split
            if i < override_text_pos && i + width >= override_text_pos {
                let available = override_text_pos - i;
                width -= available;
                i += available;
                let width = available;

                let main = gp.get_horizontal((row, col));
                match main {
                    Some(c) => {
                        let clr = gp.get_horizontal_color((row, col));
                        prepare_coloring(f, clr, &mut used_color)?;

                        print_horizontal_border(f, cfg, (row, col), width, c)?;
                    }
                    None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
                }
            }

            if i >= override_text_pos && !override_text.is_empty() {
                let text_width = string_width_tab(&override_text, cfg.get_tab_width());
                let print_width = cmp::min(text_width, width);

                let (c, rest) = spplit_str_at(&override_text, print_width);
                f.write_str(&c)?;

                override_text = rest.into_owned();
                if string_width(&override_text) == 0 {
                    override_text = String::new()
                }

                width -= print_width;
            }

            // general case
            let main = gp.get_horizontal((row, col));
            match main {
                Some(c) => {
                    let clr = gp.get_horizontal_color((row, col));
                    prepare_coloring(f, clr, &mut used_color)?;

                    print_horizontal_border(f, cfg, (row, col), width, c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }

            i += width;
        }

        let right = gp.get_intersection((row, col + 1));
        if let Some(c) = right {
            if i >= override_text_pos && !override_text.is_empty() {
                let (c, rest) = spplit_str_at(&override_text, 1);
                f.write_str(&c)?;
                override_text = rest.into_owned();
                if string_width(&override_text) == 0 {
                    override_text = String::new()
                }
            } else {
                let clr = gp.get_intersection_color((row, col + 1));
                prepare_coloring(f, clr, &mut used_color)?;

                f.write_char(c)?;
                i += 1;
            }
        }
    }

    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_spanned_columns<'a, F, I, D, C>(
    f: &mut F,
    columns: &mut BTreeMap<usize, (CLines<'a, I::Item, C>, usize, usize)>,
    iter: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    dimension: &D,
    this_height: usize,
    row: usize,
    line: usize,
    totalh: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    if this_height == 0 {
        let gp = GridProjection::with_shape(cfg, shape);

        // it's possible that we dont show row but it contains an actuall cell which will be
        // rendered after all cause it's a rowspanned

        let mut skip = 0;
        for (col, cell) in iter.enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if let Some((_, _, colspan)) = columns.get(&col) {
                skip = *colspan - 1;
                continue;
            }

            let rowspan = cfg.get_span_row((row, col)).unwrap_or(1);
            if rowspan < 2 {
                continue;
            }

            let height = if rowspan > 1 {
                range_height(cfg, dimension, row, row + rowspan, shape.0)
            } else {
                this_height
            };

            let colspan = gp.get_span_column((row, col)).unwrap_or(1);
            skip = colspan - 1;
            let width = if colspan > 1 {
                range_width(cfg, dimension, col, col + colspan, shape.1)
            } else {
                dimension.get_width(col)
            };

            let pos = (row, col).into();
            let cell = CellLines::new(
                cell,
                width,
                height,
                cfg.get_formatting(pos),
                cfg.get_padding(pos),
                cfg.get_padding_color(pos),
                *cfg.get_alignment_horizontal(pos),
                *cfg.get_alignment_vertical(pos),
                cfg.get_tab_width(),
                colors.and_then(|c| c.get_color((row, col))),
            );

            columns.insert(col, (cell, rowspan, colspan));
        }

        columns.retain(|_, (_, rowspan, _)| {
            *rowspan -= 1;
            *rowspan != 0
        });

        return Ok(());
    }

    let gp = GridProjection::with_shape(cfg, shape);

    let mut skip = 0;
    for (col, cell) in iter.enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if let Some((_, _, colspan)) = columns.get(&col) {
            skip = *colspan - 1;
            continue;
        }

        let colspan = gp.get_span_column((row, col)).unwrap_or(1);
        skip = colspan - 1;
        let width = if colspan > 1 {
            range_width(cfg, dimension, col, col + colspan, shape.1)
        } else {
            dimension.get_width(col)
        };

        let rowspan = cfg.get_span_row((row, col)).unwrap_or(1);
        let height = if rowspan > 1 {
            range_height(cfg, dimension, row, row + rowspan, shape.0)
        } else {
            this_height
        };

        let pos = (row, col).into();
        let cell = CellLines::new(
            cell,
            width,
            height,
            cfg.get_formatting(pos),
            cfg.get_padding(pos),
            cfg.get_padding_color(pos),
            *cfg.get_alignment_horizontal(pos),
            *cfg.get_alignment_vertical(pos),
            cfg.get_tab_width(),
            colors.and_then(|c| c.get_color((row, col))),
        );

        columns.insert(col, (cell, rowspan, colspan));
    }

    for i in 0..this_height {
        let exact_line = line + i;
        let cell_line = i;

        print_margin_left(f, cfg, exact_line, totalh)?;

        for (&col, (cell, _, _)) in columns.iter_mut() {
            print_vertical_char(f, cfg, (row, col), cell_line, this_height, shape)?;
            cell.display(f, cfg.get_tab_width())?;
        }

        print_vertical_char(f, cfg, (row, shape.1), cell_line, this_height, shape)?;

        print_margin_right(f, cfg, exact_line, totalh)?;

        if i + 1 != this_height {
            f.write_char('\n')?;
        }
    }

    columns.retain(|_, (_, rowspan, _)| {
        *rowspan -= 1;
        *rowspan != 0
    });

    Ok(())
}

fn print_horizontal_border<F: Write>(
    f: &mut F,
    cfg: &GridConfig,
    pos: Position,
    width: usize,
    c: char,
) -> fmt::Result {
    if cfg.is_overridden_horizontal(pos) {
        for i in 0..width {
            let c = cfg.lookup_overridden_horizontal(pos, i, width).unwrap_or(c);

            f.write_char(c)?;
        }
    } else {
        repeat_char(f, c, width)?;
    }

    Ok(())
}

struct CellLines<'a, S, C> {
    lines: LinesIter<S>,
    maxwidth: usize,
    top_indent: usize,
    alignmenth: AlignmentHorizontal,
    formatting: &'a Formatting,
    padding: &'a Padding,
    padding_color: &'a PaddingColor<'static>,
    color: Option<&'a C>,
    indent: Option<usize>,
}

impl CellLines<'_, (), ()> {
    fn new<'a, C: Color, A: AsRef<str>>(
        text: A,
        maxwidth: usize,
        height: usize,
        formatting: &'a Formatting,
        padding: &'a Padding,
        padding_color: &'a PaddingColor<'static>,
        alignmenth: AlignmentHorizontal,
        alignmentv: AlignmentVertical,
        tab_width: usize,
        color: Option<&'a C>,
    ) -> CellLines<'a, A, C> {
        let (cell_height, vindent) = get_top_bottom_skip(text.as_ref(), formatting);
        let top_indent = top_indent(padding, alignmentv, cell_height, height);

        let mut indent = None;
        if !formatting.allow_lines_alignment {
            let available_width = maxwidth - padding.left.size - padding.right.size;
            let trim = formatting.horizontal_trim;
            let hindent =
                get_left_right_indent(text.as_ref(), alignmenth, trim, tab_width, available_width);
            indent = Some(hindent);
        }

        let mut lines = LinesIter::new(text);
        if let Some(top) = vindent {
            for _ in 0..top {
                let _ = lines.lines.next();
            }
        }

        CellLines {
            lines,
            indent,
            top_indent,
            maxwidth,
            alignmenth,
            formatting,
            padding,
            padding_color,
            color,
        }
    }
}

impl<S, C> CellLines<'_, S, C>
where
    C: Color,
{
    fn display<F: Write>(&mut self, f: &mut F, tab_width: usize) -> fmt::Result {
        // todo: fix bottom/top padding and indent symbol issue

        let pad = &self.padding;
        let pad_color = &self.padding_color;
        let formatting = &self.formatting;
        let alignment = self.alignmenth;
        let color = self.color;

        if self.top_indent > 0 {
            self.top_indent -= 1;
            return print_indent(f, pad.top.fill, self.maxwidth, &pad_color.top);
        }

        let line = match self.lines.lines.next() {
            Some(line) => line,
            None => return print_indent(f, pad.bottom.fill, self.maxwidth, &pad_color.bottom),
        };

        let available_width = self.maxwidth - pad.left.size - pad.right.size;

        let line = if formatting.horizontal_trim && !line.is_empty() {
            string_trim(&line)
        } else {
            line
        };

        let line_width = string_width_tab(&line, tab_width);

        let (left, right) = if formatting.allow_lines_alignment {
            calculate_indent(alignment, line_width, available_width)
        } else {
            let left = self.indent.expect("must be here");
            (left, available_width - line_width - left)
        };

        print_indent(f, pad.left.fill, pad.left.size, &pad_color.left)?;

        repeat_char(f, DEFAULT_SPACE_CHAR, left)?;
        print_text(f, &line, tab_width, color)?;
        repeat_char(f, DEFAULT_SPACE_CHAR, right)?;

        print_indent(f, pad.right.fill, pad.right.size, &pad_color.right)?;

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
        // It's currerently not possible due to a lifetime issues. (It's known as self-referential struct)
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

fn get_top_bottom_skip(cell: &str, format: &Formatting) -> (usize, Option<usize>) {
    let count_lines = count_lines(cell);
    if format.vertical_trim {
        let (top, bottom) = count_empty_lines(cell);
        (count_lines - bottom - top, Some(top))
    } else {
        (count_lines, None)
    }
}

fn get_left_right_indent(
    cell: &str,
    alignment: AlignmentHorizontal,
    trim: bool,
    tab_width: usize,
    available: usize,
) -> usize {
    let cell_width = if trim {
        get_lines(cell)
            .into_iter()
            .map(|line| string_width_tab(line.trim(), tab_width))
            .max()
            .unwrap_or(0)
    } else {
        string_width_multiline_tab(cell, tab_width)
    };

    let (left, _) = calculate_indent(alignment, cell_width, available);

    left
}

fn print_text<F: Write>(f: &mut F, s: &str, tab: usize, clr: Option<impl Color>) -> fmt::Result {
    match clr {
        Some(color) => {
            color.fmt_prefix(f)?;
            print_str(f, s, tab)?;
            color.fmt_suffix(f)?;
        }
        None => {
            print_str(f, s, tab)?;
        }
    }

    Ok(())
}

fn print_str<F: Write>(f: &mut F, text: &str, tab_width: usize) -> fmt::Result {
    // So to not use replace_tab we are printing by char;
    // Hopefully it's more affective as it reduceses a number of allocations.
    for c in text.chars() {
        match c {
            '\r' => (),
            '\t' => repeat_char(f, ' ', tab_width)?,
            c => f.write_char(c)?,
        }
    }

    Ok(())
}

fn prepare_coloring<'a, F: Write>(
    f: &mut F,
    clr: Option<&'a AnsiColor<'a>>,
    used_color: &mut Option<&'a AnsiColor<'a>>,
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
    padding: &Padding,
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

fn count_empty_lines(cell: &str) -> (usize, usize) {
    let mut top = 0;
    let mut bottom = 0;
    let mut top_check = true;

    for line in get_lines(cell) {
        let is_empty = line.trim().is_empty();
        if top_check {
            if is_empty {
                top += 1;
            } else {
                top_check = false;
            }
        } else {
            if is_empty {
                bottom += 1;
            } else {
                bottom = 0;
            }
        }
    }

    (top, bottom)
}

fn repeat_char<F: Write>(f: &mut F, c: char, n: usize) -> fmt::Result {
    for _ in 0..n {
        f.write_char(c)?;
    }

    Ok(())
}

fn print_vertical_char<F: Write>(
    f: &mut F,
    cfg: &GridConfig,
    pos: Position,
    line: usize,
    count_lines: usize,
    shape: (usize, usize),
) -> fmt::Result {
    // todo: Add Border/verticals to CellLines structure to not make these lookup calls

    let gp = GridProjection::with_shape(cfg, shape);

    let symbol = match gp.get_vertical(pos) {
        Some(c) => c,
        None => return Ok(()),
    };

    let symbol = cfg
        .is_overridden_vertical(pos)
        .then(|| cfg.lookup_overridden_vertical(pos, line, count_lines))
        .flatten()
        .unwrap_or(symbol);

    match gp.get_vertical_color(pos) {
        Some(clr) => {
            clr.fmt_prefix(f)?;
            f.write_char(symbol)?;
            clr.fmt_suffix(f)?;
        }
        None => f.write_char(symbol)?,
    }

    Ok(())
}

fn print_margin_top<F: Write>(f: &mut F, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().top,
        &cfg.get_margin_offset().top,
        &cfg.get_margin_color().top,
        width,
    )
}

fn print_margin_bottom<F: Write>(f: &mut F, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().bottom,
        &cfg.get_margin_offset().bottom,
        &cfg.get_margin_color().bottom,
        width,
    )
}

fn print_margin_left<F: Write>(
    f: &mut F,
    cfg: &GridConfig,
    line: usize,
    height: Option<usize>,
) -> fmt::Result {
    print_margin_vertical(
        f,
        cfg.get_margin().left,
        cfg.get_margin_offset().left,
        &cfg.get_margin_color().left,
        line,
        height,
    )
}

fn print_margin_right<F: Write>(
    f: &mut F,
    cfg: &GridConfig,
    line: usize,
    height: Option<usize>,
) -> fmt::Result {
    print_margin_vertical(
        f,
        cfg.get_margin().right,
        cfg.get_margin_offset().right,
        &cfg.get_margin_color().right,
        line,
        height,
    )
}

fn print_margin_vertical<F: Write>(
    f: &mut F,
    indent: Indent,
    offset: Offset,
    color: &AnsiColor<'_>,
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
                print_indent(f, ' ', indent.size, &AnsiColor::default())?;
            }
        }
        Offset::End(mut offset) => {
            if let Some(max) = height {
                offset = cmp::min(offset, max);
                let pos = max - offset;

                if line >= pos {
                    print_indent(f, ' ', indent.size, &AnsiColor::default())?;
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
    color: &AnsiColor<'_>,
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
            print_indent(f, ' ', start_offset, &AnsiColor::default())?;
        }

        if indent_size > 0 {
            print_indent(f, indent.fill, indent_size, color)?;
        }

        if end_offset > 0 {
            print_indent(f, ' ', end_offset, &AnsiColor::default())?;
        }

        if i + 1 != indent.size {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn print_indent<F: Write>(f: &mut F, c: char, n: usize, color: &AnsiColor<'_>) -> fmt::Result {
    color.fmt_prefix(f)?;
    repeat_char(f, c, n)?;
    color.fmt_suffix(f)?;

    Ok(())
}

fn range_width<D: Dimension>(
    cfg: &GridConfig,
    dims: &D,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    let count_borders = count_borders_in_range(cfg, start, end, count_columns);
    let range_width = (start..end).map(|col| dims.get_width(col)).sum::<usize>();

    count_borders + range_width
}

fn count_borders_in_range(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    let gp = GridProjection::new(cfg).count_columns(count_columns);

    (start..end).skip(1).filter(|&i| gp.has_vertical(i)).count()
}

fn range_height<D: Dimension>(
    cfg: &GridConfig,
    dims: &D,
    start: usize,
    end: usize,
    count_rows: usize,
) -> usize {
    let count_borders = count_horizontal_borders_in_range(cfg, start, end, count_rows);
    let range_width = (start..end).map(|col| dims.get_height(col)).sum::<usize>();

    count_borders + range_width
}

fn count_horizontal_borders_in_range(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_rows: usize,
) -> usize {
    let gp = GridProjection::new(cfg).count_rows(count_rows);

    (start..end)
        .skip(1)
        .filter(|&i| gp.has_horizontal(i))
        .count()
}

fn closest_visible_row(
    cfg: &GridConfig,
    mut pos: Position,
    shape: (usize, usize),
) -> Option<usize> {
    let gp = GridProjection::with_shape(cfg, shape);

    loop {
        if gp.is_cell_visible(pos) {
            return Some(pos.0);
        }

        if pos.0 == 0 {
            return None;
        }

        pos.0 -= 1;
    }
}

fn offset_start_pos(offset: Offset, length: usize) -> usize {
    match offset {
        Offset::Begin(o) => o,
        Offset::End(o) => {
            if o > length {
                length
            } else {
                length - o
            }
        }
    }
}

fn convert_count_rows(row: usize, is_last: bool) -> usize {
    if is_last {
        row + 1
    } else {
        row + 2
    }
}

/// Get string at
///
/// BE AWARE: width is expected to be in bytes.
fn spplit_str_at(text: &str, at: usize) -> (Cow<'_, str>, Cow<'_, str>) {
    #[cfg(feature = "color")]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let stripped = ansi_str::AnsiStr::ansi_strip(text);
        let (length, count_unknowns, _) = split_at_pos(&stripped, at);

        let mut buf = ansi_str::AnsiStr::ansi_cut(text, ..length);

        if count_unknowns > 0 {
            let mut b = buf.into_owned();
            b.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            buf = Cow::Owned(b);
        }

        let rest = ansi_str::AnsiStr::ansi_cut(text, length..);

        (buf, rest)
    }
    #[cfg(not(feature = "color"))]
    {
        const REPLACEMENT: char = '\u{FFFD}';

        let (length, count_unknowns, _) = split_at_pos(text, at);
        let buf = &text[..length];
        let rest = &text[length..];
        if count_unknowns == 0 {
            return (Cow::Borrowed(buf), Cow::Borrowed(rest));
        }

        let mut buf = buf.to_owned();
        buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));

        return (Cow::Owned(buf), Cow::Borrowed(rest));
    }
}

/// The function splits a string in the position and
/// returns a exact number of bytes before the position and in case of a split in an unicode grapheme
/// a width of a character which was tried to be splited in.
///
/// BE AWARE: pos is expected to be in bytes.
fn split_at_pos(s: &str, pos: usize) -> (usize, usize, usize) {
    let mut length = 0;
    let mut i = 0;
    for c in s.chars() {
        if i == pos {
            break;
        };

        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);

        // We cut the chars which takes more then 1 symbol to display,
        // in order to archive the necessary width.
        if i + c_width > pos {
            let count = pos - i;
            return (length, count, c.len_utf8());
        }

        i += c_width;
        length += c.len_utf8();
    }

    (length, 0, 0)
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
        assert_eq!(count_empty_lines("\n\nsome text\n\n\n"), (2, 3));
    }
}
