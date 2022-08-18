//! The module contains a [`Grid`] structure.

use std::{
    borrow::Cow,
    cmp,
    fmt::{self, Display, Write},
};

use crate::{
    estimation::Estimate,
    records::Records,
    util::{cut_str, string_trim, string_width},
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

impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
{
    /// This function returns an amount of rows on the grid
    fn count_rows(&self) -> usize {
        self.records.count_rows()
    }

    /// This function returns an amount of columns on the grid
    fn count_columns(&self) -> usize {
        self.records.count_columns()
    }

    fn get_vertical(&self, pos: Position) -> Option<&char> {
        self.config.get_vertical(pos, self.count_columns())
    }

    fn get_horizontal(&self, pos: Position) -> Option<&char> {
        self.config.get_horizontal(pos, self.count_rows())
    }

    fn get_intersection(&self, pos: Position) -> Option<&char> {
        self.config
            .get_intersection(pos, (self.count_rows(), self.count_columns()))
    }

    fn has_horizontal(&self, row: usize) -> bool {
        self.config.has_horizontal(row, self.count_rows())
    }
}

impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
    W: Estimate<R>,
{
    /// Returns a total width of table, including split lines.
    fn total_width(&self) -> usize {
        if self.count_rows() == 0 || self.count_columns() == 0 {
            return 0;
        }

        total_width(self)
    }
}

#[cfg(feature = "color")]
impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
{
    fn get_intersection_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config
            .get_intersection_color(pos, (self.count_rows(), self.count_columns()))
    }

    fn get_horizontal_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config.get_horizontal_color(pos, self.count_rows())
    }

    fn get_vertical_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config.get_vertical_color(pos, self.count_columns())
    }
}

impl<'a, R, W, H> fmt::Display for Grid<'a, R, W, H>
where
    R: Records,
    W: Estimate<R>,
    H: Estimate<R>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count_rows() == 0 || self.count_columns() == 0 {
            return Ok(());
        }

        print_grid(self, f)
    }
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

fn indent_from_top(alignment: AlignmentVertical, available: usize, real: usize) -> usize {
    match alignment {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - real,
        AlignmentVertical::Center => (available - real) / 2,
    }
}

fn print_cell_line<R, W, H>(
    f: &mut fmt::Formatter<'_>,
    grid: &Grid<'_, R, W, H>,
    pos: Position,
    line: usize,
) -> fmt::Result
where
    R: Records,
    W: Estimate<R>,
    H: Estimate<R>,
{
    let width = grid_cell_width(grid, pos);
    let height = grid_cell_height(grid, pos);

    let width_ctrl = CfgWidthFunction::from_cfg(grid.config);

    let mut cell_height = grid.records.count_lines(pos);
    let formatting = *grid.config.get_formatting(pos.into());
    if formatting.vertical_trim {
        cell_height -= count_empty_lines_at_start(&grid.records, pos)
            + count_empty_lines_at_end(&grid.records, pos);
    }

    #[cfg(feature = "color")]
    let padding_color = grid.config.get_padding_color(pos.into());

    let padding = grid.config.get_padding(pos.into());
    let alignment = grid.config.get_alignment_vertical(pos.into());
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
        let empty_lines = count_empty_lines_at_start(&grid.records, pos);
        index += empty_lines;

        if index > grid.records.count_lines(pos) {
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
    let alignment = *grid.config.get_alignment_horizontal(pos.into());
    print_line_aligned(
        f,
        &grid.records,
        pos,
        index,
        alignment,
        formatting,
        width,
        grid.config.get_tab_width(),
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
fn total_width<R, W, H>(grid: &Grid<'_, R, W, H>) -> usize
where
    W: Estimate<R>,
    R: Records,
{
    let content_width = grid.width.total();
    let count_borders = grid.config.count_vertical(grid.count_columns());
    let margin = grid.config.get_margin();

    content_width + count_borders + margin.left.size + margin.right.size
}

fn print_grid<R, W, H>(grid: &Grid<'_, R, W, H>, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    W: Estimate<R>,
    H: Estimate<R>,
    R: Records,
{
    let total_width = grid.total_width();

    if grid.config.get_margin().top.size > 0 {
        print_margin_top(grid, total_width, f)?;
        f.write_char('\n')?;
    }

    let mut prev_empty_horizontal = false;
    for row in 0..grid.count_rows() {
        let height = grid.height.get(row).unwrap();

        if grid.has_horizontal(row) {
            if prev_empty_horizontal {
                '\n'.fmt(f)?;
            }

            print_margin_left(grid, f)?;
            print_split_line(grid, total_width, row, f)?;
            print_margin_right(grid, f)?;

            if height > 0 {
                '\n'.fmt(f)?;
                prev_empty_horizontal = false;
            } else {
                prev_empty_horizontal = true;
            }
        } else if height > 0 && prev_empty_horizontal {
            '\n'.fmt(f)?;
            prev_empty_horizontal = false;
        }

        for i in 0..height {
            print_margin_left(grid, f)?;

            for col in 0..grid.count_columns() {
                if !grid.config.is_cell_covered_by_both_spans((row, col)) {
                    if grid.config.is_cell_covered_by_row_span((row, col)) {
                        print_vertical_char(grid, (row, col), f)?;

                        // means it's part of other a spanned cell
                        // so. we just need to use line from other cell.
                        let original_row = closest_visible_row(grid.config, (row, col)).unwrap();

                        // considering that the content will be printed instead horizontal lines so we can skip some lines.
                        let mut skip_lines = (original_row..row)
                            .map(|i| grid.height.get(i).unwrap())
                            .sum::<usize>();

                        skip_lines += (original_row..row)
                            .map(|row| grid.has_horizontal(row) as usize)
                            .sum::<usize>();

                        let line = i + skip_lines;
                        print_cell_line(f, grid, (original_row, col), line)?;
                    } else if !grid.config.is_cell_covered_by_column_span((row, col)) {
                        print_vertical_char(grid, (row, col), f)?;
                        print_cell_line(f, grid, (row, col), i)?;
                    }
                }

                let is_last_column = col + 1 == grid.count_columns();
                if is_last_column {
                    print_vertical_char(grid, (row, col + 1), f)?;
                }
            }

            print_margin_right(grid, f)?;

            let is_last_line = i + 1 == height;
            let is_last_row = row + 1 == grid.count_rows();
            if !(is_last_line && is_last_row) {
                '\n'.fmt(f)?;
            }
        }
    }

    if grid.has_horizontal(grid.count_rows()) {
        f.write_char('\n')?;
        print_margin_left(grid, f)?;
        print_split_line(grid, total_width, grid.count_rows(), f)?;
        print_margin_right(grid, f)?;
    }

    if grid.config.get_margin().bottom.size > 0 {
        f.write_char('\n')?;
        print_margin_bottom(grid, total_width, f)?;
    }

    Ok(())
}

fn print_vertical_char<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    pos: Position,
    f: &mut fmt::Formatter<'_>,
) -> Result<(), fmt::Error>
where
    R: Records,
{
    let left = grid.get_vertical(pos);
    if let Some(c) = left {
        #[cfg(feature = "color")]
        write_colored(f, c, grid.get_vertical_color(pos))?;

        #[cfg(not(feature = "color"))]
        c.fmt(f)?;
    }

    Ok(())
}

fn print_margin_top<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    width: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent_lines(
        f,
        &grid.config.get_margin().top,
        width,
        #[cfg(feature = "color")]
        &grid.config.get_margin_color().top,
    )
}

fn print_margin_bottom<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    width: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent_lines(
        f,
        &grid.config.get_margin().bottom,
        width,
        #[cfg(feature = "color")]
        &grid.config.get_margin_color().bottom,
    )
}

fn print_margin_left<R, W, H>(grid: &Grid<'_, R, W, H>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    print_indent(
        f,
        grid.config.get_margin().left.fill,
        grid.config.get_margin().left.size,
        #[cfg(feature = "color")]
        &grid.config.get_margin_color().left,
    )
}

fn print_margin_right<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent(
        f,
        grid.config.get_margin().right.fill,
        grid.config.get_margin().right.size,
        #[cfg(feature = "color")]
        &grid.config.get_margin_color().right,
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

fn grid_cell_width<R, W, H>(grid: &Grid<'_, R, W, H>, pos: Position) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    match grid.config.get_column_span(pos) {
        Some(span) => range_width(grid, pos.1, pos.1 + span),
        None => grid.width.get(pos.1).unwrap(),
    }
}

fn range_width<R, W, H>(grid: &Grid<'_, R, W, H>, start: usize, end: usize) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    let count_borders =
        count_borders_in_range(grid.config, start, end, grid.records.count_columns());
    let range_width = (start..end)
        .map(|col| grid.width.get(col).unwrap())
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

fn grid_cell_height<R, W, H>(grid: &Grid<'_, R, W, H>, pos: Position) -> usize
where
    R: Records,
    H: Estimate<R>,
{
    match grid.config.get_row_span(pos) {
        Some(span) => range_height(grid, pos.0, pos.0 + span),
        None => grid.height.get(pos.0).unwrap(),
    }
}

fn range_height<R, W, H>(grid: &Grid<'_, R, W, H>, start: usize, end: usize) -> usize
where
    R: Records,
    H: Estimate<R>,
{
    let count_borders =
        count_horizontal_borders_in_range(grid.config, start, end, grid.records.count_rows());
    let range_width = (start..end)
        .map(|col| grid.height.get(col).unwrap())
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

fn print_split_line<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    total_width: usize,
    row: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    W: Estimate<R>,
    H: Estimate<R>,
    R: Records,
{
    // fixme: an override text may break row span.
    let mut char_skip = 0;
    let override_text = grid.config.get_split_line_text(row);
    if let Some(text) = override_text {
        if !text.is_empty() {
            let text = cut_str(text, total_width);
            let line = text.lines().next().unwrap();
            char_skip = string_width(line);
            f.write_str(line)?;
        }
    }

    #[cfg(feature = "color")]
    let mut used_color = None;

    for col in 0..grid.count_columns() {
        if col == 0 {
            let left = grid.get_intersection((row, col));
            if let Some(c) = left {
                if char_skip == 0 {
                    #[cfg(feature = "color")]
                    {
                        if let Some(clr) = grid.get_intersection_color((row, col)) {
                            clr.fmt_prefix(f)?;
                            used_color = Some(clr);
                        }
                    }

                    c.fmt(f)?;
                } else {
                    char_skip -= 1;
                }
            }
        }

        let mut width = grid.width.get(col).unwrap();
        if char_skip > 0 {
            let sub = cmp::min(width, char_skip);
            width -= sub;
            char_skip -= sub;
        }

        if grid.config.is_cell_covered_by_both_spans((row, col)) {
            continue;
        }

        let mut col = col;
        if grid.config.is_cell_covered_by_row_span((row, col)) {
            // means it's part of other a spanned cell
            // so. we just need to use line from other cell.

            let original_row = closest_visible_row(grid.config, (row, col)).unwrap();

            // considering that the content will be printed instead horizontal lines so we can skip some lines.
            let mut skip_lines = (original_row..row)
                .map(|i| grid.height.get(i).unwrap())
                .sum::<usize>();

            // skip horizontal lines
            if row > 0 {
                skip_lines += (original_row..row - 1)
                    .map(|row| grid.has_horizontal(row) as usize)
                    .sum::<usize>();
            }

            let line = skip_lines;
            print_cell_line(f, grid, (original_row, col), line)?;

            // We need to use a correct right split char.
            if let Some(span) = grid.config.get_column_span((original_row, col)) {
                col += span - 1;
            }
        } else {
            // general case
            let main = grid.get_horizontal((row, col));
            match main {
                Some(c) => {
                    #[cfg(feature = "color")]
                    {
                        prepare_coloring(
                            f,
                            grid.get_horizontal_color((row, col)),
                            &mut used_color,
                        )?;
                    }

                    repeat_char(f, *c, width)?;
                }
                None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
            }
        }

        let right = grid.get_intersection((row, col + 1));
        if let Some(c) = right {
            if char_skip == 0 {
                #[cfg(feature = "color")]
                {
                    prepare_coloring(
                        f,
                        grid.get_intersection_color((row, col + 1)),
                        &mut used_color,
                    )?;
                }

                c.fmt(f)?;
            } else {
                char_skip -= 1;
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

#[cfg(feature = "color")]
fn write_colored(
    f: &mut fmt::Formatter<'_>,
    c: impl fmt::Display,
    clr: Option<&AnsiColor>,
) -> fmt::Result {
    if let Some(clr) = &clr {
        clr.fmt_prefix(f)?;
        c.fmt(f)?;
        clr.fmt_suffix(f)?;
    } else {
        c.fmt(f)?;
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn horizontal_aligment_test() {
    //     use std::fmt;

    //     struct F<'a>(&'a str, AlignmentHorizontal, usize);

    //     impl fmt::Display for F<'_> {
    //         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //             let width = string_width(self.0);
    //             print_text_formated(f, &EmptyRecords::default(), (0, 0), self.0, 4, self.1, self.2, 0)
    //             Ok(())
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
