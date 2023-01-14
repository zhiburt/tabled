//! The module contains a [`Grid`] structure.

use std::{
    borrow::Cow,
    cmp,
    collections::{BTreeMap, HashMap},
    fmt::{self, Formatter, Write},
};

use crate::{
    colors::{self, Colors},
    config::EntityMap,
    estimation::{Estimate, ExactEstimate},
    records::{RecordCell, Records},
    util::{get_lines, spplit_str_at, string_trim, string_width},
    width::{CfgWidthFunction, WidthFunc},
    AlignmentHorizontal, AlignmentVertical, AnsiColor, Color, Entity, Formatting, GridConfig,
    Indent, Offset, Padding, PaddingColor, Position,
};

const DEFAULT_SPACE_CHAR: char = ' ';
const DEFAULT_BORDER_HORIZONTAL_CHAR: char = ' ';

type DefaultColors = EntityMap<Option<AnsiColor<'static>>>;

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct Grid<'a, R, W, H, C = DefaultColors> {
    records: R,
    config: &'a GridConfig,
    width: &'a W,
    height: &'a H,
    colors: Option<C>,
}

impl<'a, R, W, H> Grid<'a, R, W, H, DefaultColors> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: &'a GridConfig, width: &'a W, height: &'a H) -> Self {
        Grid {
            config,
            width,
            height,
            records,
            colors: None,
        }
    }
}

impl<'a, R, W, H, C> Grid<'a, R, W, H, C> {
    pub fn set_colors<Colors: colors::Colors>(self, colors: Colors) -> Grid<'a, R, W, H, Colors> {
        Grid {
            config: self.config,
            width: self.width,
            height: self.height,
            records: self.records,
            colors: Some(colors),
        }
    }
}

impl<'a, R: Records, W: Estimate + ExactEstimate, H: Estimate, C: Colors> fmt::Display
    for Grid<'a, R, W, H, C>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.records.count_columns() == 0 {
            return Ok(());
        }

        print_grid(
            f,
            self.config,
            &self.records,
            self.width,
            self.height,
            self.colors.as_ref(),
        )
    }
}

fn print_grid<R: Records, W: Estimate + ExactEstimate, H: Estimate, C: Colors>(
    f: &mut Formatter<'_>,
    cfg: &GridConfig,
    records: R,
    width: &W,
    height: &H,
    colors: Option<&C>,
) -> fmt::Result {
    // spanned version is a bit more complex and 'supposedly' slower,
    // because spans are considered to be not a general case we are having 2 versions
    if cfg.has_column_spans() || cfg.has_row_spans() {
        print_grid_spanned(f, cfg, &records, width, height, colors)
    } else {
        print_grid_general(f, cfg, &records, width, height, colors)
    }
}

fn print_grid_general<R: Records, W: Estimate + ExactEstimate, H: Estimate, C: Colors>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    width: &W,
    height: &H,
    colors: Option<&C>,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let total_width = total_width(cfg, width, count_columns);
    let total_width_with_margin =
        total_width + cfg.get_margin().left.size + cfg.get_margin().right.size;

    let mut records_iter = records.iter_rows();
    let mut next_columns = records_iter.next();

    let mut row_columns = Vec::with_capacity(count_columns);

    let mut line = 0;

    let total_height = records
        .hint_rows()
        .and_then(|rows| total_height(cfg, height, rows));

    if cfg.get_margin().top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut row = 0;
    while let Some(columns) = next_columns {
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let height = height.get(row).unwrap();
        let count_rows = convert_count_rows(line + height, is_last_row);
        let has_horizontal = has_horizontal(cfg, row, count_rows);

        if row > 0 && (has_horizontal || height > 0) {
            f.write_char('\n')?;
        }

        if has_horizontal {
            let shape = (count_rows, count_columns);
            print_horizontal_line(f, cfg, line, total_height, width, row, total_width, shape)?;

            line += 1;

            if height > 0 {
                f.write_char('\n')?;
            }
        }

        match height {
            0 => {}
            1 => print_single_line_columns(
                f,
                columns,
                cfg,
                colors,
                width,
                row,
                line,
                total_height,
                (count_rows, count_columns),
            )?,
            _ => {
                print_multiline_columns(
                    f,
                    &mut row_columns,
                    columns,
                    cfg,
                    colors,
                    width,
                    height,
                    row,
                    line,
                    total_height,
                    (count_rows, count_columns),
                )?;
            }
        }

        line += height;
        row += 1;
    }

    if row > 0 {
        if has_horizontal(cfg, row, row) {
            f.write_char('\n')?;
            let shape = (row, count_columns);
            print_horizontal_line(f, cfg, line, total_height, width, row, total_width, shape)?;
        }

        if cfg.get_margin().bottom.size > 0 {
            f.write_char('\n')?;
            print_margin_bottom(f, cfg, total_width_with_margin)?;
        }
    }

    Ok(())
}

fn print_horizontal_line<W: Estimate>(
    f: &mut Formatter<'_>,
    cfg: &GridConfig,
    line: usize,
    total_height: Option<usize>,
    width: &W,
    row: usize,
    total_width: usize,
    shape: (usize, usize),
) -> Result<(), fmt::Error> {
    print_margin_left(f, cfg, line, total_height)?;
    print_split_line(f, cfg, width, row, total_width, shape.0, shape.1)?;
    print_margin_right(f, cfg, line, total_height)?;
    Ok(())
}

type CLines<'a, I, C> = CellLines<
    'a,
    <<<I as Iterator>::Item as RecordCell>::Lines as IntoIterator>::IntoIter,
    <C as Colors>::Color,
>;

fn print_multiline_columns<'a, I, W, C>(
    f: &mut Formatter<'_>,
    row_columns: &mut Vec<CLines<'a, I, C>>,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    width: &W,
    height: usize,
    row: usize,
    line: usize,
    total_height: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    I: Iterator,
    I::Item: RecordCell,
    W: Estimate,
    C: Colors,
{
    let width_ctrl = CfgWidthFunction::from_cfg(cfg);

    collect_columns(
        row_columns,
        columns,
        cfg,
        colors,
        width,
        &width_ctrl,
        height,
        row,
    );

    print_columns_lines(
        f,
        row_columns,
        height,
        cfg,
        &width_ctrl,
        line,
        row,
        total_height,
        shape,
    )?;

    row_columns.clear();

    Ok(())
}

fn print_single_line_columns<'a, I, W, C>(
    f: &mut Formatter<'_>,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    width: &W,
    row: usize,
    line: usize,
    total_height: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    I: Iterator,
    I::Item: RecordCell,
    W: Estimate,
    C: Colors,
{
    let width_ctrl = CfgWidthFunction::from_cfg(cfg);

    print_margin_left(f, cfg, line, total_height)?;

    for (col, cell) in columns.enumerate() {
        print_vertical_char(f, cfg, (row, col), line, shape.0, shape.1)?;

        let width = width.get(col).expect("guaranteed");
        let color = colors.and_then(|c| c.get_color((row, col)));
        print_single_line_column(f, cell, cfg, width, color, (row, col), &width_ctrl)?;
    }

    print_vertical_char(f, cfg, (row, shape.1), line, shape.0, shape.1)?;

    print_margin_right(f, cfg, line, total_height)?;

    Ok(())
}

fn print_single_line_column<C: RecordCell, Clr: Color, W: WidthFunc>(
    f: &mut Formatter<'_>,
    cell: C,
    cfg: &GridConfig,
    width: usize,
    color: Option<&Clr>,
    pos: Position,
    width_ctrl: &W,
) -> Result<(), fmt::Error> {
    let pos = pos.into();
    let pad = cfg.get_padding(pos);
    let pad_color = cfg.get_padding_color(pos);

    let formatting = cfg.get_formatting(pos);
    let text = cell.get_text();
    let text = text.as_ref();

    let (text, text_width) = if formatting.horizontal_trim && !text.is_empty() {
        let text = string_trim(text);
        let width = width_ctrl.width(&text);
        (text, width)
    } else {
        (Cow::Borrowed(text), cell.get_width(width_ctrl))
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

fn print_columns_lines<L, C>(
    f: &mut Formatter<'_>,
    columns: &mut [CellLines<'_, L, C>],
    height: usize,
    cfg: &GridConfig,
    width_ctrl: &CfgWidthFunction,
    line: usize,
    row: usize,
    total_height: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    L: Iterator,
    L::Item: AsRef<str>,
    C: Color,
{
    for i in 0..height {
        let line = line + i;

        print_margin_left(f, cfg, line, total_height)?;

        for (col, cell) in columns.iter_mut().enumerate() {
            print_vertical_char(f, cfg, (row, col), line, shape.0, shape.1)?;
            cell.display(f, width_ctrl, cfg.get_tab_width())?;
        }

        print_vertical_char(f, cfg, (row, shape.1), line, shape.0, shape.1)?;

        print_margin_right(f, cfg, line, total_height)?;

        if i + 1 != height {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn collect_columns<'a, I, W, C>(
    row_columns: &mut Vec<CLines<'a, I, C>>,
    columns: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    width: &W,
    width_ctrl: &CfgWidthFunction,
    height: usize,
    row: usize,
) where
    I: Iterator,
    I::Item: RecordCell,
    C: Colors,
    W: Estimate,
{
    let iter = columns.enumerate().map(|(col, cell)| {
        let width = width.get(col).expect("must be here");

        let pos = (row, col).into();
        CellLines::new(
            cell,
            width_ctrl,
            width,
            height,
            cfg.get_formatting(pos),
            cfg.get_padding(pos),
            cfg.get_padding_color(pos),
            *cfg.get_alignment_horizontal(pos),
            *cfg.get_alignment_vertical(pos),
            colors.and_then(|c| c.get_color((row, col))),
        )
    });

    row_columns.extend(iter);
}

fn print_split_line<W: Estimate>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    width_ctrl: &W,
    row: usize,
    total_width: usize,
    count_rows: usize,
    count_columns: usize,
) -> fmt::Result {
    let mut override_text = cfg
        .get_split_line_text(row)
        .and_then(|text| get_lines(text).next())
        .unwrap_or_default()
        .into_owned();
    let override_text_offset = cfg.get_split_line_offset(row).unwrap_or(Offset::Begin(0));
    let override_text_pos = offset_start_pos(override_text_offset, total_width);

    #[cfg(feature = "color")]
    let mut used_color = None;

    let mut i = 0;
    for col in 0..count_columns {
        if col == 0 {
            let left = cfg.get_intersection((row, col), (count_rows, count_columns));
            if let Some(c) = left {
                if i >= override_text_pos && !override_text.is_empty() {
                    let (c, rest) = spplit_str_at(&override_text, 1);
                    f.write_str(&c)?;
                    override_text = rest.into_owned();
                    if string_width(&override_text) == 0 {
                        override_text = String::new()
                    }
                } else {
                    #[cfg(feature = "color")]
                    {
                        let clr =
                            cfg.get_intersection_color((row, col), (count_rows, count_columns));
                        if let Some(clr) = clr {
                            clr.fmt_prefix(f)?;
                            used_color = Some(clr);
                        }
                    }

                    f.write_char(*c)?;
                    i += 1;
                }
            }
        }

        let mut width = width_ctrl.get(col).unwrap();

        // a situation when need to partially print split
        if i < override_text_pos && i + width >= override_text_pos {
            let available = override_text_pos - i;
            width -= available;
            i += available;
            let width = available;

            let main = get_horizontal(cfg, (row, col), count_rows);
            match main {
                Some(c) => {
                    #[cfg(feature = "color")]
                    {
                        prepare_coloring(
                            f,
                            get_horizontal_color(cfg, (row, col), count_rows),
                            &mut used_color,
                        )?;
                    }

                    print_horizontal_border(f, cfg, (row, col), width, *c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }
        }

        if i >= override_text_pos && !override_text.is_empty() {
            let width_ctrl = CfgWidthFunction::from_cfg(cfg);
            let text_width = width_ctrl.width(&override_text);
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
            let main = get_horizontal(cfg, (row, col), count_rows);
            match main {
                Some(c) => {
                    #[cfg(feature = "color")]
                    {
                        prepare_coloring(
                            f,
                            get_horizontal_color(cfg, (row, col), count_rows),
                            &mut used_color,
                        )?;
                    }

                    print_horizontal_border(f, cfg, (row, col), width, *c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }

            i += width;
        }

        let right = get_intersection(cfg, (row, col + 1), count_rows, count_columns);
        if let Some(c) = right {
            if i >= override_text_pos && !override_text.is_empty() {
                let (c, rest) = spplit_str_at(&override_text, 1);
                f.write_str(&c)?;
                override_text = rest.into_owned();
                if string_width(&override_text) == 0 {
                    override_text = String::new()
                }
            } else {
                #[cfg(feature = "color")]
                {
                    prepare_coloring(
                        f,
                        get_intersection_color(cfg, (row, col + 1), count_rows, count_columns),
                        &mut used_color,
                    )?;
                }

                f.write_char(*c)?;
                i += 1;
            }
        }
    }

    #[cfg(feature = "color")]
    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_grid_spanned<R: Records, W: Estimate + ExactEstimate, H: Estimate, C: Colors>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    width: &W,
    height: &H,
    colors: Option<&C>,
) -> fmt::Result {
    let count_columns = records.count_columns();

    let total_width = total_width(cfg, width, count_columns);
    let total_width_with_margin =
        total_width + cfg.get_margin().left.size + cfg.get_margin().right.size;

    let mut records_iter = records.iter_rows();
    let mut next_columns = records_iter.next();

    let mut row_columns = BTreeMap::new();

    let mut line = 0;

    let total_height = records
        .hint_rows()
        .and_then(|rows| total_height(cfg, height, rows));

    if cfg.get_margin().top.size > 0 {
        print_margin_top(f, cfg, total_width_with_margin)?;
        f.write_char('\n')?;
    }

    let mut row = 0;
    while let Some(columns) = next_columns {
        next_columns = records_iter.next();
        let is_last_row = next_columns.is_none();

        let row_height = height.get(row).unwrap();
        let count_rows = convert_count_rows(line + row_height, is_last_row);
        let has_horizontal = has_horizontal(cfg, row, count_rows);

        if row > 0 && (has_horizontal || row_height > 0) {
            f.write_char('\n')?;
        }

        if has_horizontal {
            let shape = (count_rows, count_columns);
            print_split_line_spanned(f, &mut row_columns, cfg, width, total_width, row, shape)?;

            line += 1;

            if row_height > 0 {
                f.write_char('\n')?;
            }
        }

        print_spanned_columns(
            f,
            &mut row_columns,
            columns,
            cfg,
            colors,
            width,
            height,
            row_height,
            row,
            line,
            total_height,
            (count_rows, count_columns),
        )?;

        line += row_height;
        row += 1;
    }

    if has_horizontal(cfg, 1, 2) {
        f.write_char('\n')?;
        let shape = (2, count_columns);
        print_horizontal_line(f, cfg, line, total_height, width, 1, total_width, shape)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;
        print_margin_bottom(f, cfg, total_width_with_margin)?;
    }

    Ok(())
}

fn print_split_line_spanned<W, L, C>(
    f: &mut fmt::Formatter<'_>,
    columns: &mut BTreeMap<usize, (CellLines<'_, L, C>, usize, usize)>,
    cfg: &GridConfig,
    width_ctrl: &W,
    total_width: usize,
    row: usize,
    shape: (usize, usize),
) -> fmt::Result
where
    W: Estimate,
    L: Iterator,
    L::Item: AsRef<str>,
    C: Color,
{
    let mut override_text = cfg
        .get_split_line_text(row)
        .and_then(|text| get_lines(text).next())
        .unwrap_or_default()
        .into_owned();
    let override_text_offset = cfg.get_split_line_offset(row).unwrap_or(Offset::Begin(0));
    let override_text_pos = offset_start_pos(override_text_offset, total_width);

    #[cfg(feature = "color")]
    let mut used_color = None;

    let mut i = 0;
    for col in 0..shape.1 {
        if col == 0 {
            let left = cfg.get_intersection((row, col), shape);
            if let Some(c) = left {
                if i >= override_text_pos && !override_text.is_empty() {
                    let (c, rest) = spplit_str_at(&override_text, 1);
                    f.write_str(&c)?;
                    override_text = rest.into_owned();
                    if string_width(&override_text) == 0 {
                        override_text = String::new()
                    }
                } else {
                    #[cfg(feature = "color")]
                    {
                        let clr = cfg.get_intersection_color((row, col), shape);
                        if let Some(clr) = clr {
                            clr.fmt_prefix(f)?;
                            used_color = Some(clr);
                        }
                    }

                    f.write_char(*c)?;
                    i += 1;
                }
            }
        }

        if cfg.is_cell_covered_by_both_spans((row, col), shape) {
            continue;
        }

        let is_spanned_split_line_part = cfg.is_cell_covered_by_row_span((row, col), shape);

        let mut width = width_ctrl.get(col).unwrap();
        let mut col = col;
        if is_spanned_split_line_part {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let (cell, _, _) = columns.get_mut(&col).unwrap();
            cell.display(f, &CfgWidthFunction::from_cfg(cfg), cfg.get_tab_width())?;

            // We need to use a correct right split char.
            let original_row = closest_visible_row(cfg, (row, col), shape).unwrap();
            if let Some(span) = cfg.get_column_span((original_row, col), shape) {
                col += span - 1;
            }
        } else if width > 0 {
            // a situation when need to partially print split
            if i < override_text_pos && i + width >= override_text_pos {
                let available = override_text_pos - i;
                width -= available;
                i += available;
                let width = available;

                let main = get_horizontal(cfg, (row, col), shape.0);
                match main {
                    Some(c) => {
                        #[cfg(feature = "color")]
                        {
                            let clr = get_horizontal_color(cfg, (row, col), shape.0);
                            prepare_coloring(f, clr, &mut used_color)?;
                        }

                        print_horizontal_border(f, cfg, (row, col), width, *c)?;
                    }
                    None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
                }
            }

            if i >= override_text_pos && !override_text.is_empty() {
                let width_ctrl = CfgWidthFunction::from_cfg(cfg);
                let text_width = width_ctrl.width(&override_text);
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
            let main = get_horizontal(cfg, (row, col), shape.0);
            match main {
                Some(c) => {
                    #[cfg(feature = "color")]
                    {
                        let clr = get_horizontal_color(cfg, (row, col), shape.0);
                        prepare_coloring(f, clr, &mut used_color)?;
                    }

                    print_horizontal_border(f, cfg, (row, col), width, *c)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }

            i += width;
        }

        let right = get_intersection(cfg, (row, col + 1), shape.0, shape.1);
        if let Some(c) = right {
            if i >= override_text_pos && !override_text.is_empty() {
                let (c, rest) = spplit_str_at(&override_text, 1);
                f.write_str(&c)?;
                override_text = rest.into_owned();
                if string_width(&override_text) == 0 {
                    override_text = String::new()
                }
            } else {
                #[cfg(feature = "color")]
                {
                    let clr = get_intersection_color(cfg, (row, col + 1), shape.0, shape.1);
                    prepare_coloring(f, clr, &mut used_color)?;
                }

                f.write_char(*c)?;
                i += 1;
            }
        }
    }

    #[cfg(feature = "color")]
    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

fn print_spanned_columns<'a, I, W, H, C>(
    f: &mut Formatter<'_>,
    columns: &mut BTreeMap<usize, (CLines<'a, I, C>, usize, usize)>,
    iter: I,
    cfg: &'a GridConfig,
    colors: Option<&'a C>,
    width: &W,
    height: &H,
    this_height: usize,
    row: usize,
    line: usize,
    total_height: Option<usize>,
    shape: (usize, usize),
) -> Result<(), fmt::Error>
where
    I: Iterator,
    I::Item: RecordCell,
    W: Estimate,
    H: Estimate,
    C: Colors,
{
    let width_ctrl = CfgWidthFunction::from_cfg(cfg);

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

        let colspan = cfg.get_column_span((row, col), shape).unwrap_or(1);
        skip = colspan - 1;
        let width = if colspan > 1 {
            range_width(cfg, width, col, col + colspan, shape.1)
        } else {
            width.get(col).unwrap()
        };

        let rowspan = cfg.get_row_span((row, col), shape).unwrap_or(1);
        let height = if rowspan > 1 {
            range_height(cfg, height, row, row + rowspan, shape.0)
        } else {
            this_height
        };

        let pos = (row, col).into();
        let cell = CellLines::new(
            cell,
            &width_ctrl,
            width,
            height,
            cfg.get_formatting(pos),
            cfg.get_padding(pos),
            cfg.get_padding_color(pos),
            *cfg.get_alignment_horizontal(pos),
            *cfg.get_alignment_vertical(pos),
            colors.and_then(|c| c.get_color((row, col))),
        );

        columns.insert(col, (cell, rowspan, colspan));
    }

    for i in 0..this_height {
        let line = line + i;

        print_margin_left(f, cfg, line, total_height)?;

        for (&col, (cell, _, _)) in columns.iter_mut() {
            print_vertical_char(f, cfg, (row, col), line, shape.0, shape.1)?;
            cell.display(f, &width_ctrl, cfg.get_tab_width())?;
        }

        print_vertical_char(f, cfg, (row, shape.1), line, shape.0, shape.1)?;

        print_margin_right(f, cfg, line, total_height)?;

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

mod print_spanned {
    use super::*;

    pub(super) fn print_grid<R, W, H>(
        f: &mut fmt::Formatter<'_>,
        cfg: &GridConfig,
        records: &R,
        width: &W,
        height: &H,
    ) -> fmt::Result
    where
        W: Estimate,
        H: Estimate,
        R: Records,
    {
        // let count_columns = records.count_columns();

        // let total_width = total_width(cfg, width, count_columns);
        // let total_width_with_margin =
        //     total_width + cfg.get_margin().left.size + cfg.get_margin().right.size;

        // if cfg.get_margin().top.size > 0 {
        //     print_margin_top(f, cfg, total_width_with_margin)?;
        //     f.write_char('\n')?;
        // }

        // let mut table_line = 0;
        // let mut prev_empty_horizontal = false;

        // let mut records_iter = records.iter_rows();
        // let mut next_columns = records_iter.next();

        // let mut row_columns = Vec::with_capacity(count_columns);

        // for row in 0.. {
        //     let columns = match next_columns {
        //         Some(columns) => columns,
        //         None => break,
        //     };

        //     next_columns = records_iter.next();

        //     let is_last_row = next_columns.is_none();
        //     let count_rows = convert_count_rows(row, is_last_row);

        //     let row_height = height.get(row).unwrap();

        //     let total_height = total_height(cfg, height, row + 1);

        //     if has_horizontal(cfg, row, count_rows) {
        //         if prev_empty_horizontal {
        //             f.write_char('\n')?;
        //         }

        //         print_margin_left(f, cfg, table_line, total_height)?;
        //         print_split_line(f, cfg, width, height, total_width, row, count_rows, count_columns, )?;
        //         print_margin_right(f, cfg, table_line, total_height)?;

        //         if count_lines > 0 {
        //             f.write_char('\n')?;
        //             prev_empty_horizontal = false;
        //         } else {
        //             prev_empty_horizontal = true;
        //         }

        //         table_line += 1;
        //     } else if count_lines > 0 && prev_empty_horizontal {
        //         f.write_char('\n')?;
        //         prev_empty_horizontal = false;
        //     }

        //     for i in 0..count_lines {
        //         print_margin_left(f, cfg, table_line, total_height)?;

        //         for col in 0..records.count_columns() {
        //             if !cfg.is_cell_covered_by_both_spans((row, col), shape) {
        //                 if cfg.is_cell_covered_by_row_span((row, col), shape) {
        //                     print_vertical_char(f, cfg, records, (row, col), i, count_lines)?;

        //                     // means it's part of other a spanned cell
        //                     // so. we just need to use line from other cell.
        //                     let original_row = closest_visible_row(cfg, (row, col), shape).unwrap();

        //                     // considering that the content will be printed instead horizontal lines so we can skip some lines.
        //                     let mut skip_lines = (original_row..row)
        //                         .map(|i| height.get(i).unwrap())
        //                         .sum::<usize>();

        //                     skip_lines += (original_row + 1..=row)
        //                         .map(|row| has_horizontal(cfg, records, row) as usize)
        //                         .sum::<usize>();

        //                     let line = i + skip_lines;
        //                     print_cell_line(
        //                         f,
        //                         cfg,
        //                         records,
        //                         width,
        //                         height,
        //                         (original_row, col),
        //                         line,
        //                     )?;
        //                 } else if !cfg.is_cell_covered_by_column_span((row, col), shape) {
        //                     print_vertical_char(f, cfg, records, (row, col), i, count_lines)?;
        //                     print_cell_line(f, cfg, records, width, height, (row, col), i)?;
        //                 }
        //             }

        //             let is_last_column = col + 1 == records.count_columns();
        //             if is_last_column {
        //                 print_vertical_char(f, cfg, records, (row, col + 1), i, count_lines)?;
        //             }
        //         }

        //         print_margin_right(f, cfg, table_line, total_height)?;

        //         let is_last_line = i + 1 == count_lines;
        //         let is_last_row = row + 1 == records.count_rows();
        //         if !(is_last_line && is_last_row) {
        //             f.write_char('\n')?;
        //         }

        //         table_line += 1;
        //     }
        // }

        // if has_horizontal(cfg, records, records.count_rows()) {
        //     f.write_char('\n')?;
        //     print_margin_left(f, cfg, table_line, total_height)?;
        //     let row = records.count_rows();
        //     print_split_line(f, cfg, records, width, height, row, total_width)?;
        //     print_margin_right(f, cfg, table_line, total_height)?;
        // }

        // if cfg.get_margin().bottom.size > 0 {
        //     f.write_char('\n')?;
        //     print_margin_bottom(f, cfg, total_width_with_margin)?;
        // }

        Ok(())
    }

    // fn print_split_line<R, C, W, H>(
    //     f: &mut fmt::Formatter<'_>,
    //     cfg: &GridConfig,
    //     width_ctrl: &W,
    //     height_ctrl: &H,
    //     total_width: usize,
    //     row: usize,
    //     count_rows: usize,
    //     count_columns: usize,
    //     cell: &C,
    // ) -> fmt::Result
    // where
    //     W: Estimate,
    //     H: Estimate,
    //     C: RecordCell,
    // {
    //     let shape = (count_rows, count_columns);

    //     let mut override_text = cfg
    //         .get_split_line_text(row)
    //         .and_then(|text| get_lines(text).next())
    //         .unwrap_or_default()
    //         .into_owned();
    //     let override_text_offset = cfg.get_split_line_offset(row).unwrap_or(Offset::Begin(0));
    //     let override_text_pos = offset_start_pos(override_text_offset, total_width);

    //     #[cfg(feature = "color")]
    //     let mut used_color = None;

    //     let mut i = 0;
    //     for col in 0..count_columns {
    //         if col == 0 {
    //             let left = cfg.get_intersection((row, col), shape);
    //             if let Some(c) = left {
    //                 if i >= override_text_pos && !override_text.is_empty() {
    //                     let (c, rest) = spplit_str_at(&override_text, 1);
    //                     f.write_str(&c)?;
    //                     override_text = rest.into_owned();
    //                     if string_width(&override_text) == 0 {
    //                         override_text = String::new()
    //                     }
    //                 } else {
    //                     #[cfg(feature = "color")]
    //                     {
    //                         let clr = cfg.get_intersection_color((row, col), shape);
    //                         if let Some(clr) = clr {
    //                             clr.fmt_prefix(f)?;
    //                             used_color = Some(clr);
    //                         }
    //                     }

    //                     f.write_char(*c)?;
    //                     i += 1;
    //                 }
    //             }
    //         }

    //         if cfg.is_cell_covered_by_both_spans((row, col), shape) {
    //             continue;
    //         }

    //         let is_spanned_split_line_part = cfg.is_cell_covered_by_row_span((row, col), shape);

    //         let mut width = width_ctrl.get(col).unwrap();
    //         let mut col = col;
    //         if is_spanned_split_line_part {
    //             // means it's part of other a spanned cell
    //             // so. we just need to use line from other cell.

    //             let original_row = closest_visible_row(cfg, (row, col), shape).unwrap();

    //             // considering that the content will be printed instead horizontal lines so we can skip some lines.
    //             let mut skip_lines = (original_row..row)
    //                 .map(|i| height_ctrl.get(i).unwrap())
    //                 .sum::<usize>();

    //             // skip horizontal lines
    //             if row > 0 {
    //                 skip_lines += (original_row..row - 1)
    //                     .map(|row| cfg.has_horizontal(row + 1, count_rows) as usize)
    //                     .sum::<usize>();
    //             }

    //             let line = skip_lines;
    //             let pos = (original_row, col);
    //             print_cell_line(
    //                 f,
    //                 cfg,
    //                 width_ctrl,
    //                 height_ctrl,
    //                 pos,
    //                 line,
    //                 count_rows,
    //                 count_columns,
    //                 cell,
    //             )?;

    //             // We need to use a correct right split char.
    //             if let Some(span) = cfg.get_column_span((original_row, col), shape) {
    //                 col += span - 1;
    //             }
    //         } else if width > 0 {
    //             // a situation when need to partially print split
    //             if i < override_text_pos && i + width >= override_text_pos {
    //                 let available = override_text_pos - i;
    //                 width -= available;
    //                 i += available;
    //                 let width = available;

    //                 let main = get_horizontal(cfg, (row, col), count_rows);
    //                 match main {
    //                     Some(c) => {
    //                         #[cfg(feature = "color")]
    //                         {
    //                             let clr = get_horizontal_color(cfg, (row, col), count_rows);
    //                             prepare_coloring(f, clr, &mut used_color)?;
    //                         }

    //                         print_horizontal_border(f, cfg, (row, col), width, *c)?;
    //                     }
    //                     None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
    //                 }
    //             }

    //             if i >= override_text_pos && !override_text.is_empty() {
    //                 let width_ctrl = CfgWidthFunction::from_cfg(cfg);
    //                 let text_width = width_ctrl.width(&override_text);
    //                 let print_width = cmp::min(text_width, width);
    //                 let (c, rest) = spplit_str_at(&override_text, print_width);
    //                 f.write_str(&c)?;
    //                 override_text = rest.into_owned();
    //                 if string_width(&override_text) == 0 {
    //                     override_text = String::new()
    //                 }

    //                 width -= print_width;
    //             }

    //             // general case
    //             let main = get_horizontal(cfg, (row, col), count_rows);
    //             match main {
    //                 Some(c) => {
    //                     #[cfg(feature = "color")]
    //                     {
    //                         let clr = get_horizontal_color(cfg, (row, col), count_rows);
    //                         prepare_coloring(f, clr, &mut used_color)?;
    //                     }

    //                     print_horizontal_border(f, cfg, (row, col), width, *c)?;
    //                 }
    //                 None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
    //             }

    //             i += width;
    //         }

    //         let right = get_intersection(cfg, (row, col + 1), count_rows, count_columns);
    //         if let Some(c) = right {
    //             if i >= override_text_pos && !override_text.is_empty() {
    //                 let (c, rest) = spplit_str_at(&override_text, 1);
    //                 f.write_str(&c)?;
    //                 override_text = rest.into_owned();
    //                 if string_width(&override_text) == 0 {
    //                     override_text = String::new()
    //                 }
    //             } else {
    //                 #[cfg(feature = "color")]
    //                 {
    //                     let clr =
    //                         get_intersection_color(cfg, (row, col + 1), count_rows, count_columns);
    //                     prepare_coloring(f, clr, &mut used_color)?;
    //                 }

    //                 f.write_char(*c)?;
    //                 i += 1;
    //             }
    //         }
    //     }

    //     #[cfg(feature = "color")]
    //     if let Some(clr) = used_color.take() {
    //         clr.fmt_suffix(f)?;
    //     }

    //     Ok(())
    // }

    // fn print_cell_line<R, C, W, H>(
    //     f: &mut fmt::Formatter<'_>,
    //     cfg: &GridConfig,
    //     width: &W,
    //     height: &H,
    //     pos: Position,
    //     line: usize,
    //     count_rows: usize,
    //     count_columns: usize,
    //     cell: &C,
    // ) -> fmt::Result
    // where
    //     C: RecordCell,
    //     W: Estimate,
    //     H: Estimate,
    // {
    //     let width = grid_cell_width(cfg, width, pos, count_rows, count_columns);
    //     let height = grid_cell_height(cfg, height, pos, count_rows, count_columns);
    //     super::print_cell_line(f, cfg, width, height, pos, line, cell)
    // }
}

fn print_horizontal_border(
    f: &mut Formatter<'_>,
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

struct CellLines<'a, L, C> {
    lines: L,
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
    fn new<'a, C: RecordCell, W: WidthFunc, Clr: Color>(
        cell: C,
        width_ctrl: &W,
        maxwidth: usize,
        height: usize,
        formatting: &'a Formatting,
        padding: &'a Padding,
        padding_color: &'a PaddingColor<'static>,
        alignmenth: AlignmentHorizontal,
        alignmentv: AlignmentVertical,
        color: Option<&'a Clr>,
    ) -> CellLines<'a, <C::Lines as IntoIterator>::IntoIter, Clr> {
        let (cell_height, vindent) = get_top_bottom_skip(formatting, &cell);
        let top_indent = top_indent(padding, alignmentv, cell_height, height);
        let width = maxwidth - padding.left.size - padding.right.size;
        let indent = get_left_right_indent(&cell, width_ctrl, formatting, alignmenth, width);

        let mut lines = cell.get_lines().into_iter();
        if let Some((top, _)) = vindent {
            for _ in 0..top {
                let s = lines.next();
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

impl<L, T, C> CellLines<'_, L, C>
where
    L: Iterator<Item = T>,
    T: AsRef<str>,
    C: Color,
{
    fn display<W: WidthFunc>(
        &mut self,
        f: &mut Formatter<'_>,
        width_ctrl: &W,
        tab_width: usize,
    ) -> fmt::Result {
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

        let line = match self.lines.next() {
            Some(line) => line,
            None => return print_indent(f, pad.bottom.fill, self.maxwidth, &pad_color.bottom),
        };

        let available_width = self.maxwidth - pad.left.size - pad.right.size;

        let line = if formatting.horizontal_trim && !line.as_ref().is_empty() {
            string_trim(line.as_ref())
        } else {
            Cow::Borrowed(line.as_ref())
        };

        let line_width = width_ctrl.width(&line);

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

fn get_top_bottom_skip<C: RecordCell>(
    format: &Formatting,
    cell: &C,
) -> (usize, Option<(usize, usize)>) {
    if format.vertical_trim {
        let (top, bottom, pos) = count_empty_lines(cell);
        (cell.count_lines() - bottom - top, Some((top, pos)))
    } else {
        (cell.count_lines(), None)
    }
}

fn get_left_right_indent<C: RecordCell, W: WidthFunc>(
    cell: &C,
    width_ctrl: &W,
    formatting: &Formatting,
    alignment: AlignmentHorizontal,
    maxwidth: usize,
) -> Option<usize> {
    if formatting.allow_lines_alignment {
        return None;
    }

    let width = if formatting.horizontal_trim {
        cell.get_lines()
            .into_iter()
            .map(|line| width_ctrl.width(line.as_ref().trim()))
            .max()
            .unwrap_or(0)
    } else {
        cell.get_width(width_ctrl)
    };

    let (left, _) = calculate_indent(alignment, width, maxwidth);

    Some(left)
}

fn print_text(f: &mut Formatter<'_>, s: &str, tab: usize, clr: Option<impl Color>) -> fmt::Result {
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

fn print_str(f: &mut Formatter<'_>, text: &str, tab_width: usize) -> fmt::Result {
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

#[cfg(feature = "color")]
fn prepare_coloring<'a>(
    f: &mut Formatter<'_>,
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

fn count_empty_lines<C: RecordCell>(cell: C) -> (usize, usize, usize) {
    let mut top = 0;
    let mut bottom = 0;
    let mut top_check = true;
    let mut bottom_pos = 0;

    for (i, line) in cell.get_lines().into_iter().enumerate() {
        let is_empty = line.as_ref().trim().is_empty();
        if top_check {
            if is_empty {
                top += 1;
            } else {
                top_check = false;
            }
        }

        if is_empty {
            bottom += 1;
        } else {
            bottom = 0;
            bottom_pos = i;
        }
    }

    (top, bottom, bottom_pos)
}

fn repeat_char(f: &mut Formatter<'_>, c: char, n: usize) -> fmt::Result {
    for _ in 0..n {
        f.write_char(c)?;
    }

    Ok(())
}

// only valid to call for stabilized widths.
fn total_width<W: Estimate>(cfg: &GridConfig, width: &W, count_columns: usize) -> usize {
    width.total().expect("must be here") + cfg.count_vertical(count_columns)
}

fn total_height<H: Estimate>(cfg: &GridConfig, height: &H, count_rows: usize) -> Option<usize> {
    height
        .total()
        .map(|total| total + cfg.count_horizontal(count_rows))
}

fn print_vertical_char(
    f: &mut Formatter<'_>,
    cfg: &GridConfig,
    pos: Position,
    line_index: usize,
    count_lines: usize,
    count_columns: usize,
) -> fmt::Result {
    let symbol = match get_vertical(cfg, pos, count_columns) {
        Some(c) => *c,
        None => return Ok(()),
    };

    let symbol = cfg
        .is_overridden_vertical(pos)
        .then(|| cfg.lookup_overridden_vertical(pos, line_index, count_lines))
        .flatten()
        .unwrap_or(symbol);

    #[cfg(feature = "color")]
    {
        if let Some(clr) = get_vertical_color(cfg, pos, count_columns) {
            clr.fmt_prefix(f)?;
            f.write_char(symbol)?;
            clr.fmt_suffix(f)?;
        } else {
            f.write_char(symbol)?;
        }
    }

    #[cfg(not(feature = "color"))]
    f.write_char(symbol)?;

    Ok(())
}

fn print_margin_top(f: &mut Formatter<'_>, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().top,
        &cfg.get_margin_offset().top,
        &cfg.get_margin_color().top,
        width,
    )
}

fn print_margin_bottom(f: &mut Formatter<'_>, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().bottom,
        &cfg.get_margin_offset().bottom,
        &cfg.get_margin_color().bottom,
        width,
    )
}

fn print_margin_left(
    f: &mut Formatter<'_>,
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

fn print_margin_right(
    f: &mut Formatter<'_>,
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

fn print_margin_vertical(
    f: &mut Formatter<'_>,
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
                if line <= pos {
                    print_indent(f, indent.fill, indent.size, color)?;
                } else {
                    print_indent(f, ' ', indent.size, &AnsiColor::default())?;
                }
            }
        }
    }

    Ok(())
}

fn print_indent_lines(
    f: &mut Formatter<'_>,
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

fn print_indent(f: &mut Formatter<'_>, c: char, n: usize, color: &AnsiColor<'_>) -> fmt::Result {
    color.fmt_prefix(f)?;
    repeat_char(f, c, n)?;
    color.fmt_suffix(f)?;

    Ok(())
}

fn grid_cell_width<W: Estimate>(
    cfg: &GridConfig,
    width: &W,
    pos: Position,
    shape: (usize, usize),
) -> usize {
    match cfg.get_column_span(pos, shape) {
        Some(span) => range_width(cfg, width, pos.1, pos.1 + span, shape.1),
        None => width.get(pos.1).unwrap(),
    }
}

fn range_width<W: Estimate>(
    cfg: &GridConfig,
    width: &W,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    let count_borders = count_borders_in_range(cfg, start, end, count_columns);
    let range_width = (start..end)
        .map(|col| width.get(col).unwrap())
        .sum::<usize>();
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

fn grid_cell_height<H: Estimate>(
    cfg: &GridConfig,
    height: &H,
    pos: Position,
    count_rows: usize,
    count_columns: usize,
) -> usize {
    match cfg.get_row_span(pos, (count_rows, count_columns)) {
        Some(span) => range_height(cfg, height, pos.0, pos.0 + span, count_rows),
        None => height.get(pos.0).unwrap(),
    }
}

fn range_height<H: Estimate>(
    cfg: &GridConfig,
    height: &H,
    start: usize,
    end: usize,
    count_rows: usize,
) -> usize {
    let count_borders = count_horizontal_borders_in_range(cfg, start, end, count_rows);
    let range_width = (start..end)
        .map(|col| height.get(col).unwrap())
        .sum::<usize>();

    count_borders + range_width
}

fn count_horizontal_borders_in_range(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_rows: usize,
) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_horizontal(i, count_rows))
        .count()
}

fn closest_visible_row(
    cfg: &GridConfig,
    mut pos: Position,
    shape: (usize, usize),
) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos, shape) {
            return Some(pos.0);
        }

        if pos.0 == 0 {
            return None;
        }

        pos.0 -= 1;
    }
}

fn get_vertical(cfg: &GridConfig, pos: Position, count_columns: usize) -> Option<&char> {
    cfg.get_vertical(pos, count_columns)
}

fn get_horizontal(cfg: &GridConfig, pos: Position, count_rows: usize) -> Option<&char> {
    cfg.get_horizontal(pos, count_rows)
}

fn get_intersection(
    cfg: &GridConfig,
    pos: Position,
    count_rows: usize,
    count_columns: usize,
) -> Option<&char> {
    cfg.get_intersection(pos, (count_rows, count_columns))
}

fn has_horizontal(cfg: &GridConfig, row: usize, count_rows: usize) -> bool {
    cfg.has_horizontal(row, count_rows)
}

#[cfg(feature = "color")]
fn get_intersection_color(
    cfg: &GridConfig,
    pos: Position,
    count_rows: usize,
    count_columns: usize,
) -> Option<&AnsiColor<'_>> {
    cfg.get_intersection_color(pos, (count_rows, count_columns))
}

#[cfg(feature = "color")]
fn get_vertical_color(
    cfg: &GridConfig,
    pos: Position,
    count_columns: usize,
) -> Option<&AnsiColor<'_>> {
    cfg.get_vertical_color(pos, count_columns)
}

#[cfg(feature = "color")]
fn get_horizontal_color(
    cfg: &GridConfig,
    pos: Position,
    count_rows: usize,
) -> Option<&AnsiColor<'_>> {
    cfg.get_horizontal_color(pos, count_rows)
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
        usize::MAX
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
    //         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
}
