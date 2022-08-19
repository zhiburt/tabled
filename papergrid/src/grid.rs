//! The module contains a [`Grid`] structure.

use std::{
    borrow::Cow,
    cmp,
    fmt::{self, Display, Write},
};

use crate::{
    estimation::Estimate,
    records::Records,
    util::{get_lines, spplit_str_at, string_trim},
    width::{CfgWidthFunction, WidthFunc},
    AlignmentHorizontal, AlignmentVertical, Formatting, GridConfig, Indent, Padding, Position,
};

#[cfg(feature = "color")]
use crate::{AnsiColor, Color};

const DEFAULT_SPACE_CHAR: char = ' ';
const DEFAULT_BORDER_HORIZONTAL_CHAR: char = ' ';

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct Grid<'a, R, W, H> {
    config: &'a GridConfig,
    width: &'a W,
    height: &'a H,
    records: R,
}

impl<'a, R, W, H> Grid<'a, R, W, H> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: &'a GridConfig, width: &'a W, height: &'a H) -> Self {
        Grid {
            config,
            width,
            height,
            records,
        }
    }
}

impl<'a, R, W, H> fmt::Display for Grid<'a, R, W, H>
where
    R: Records,
    W: Estimate<R>,
    H: Estimate<R>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.records.count_rows() == 0 || self.records.count_columns() == 0 {
            return Ok(());
        }

        print_grid(f, self.config, &self.records, self.width, self.height)
    }
}

fn print_grid<R, W, H>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    width: &W,
    height: &H,
) -> fmt::Result
where
    W: Estimate<R>,
    H: Estimate<R>,
    R: Records,
{
    // // todo:
    // // Spans requires a few additional `if` compared to the flow without it.
    // // It also makes the logic a bit more
    // if cfg.has_column_spans() || cfg.has_row_spans() {
    // }

    if cfg.get_margin().top.size > 0 {
        let total_width = total_width(cfg, records, width);
        print_margin_top(f, cfg, total_width)?;
        f.write_char('\n')?;
    }

    let mut prev_empty_horizontal = false;
    for row in 0..records.count_rows() {
        let count_lines = height.get(row).unwrap();

        if has_horizontal(cfg, records, row) {
            if prev_empty_horizontal {
                '\n'.fmt(f)?;
            }

            print_margin_left(f, cfg)?;
            print_split_line(f, cfg, records, width, height, row)?;
            print_margin_right(f, cfg)?;

            if count_lines > 0 {
                '\n'.fmt(f)?;
                prev_empty_horizontal = false;
            } else {
                prev_empty_horizontal = true;
            }
        } else if count_lines > 0 && prev_empty_horizontal {
            '\n'.fmt(f)?;
            prev_empty_horizontal = false;
        }

        for i in 0..count_lines {
            print_margin_left(f, cfg)?;

            for col in 0..records.count_columns() {
                if !cfg.is_cell_covered_by_both_spans((row, col)) {
                    if cfg.is_cell_covered_by_row_span((row, col)) {
                        print_vertical_char(f, cfg, records, (row, col))?;

                        // means it's part of other a spanned cell
                        // so. we just need to use line from other cell.
                        let original_row = closest_visible_row(cfg, (row, col)).unwrap();

                        // considering that the content will be printed instead horizontal lines so we can skip some lines.
                        let mut skip_lines = (original_row..row)
                            .map(|i| height.get(i).unwrap())
                            .sum::<usize>();

                        skip_lines += (original_row..row)
                            .map(|row| has_horizontal(cfg, records, row) as usize)
                            .sum::<usize>();

                        let line = i + skip_lines;
                        print_cell_line(f, cfg, records, width, height, (original_row, col), line)?;
                    } else if !cfg.is_cell_covered_by_column_span((row, col)) {
                        print_vertical_char(f, cfg, records, (row, col))?;
                        print_cell_line(f, cfg, records, width, height, (row, col), i)?;
                    }
                }

                let is_last_column = col + 1 == records.count_columns();
                if is_last_column {
                    print_vertical_char(f, cfg, records, (row, col + 1))?;
                }
            }

            print_margin_right(f, cfg)?;

            let is_last_line = i + 1 == count_lines;
            let is_last_row = row + 1 == records.count_rows();
            if !(is_last_line && is_last_row) {
                '\n'.fmt(f)?;
            }
        }
    }

    if has_horizontal(cfg, records, records.count_rows()) {
        f.write_char('\n')?;
        print_margin_left(f, cfg)?;
        print_split_line(f, cfg, records, width, height, records.count_rows())?;
        print_margin_right(f, cfg)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;
        let total_width = total_width(cfg, records, width);
        print_margin_bottom(f, cfg, total_width)?;
    }

    Ok(())
}

fn print_cell_line<R, W, H>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    width: &W,
    height: &H,
    pos: Position,
    line: usize,
) -> fmt::Result
where
    R: Records,
    W: Estimate<R>,
    H: Estimate<R>,
{
    let width = grid_cell_width(cfg, records, width, pos);
    let height = grid_cell_height(cfg, records, height, pos);

    let mut cell_height = records.count_lines(pos);
    let formatting = *cfg.get_formatting(pos.into());
    if formatting.vertical_trim {
        cell_height -=
            count_empty_lines_at_start(&records, pos) + count_empty_lines_at_end(&records, pos);
    }

    #[cfg(feature = "color")]
    let padding_color = cfg.get_padding_color(pos.into());

    let padding = cfg.get_padding(pos.into());
    let alignment = cfg.get_alignment_vertical(pos.into());
    let indent = top_indent(*padding, *alignment, cell_height, height);
    if indent > line {
        return print_indent(
            f,
            padding.top.fill,
            width,
            #[cfg(feature = "color")]
            &padding_color.top,
        );
    }

    let mut index = line - indent;
    let cell_has_this_line = cell_height > index;
    if !cell_has_this_line {
        // happens when other cells have bigger height
        return print_indent(
            f,
            padding.bottom.fill,
            width,
            #[cfg(feature = "color")]
            &padding_color.bottom,
        );
    }

    if formatting.vertical_trim {
        let empty_lines = count_empty_lines_at_start(&records, pos);
        index += empty_lines;

        if index > records.count_lines(pos) {
            return print_indent(
                f,
                padding.top.fill,
                width,
                #[cfg(feature = "color")]
                &padding_color.top,
            );
        }
    }

    print_indent(
        f,
        padding.left.fill,
        padding.left.size,
        #[cfg(feature = "color")]
        &padding_color.left,
    )?;

    let width = width - padding.left.size - padding.right.size;
    let alignment = *cfg.get_alignment_horizontal(pos.into());
    let width_ctrl = CfgWidthFunction::from_cfg(cfg);
    print_line_aligned(
        f,
        &records,
        pos,
        index,
        alignment,
        formatting,
        width,
        cfg.get_tab_width(),
        &width_ctrl,
    )?;

    print_indent(
        f,
        padding.right.fill,
        padding.right.size,
        #[cfg(feature = "color")]
        &padding_color.right,
    )?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_line_aligned<R, W>(
    f: &mut fmt::Formatter<'_>,
    records: &R,
    pos: Position,
    index: usize,
    alignment: AlignmentHorizontal,
    formatting: Formatting,
    available_width: usize,
    tab_width: usize,
    width_ctrl: &W,
) -> Result<(), fmt::Error>
where
    R: Records,
    W: WidthFunc,
{
    let line = records.get_line(pos, index);
    let (line, line_width) = if formatting.horizontal_trim && !line.is_empty() {
        let line = string_trim(line);
        let width = width_ctrl.width(&line);
        (line, width)
    } else {
        let line = Cow::Borrowed(line);
        let width = records.get_line_width(pos, index, width_ctrl);
        (line, width)
    };

    if formatting.allow_lines_alignement {
        let (left, right) = calculate_indent(alignment, line_width, available_width);
        return print_text_formated(f, records, pos, &line, tab_width, left, right);
    }

    let cell_width = if formatting.horizontal_trim {
        (0..records.count_lines(pos))
            .map(|i| records.get_line(pos, i))
            .map(|line| width_ctrl.width(line.trim()))
            .max()
            .unwrap_or(0)
    } else {
        records.get_width(pos, width_ctrl)
    };

    let (left, right) = calculate_indent(alignment, cell_width, available_width);
    print_text_formated(f, records, pos, &line, tab_width, left, right)?;

    let rest_width = cell_width - line_width;
    repeat_char(f, DEFAULT_SPACE_CHAR, rest_width)?;

    Ok(())
}

#[allow(unused)]
fn print_text_formated<R>(
    f: &mut fmt::Formatter<'_>,
    records: &R,
    pos: Position,
    text: &str,
    tab_width: usize,
    left: usize,
    right: usize,
) -> fmt::Result
where
    R: Records,
{
    repeat_char(f, DEFAULT_SPACE_CHAR, left)?;

    #[cfg(feature = "color")]
    records.fmt_text_prefix(f, pos)?;

    print_text(f, text, tab_width)?;

    #[cfg(feature = "color")]
    records.fmt_text_suffix(f, pos)?;

    repeat_char(f, DEFAULT_SPACE_CHAR, right)?;

    Ok(())
}

fn print_text(f: &mut fmt::Formatter<'_>, text: &str, tab_width: usize) -> fmt::Result {
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

fn print_split_line<R, W, H>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    width_ctrl: &W,
    height_ctrl: &H,
    row: usize,
) -> fmt::Result
where
    W: Estimate<R>,
    H: Estimate<R>,
    R: Records,
{
    let mut override_text = cfg
        .get_split_line_text(row)
        .and_then(|text| get_lines(text).next())
        .map(|text| text.into_owned())
        .unwrap_or_default();

    #[cfg(feature = "color")]
    let mut used_color = None;

    for col in 0..records.count_columns() {
        if col == 0 {
            let left =
                cfg.get_intersection((row, col), (records.count_rows(), records.count_columns()));
            if let Some(c) = left {
                if !override_text.is_empty() {
                    let (c, rest) = spplit_str_at(&override_text, 1);
                    c.fmt(f)?;
                    override_text = rest.into_owned();
                } else {
                    #[cfg(feature = "color")]
                    {
                        let clr = cfg.get_intersection_color(
                            (row, col),
                            (records.count_rows(), records.count_columns()),
                        );
                        if let Some(clr) = clr {
                            clr.fmt_prefix(f)?;
                            used_color = Some(clr);
                        }
                    }

                    c.fmt(f)?;
                }
            }
        }

        let mut width = width_ctrl.get(col).unwrap();
        if cfg.is_cell_covered_by_both_spans((row, col)) {
            continue;
        }

        if !override_text.is_empty() {
            let width_ctrl = CfgWidthFunction::from_cfg(cfg);
            let text_width = width_ctrl.width(&override_text);
            let print_width = cmp::min(text_width, width);
            let (c, rest) = spplit_str_at(&override_text, print_width);
            c.fmt(f)?;
            override_text = rest.into_owned();

            width -= print_width;
        }

        let mut col = col;
        if cfg.is_cell_covered_by_row_span((row, col)) {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let original_row = closest_visible_row(cfg, (row, col)).unwrap();

            // considering that the content will be printed instead horizontal lines so we can skip some lines.
            let mut skip_lines = (original_row..row)
                .map(|i| height_ctrl.get(i).unwrap())
                .sum::<usize>();

            // skip horizontal lines
            if row > 0 {
                skip_lines += (original_row..row - 1)
                    .map(|row| cfg.has_horizontal(row, records.count_rows()) as usize)
                    .sum::<usize>();
            }

            let line = skip_lines;
            print_cell_line(
                f,
                cfg,
                records,
                width_ctrl,
                height_ctrl,
                (original_row, col),
                line,
            )?;

            // We need to use a correct right split char.
            if let Some(span) = cfg.get_column_span((original_row, col)) {
                col += span - 1;
            }
        } else {
            // general case
            let main = get_horizontal(cfg, records, (row, col));
            match main {
                Some(c) => {
                    #[cfg(feature = "color")]
                    {
                        prepare_coloring(
                            f,
                            get_horizontal_color(cfg, records, (row, col)),
                            &mut used_color,
                        )?;
                    }

                    repeat_char(f, *c, width)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }
        }

        let right = get_intersection(cfg, records, (row, col + 1));
        if let Some(c) = right {
            if !override_text.is_empty() {
                let (c, rest) = spplit_str_at(&override_text, 1);
                c.fmt(f)?;
                override_text = rest.into_owned();
            } else {
                #[cfg(feature = "color")]
                {
                    prepare_coloring(
                        f,
                        get_intersection_color(cfg, records, (row, col + 1)),
                        &mut used_color,
                    )?;
                }

                c.fmt(f)?;
            }
        }
    }

    #[cfg(feature = "color")]
    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

#[cfg(feature = "color")]
fn prepare_coloring<'a>(
    f: &mut fmt::Formatter<'_>,
    clr: Option<&'a AnsiColor>,
    used_color: &mut Option<&'a AnsiColor>,
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
        None => match used_color.take() {
            Some(clr) => clr.fmt_suffix(f)?,
            None => (),
        },
    }

    Ok(())
}

fn top_indent(
    padding: Padding,
    alignment: AlignmentVertical,
    cell_height: usize,
    height: usize,
) -> usize {
    let height = height - padding.top.size;
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

fn count_empty_lines_at_end<R>(records: R, pos: Position) -> usize
where
    R: Records,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .rev()
        .take_while(|l| l.trim().is_empty())
        .count()
}

fn count_empty_lines_at_start<R>(records: R, pos: Position) -> usize
where
    R: Records,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .take_while(|s| s.trim().is_empty())
        .count()
}

fn repeat_char(f: &mut fmt::Formatter<'_>, c: char, n: usize) -> fmt::Result {
    for _ in 0..n {
        c.fmt(f)?;
    }

    Ok(())
}

// only valid to call for stabilized widths.
fn total_width<R, W>(cfg: &GridConfig, records: &R, width: &W) -> usize
where
    W: Estimate<R>,
    R: Records,
{
    let content_width = width.total();
    let count_borders = cfg.count_vertical(records.count_columns());
    let margin = cfg.get_margin();

    content_width + count_borders + margin.left.size + margin.right.size
}

fn print_vertical_char<R>(
    f: &mut fmt::Formatter<'_>,
    cfg: &GridConfig,
    records: &R,
    pos: Position,
) -> fmt::Result
where
    R: Records,
{
    let left = get_vertical(cfg, records, pos);
    if let Some(c) = left {
        #[cfg(feature = "color")]
        {
            if let Some(clr) = get_vertical_color(cfg, records, pos) {
                clr.fmt_prefix(f)?;
                c.fmt(f)?;
                clr.fmt_suffix(f)?;
            } else {
                c.fmt(f)?;
            }
        }

        #[cfg(not(feature = "color"))]
        c.fmt(f)?;
    }

    Ok(())
}

fn print_margin_top(f: &mut fmt::Formatter<'_>, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().top,
        width,
        #[cfg(feature = "color")]
        &cfg.get_margin_color().top,
    )
}

fn print_margin_bottom(f: &mut fmt::Formatter<'_>, cfg: &GridConfig, width: usize) -> fmt::Result {
    print_indent_lines(
        f,
        &cfg.get_margin().bottom,
        width,
        #[cfg(feature = "color")]
        &cfg.get_margin_color().bottom,
    )
}

fn print_margin_left(f: &mut fmt::Formatter<'_>, cfg: &GridConfig) -> fmt::Result {
    print_indent(
        f,
        cfg.get_margin().left.fill,
        cfg.get_margin().left.size,
        #[cfg(feature = "color")]
        &cfg.get_margin_color().left,
    )
}

fn print_margin_right(f: &mut fmt::Formatter<'_>, cfg: &GridConfig) -> fmt::Result {
    print_indent(
        f,
        cfg.get_margin().right.fill,
        cfg.get_margin().right.size,
        #[cfg(feature = "color")]
        &cfg.get_margin_color().right,
    )
}

fn print_indent_lines(
    f: &mut fmt::Formatter<'_>,
    indent: &Indent,
    width: usize,
    #[cfg(feature = "color")] color: &AnsiColor,
) -> fmt::Result {
    for i in 0..indent.size {
        print_indent(
            f,
            indent.fill,
            width,
            #[cfg(feature = "color")]
            color,
        )?;

        if i + 1 != indent.size {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn print_indent(
    f: &mut fmt::Formatter<'_>,
    c: char,
    n: usize,
    #[cfg(feature = "color")] color: &AnsiColor,
) -> fmt::Result {
    #[cfg(feature = "color")]
    color.fmt_prefix(f)?;
    repeat_char(f, c, n)?;
    #[cfg(feature = "color")]
    color.fmt_suffix(f)?;

    Ok(())
}

fn grid_cell_width<R, W>(cfg: &GridConfig, records: &R, width: &W, pos: Position) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    match cfg.get_column_span(pos) {
        Some(span) => range_width(cfg, records, width, pos.1, pos.1 + span),
        None => width.get(pos.1).unwrap(),
    }
}

fn range_width<R, W>(cfg: &GridConfig, records: &R, width: &W, start: usize, end: usize) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    let count_borders = count_borders_in_range(cfg, start, end, records.count_columns());
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

fn grid_cell_height<R, H>(cfg: &GridConfig, records: &R, height: &H, pos: Position) -> usize
where
    R: Records,
    H: Estimate<R>,
{
    match cfg.get_row_span(pos) {
        Some(span) => range_height(cfg, records, height, pos.0, pos.0 + span),
        None => height.get(pos.0).unwrap(),
    }
}

fn range_height<R, H>(cfg: &GridConfig, records: &R, height: &H, start: usize, end: usize) -> usize
where
    R: Records,
    H: Estimate<R>,
{
    let count_borders = count_horizontal_borders_in_range(cfg, start, end, records.count_rows());
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

fn closest_visible_row(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
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

fn get_vertical<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&char>
where
    R: Records,
{
    cfg.get_vertical(pos, records.count_columns())
}

fn get_horizontal<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&char>
where
    R: Records,
{
    cfg.get_horizontal(pos, records.count_rows())
}

fn get_intersection<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&char>
where
    R: Records,
{
    cfg.get_intersection(pos, (records.count_rows(), records.count_columns()))
}

fn has_horizontal<R>(cfg: &GridConfig, records: R, row: usize) -> bool
where
    R: Records,
{
    cfg.has_horizontal(row, records.count_rows())
}

#[cfg(feature = "color")]
fn get_intersection_color<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&AnsiColor>
where
    R: Records,
{
    cfg.get_intersection_color(pos, (records.count_rows(), records.count_columns()))
}

#[cfg(feature = "color")]
fn get_vertical_color<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&AnsiColor>
where
    R: Records,
{
    cfg.get_vertical_color(pos, records.count_columns())
}

#[cfg(feature = "color")]
fn get_horizontal_color<R>(cfg: &GridConfig, records: R, pos: Position) -> Option<&AnsiColor>
where
    R: Records,
{
    cfg.get_horizontal_color(pos, records.count_rows())
}

#[cfg(test)]
mod tests {
    use crate::{records::empty::EmptyRecords, util::string_width};

    use super::*;

    #[test]
    fn horizontal_aligment_test() {
        use std::fmt;

        struct F<'a>(&'a str, AlignmentHorizontal, usize);

        impl fmt::Display for F<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let (left, right) = calculate_indent(self.1, string_width(self.0), self.2);
                print_text_formated(f, &EmptyRecords::default(), (0, 0), self.0, 4, left, right)
            }
        }

        assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
        assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
        assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");

        #[cfg(feature = "color")]
        {
            use owo_colors::OwoColorize;
            let text = "Colored Text".red().to_string();
            assert_eq!(
                F(&text, AlignmentHorizontal::Center, 15).to_string(),
                format!(" {}  ", text)
            );
        }
    }

    #[test]
    fn vertical_aligment_test() {
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
