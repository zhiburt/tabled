//! The module contains a [`Grid`] structure.

use core::{
    borrow::Borrow,
    fmt::{self, Display, Write},
};

use crate::{
    color::{Color, StaticColor},
    colors::{Colors, NoColors},
    config::{AlignmentHorizontal, AlignmentVertical, Borders, Indent, Sides},
    dimension::Dimension,
    records::Records,
    util::string::{get_lines, string_width_tab},
};

use super::config::CompactConfig;

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct CompactGrid<R, D, G, C> {
    records: R,
    config: G,
    dimension: D,
    colors: C,
}

impl<R, D, G> CompactGrid<R, D, G, NoColors> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, dimension: D, config: G) -> Self {
        CompactGrid {
            records,
            config,
            dimension,
            colors: NoColors::default(),
        }
    }
}

impl<R, D, G, C> CompactGrid<R, D, G, C> {
    /// Sets colors map.
    pub fn with_colors<Colors>(self, colors: Colors) -> CompactGrid<R, D, G, Colors> {
        CompactGrid {
            records: self.records,
            config: self.config,
            dimension: self.dimension,
            colors,
        }
    }

    /// Builds a table.
    pub fn build<F>(self, mut f: F) -> fmt::Result
    where
        R: Records,
        D: Dimension,
        C: Colors,
        G: Borrow<CompactConfig>,
        F: Write,
    {
        if self.records.count_columns() == 0 {
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
        G: Borrow<CompactConfig>,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranted to never happen otherwise it's considered an stdlib erorr or impl error");
        buf
    }
}

impl<R, D, G, C> Display for CompactGrid<R, D, G, C>
where
    for<'a> &'a R: Records,
    D: Dimension,
    G: Borrow<CompactConfig>,
    C: Colors,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let records = &self.records;
        let config = self.config.borrow();

        print_grid(f, records, config, &self.dimension, &self.colors)
    }
}

fn print_grid<F: Write, R: Records, D: Dimension, C: Colors>(
    f: &mut F,
    records: R,
    cfg: &CompactConfig,
    dims: &D,
    colors: &C,
) -> fmt::Result {
    let count_columns = records.count_columns();
    let count_rows = records.hint_count_rows();

    if count_columns == 0 || matches!(count_rows, Some(0)) {
        return Ok(());
    }

    let mut records = records.iter_rows().into_iter();
    let mut next_columns = records.next();

    if next_columns.is_none() {
        return Ok(());
    }

    let wtotal = total_width(cfg, dims, count_columns);

    let borders = cfg.get_borders();
    let bcolors = cfg.get_borders_color();

    let h_chars = create_horizontal(borders);
    let h_colors = create_horizontal_colors(bcolors);

    let has_horizontal = borders.has_horizontal();
    let has_horizontal_colors = bcolors.has_horizontal();

    let vert = (
        borders.left.map(|c| (c, bcolors.left)),
        borders.vertical.map(|c| (c, bcolors.vertical)),
        borders.right.map(|c| (c, bcolors.right)),
    );

    let margin = cfg.get_margin();
    let margin_color = cfg.get_margin_color();
    let pad = create_padding(cfg);
    let align = cfg.get_alignment_horizontal();
    let valign = cfg.get_alignment_vertical();
    let tab_size = cfg.get_tab_width();
    let mar = (
        (margin.left, margin_color.left),
        (margin.right, margin_color.right),
    );

    let widths = (0..count_columns).map(|col| dims.get_width(col));

    let mut new_line = false;

    if margin.top.size > 0 {
        let wtotal = wtotal + margin.left.size + margin.right.size;
        print_indent_lines(f, wtotal, margin.top, margin_color.top)?;
        new_line = true;
    }

    if borders.has_top() {
        if new_line {
            f.write_char('\n')?
        }

        print_indent2(f, margin.left, margin_color.left)?;

        let chars = create_horizontal_top(borders);
        if bcolors.has_top() {
            let chars_color = create_horizontal_top_colors(bcolors);
            print_split_line_colored(f, chars, chars_color, dims, count_columns)?;
        } else {
            print_split_line(f, chars, dims, count_columns)?;
        }

        print_indent2(f, margin.right, margin_color.right)?;

        new_line = true;
    }

    let mut row = 0;
    while let Some(columns) = next_columns {
        let columns = columns.into_iter();
        next_columns = records.next();

        if row > 0 && has_horizontal {
            if new_line {
                f.write_char('\n')?;
            }

            print_indent2(f, margin.left, margin_color.left)?;

            if has_horizontal_colors {
                print_split_line_colored(f, h_chars, h_colors, dims, count_columns)?;
            } else {
                print_split_line(f, h_chars, dims, count_columns)?;
            }

            print_indent2(f, margin.right, margin_color.right)?;
        }

        let height = dims.get_height(row);

        if height > 0 {
            if new_line {
                f.write_char('\n')?;
            }

            let columns = columns
                .enumerate()
                .map(|(col, text)| (text, colors.get_color((row, col))));

            let widths = widths.clone();
            print_grid_row(
                f, columns, widths, height, mar, pad, vert, align, valign, tab_size,
            )?;

            new_line = true;
        }

        row += 1;
    }

    if borders.has_bottom() {
        f.write_char('\n')?;

        print_indent2(f, margin.left, margin_color.left)?;

        let chars = create_horizontal_bottom(borders);
        if bcolors.has_bottom() {
            let chars_color = create_horizontal_bottom_colors(bcolors);
            print_split_line_colored(f, chars, chars_color, dims, count_columns)?;
        } else {
            print_split_line(f, chars, dims, count_columns)?;
        }

        print_indent2(f, margin.right, margin_color.right)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;

        let wtotal = wtotal + margin.left.size + margin.right.size;
        print_indent_lines(f, wtotal, margin.bottom, margin_color.bottom)?;
    }

    Ok(())
}

type ColoredIndent = (Indent, StaticColor);

fn print_grid_row<F, I, T, C, D>(
    f: &mut F,
    mut columns: I,
    widths: D,
    height: usize,
    mar: (ColoredIndent, ColoredIndent),
    pad: Sides<ColoredIndent>,
    vert: (BorderChar, BorderChar, BorderChar),
    align: AlignmentHorizontal,
    valign: AlignmentVertical,
    tab_size: usize,
) -> fmt::Result
where
    F: Write,
    I: Iterator<Item = (T, Option<C>)>,
    T: AsRef<str>,
    C: Color,
    D: Iterator<Item = usize> + Clone,
{
    {
        let top_indent = top_indent(pad.top.0.size, valign, 1, height);
        if top_indent > 0 {
            for _ in 0..top_indent {
                print_indent2(f, mar.0 .0, mar.0 .1)?;
                print_columns_empty(f, widths.clone(), vert)?;
                print_indent2(f, mar.1 .0, mar.1 .1)?;

                f.write_char('\n')?;
            }
        }
    }
    {
        if pad.top.0.size > 0 {
            for _ in 0..pad.top.0.size {
                print_indent2(f, mar.0 .0, mar.0 .1)?;
                print_columns_empty_colored(f, widths.clone(), vert, pad.top.1)?;
                print_indent2(f, mar.1 .0, mar.1 .1)?;

                f.write_char('\n')?;
            }
        }
    }

    let widths1 = widths.clone();
    let columns = widths1.map(move |width| {
        let (text, color) = columns.next().expect("must be here");
        (text, color, width)
    });

    print_indent2(f, mar.0 .0, mar.0 .1)?;
    print_row_columns(f, columns, vert, pad, align, tab_size)?;
    print_indent2(f, mar.1 .0, mar.1 .1)?;

    {
        if pad.bottom.0.size > 0 {
            for _ in 0..pad.bottom.0.size {
                f.write_char('\n')?;

                print_indent2(f, mar.0 .0, mar.0 .1)?;
                print_columns_empty_colored(f, widths.clone(), vert, pad.bottom.1)?;
                print_indent2(f, mar.1 .0, mar.1 .1)?;
            }
        }
    }

    Ok(())
}

fn create_padding(cfg: &CompactConfig) -> Sides<ColoredIndent> {
    let pad = cfg.get_padding();
    let pad_colors = cfg.get_padding_color();
    Sides::new(
        (pad.left, pad_colors.left),
        (pad.right, pad_colors.right),
        (pad.top, pad_colors.top),
        (pad.bottom, pad_colors.bottom),
    )
}

fn create_horizontal(b: &Borders<char>) -> (char, Option<char>, Option<char>, Option<char>) {
    (b.horizontal.unwrap_or(' '), b.left, b.intersection, b.right)
}

fn create_horizontal_top(b: &Borders<char>) -> (char, Option<char>, Option<char>, Option<char>) {
    (
        b.top.unwrap_or(' '),
        b.top_left,
        b.top_intersection,
        b.top_right,
    )
}

fn create_horizontal_bottom(b: &Borders<char>) -> (char, Option<char>, Option<char>, Option<char>) {
    (
        b.bottom.unwrap_or(' '),
        b.bottom_left,
        b.bottom_intersection,
        b.bottom_right,
    )
}

fn create_horizontal_colors(
    b: &Borders<StaticColor>,
) -> (StaticColor, StaticColor, StaticColor, StaticColor) {
    (
        b.horizontal.unwrap_or(StaticColor::default()),
        b.left.unwrap_or(StaticColor::default()),
        b.intersection.unwrap_or(StaticColor::default()),
        b.right.unwrap_or(StaticColor::default()),
    )
}

fn create_horizontal_top_colors(
    b: &Borders<StaticColor>,
) -> (StaticColor, StaticColor, StaticColor, StaticColor) {
    (
        b.top.unwrap_or(StaticColor::default()),
        b.top_left.unwrap_or(StaticColor::default()),
        b.top_intersection.unwrap_or(StaticColor::default()),
        b.top_right.unwrap_or(StaticColor::default()),
    )
}

fn create_horizontal_bottom_colors(
    b: &Borders<StaticColor>,
) -> (StaticColor, StaticColor, StaticColor, StaticColor) {
    (
        b.bottom.unwrap_or(StaticColor::default()),
        b.bottom_left.unwrap_or(StaticColor::default()),
        b.bottom_intersection.unwrap_or(StaticColor::default()),
        b.bottom_right.unwrap_or(StaticColor::default()),
    )
}

fn create_margin(cfg: &CompactConfig) -> Sides<ColoredIndent> {
    let indent = cfg.get_margin();
    let color = cfg.get_margin_color();

    Sides::new(
        (indent.left, color.left),
        (indent.right, color.right),
        (indent.top, color.top),
        (indent.bottom, color.bottom),
    )
}

fn total_width<D: Dimension>(cfg: &CompactConfig, dims: &D, count_columns: usize) -> usize {
    let content_width = total_columns_width(count_columns, dims);
    let count_verticals = count_verticals(cfg, count_columns);

    content_width + count_verticals
}

fn total_columns_width<D: Dimension>(count_columns: usize, dims: &D) -> usize {
    (0..count_columns).map(|i| dims.get_width(i)).sum::<usize>()
}

fn count_verticals(cfg: &CompactConfig, count_columns: usize) -> usize {
    assert!(count_columns > 1);

    let count_verticals = count_columns - 2;
    let borders = cfg.get_borders();
    borders.has_vertical() as usize * count_verticals
        + borders.has_left() as usize
        + borders.has_right() as usize
}

fn has_vertical(cfg: &CompactConfig, is_first: bool, is_last: bool) -> bool {
    if is_last {
        cfg.get_borders().has_right()
    } else if is_first {
        cfg.get_borders().has_left()
    } else {
        cfg.get_borders().has_vertical()
    }
}

fn total_height<D: Dimension>(cfg: &CompactConfig, dims: &D, count_rows: usize) -> usize {
    let content_height = total_rows_height(count_rows, dims);
    let count_horizontals = count_horizontals(cfg, count_rows);

    content_height + count_horizontals
}

fn count_horizontals(cfg: &CompactConfig, count_rows: usize) -> usize {
    assert!(count_rows > 1);

    let count_verticals = count_rows - 2;
    let borders = cfg.get_borders();
    borders.has_horizontal() as usize * count_rows
        + borders.has_top() as usize
        + borders.has_bottom() as usize
}

fn total_rows_height<D: Dimension>(count_rows: usize, dims: &D) -> usize {
    (0..count_rows).map(|i| dims.get_height(i)).sum::<usize>()
}

type BorderChar = Option<(char, Option<StaticColor>)>;

fn print_row_columns<F, I, T, C>(
    f: &mut F,
    mut columns: I,
    borders: (BorderChar, BorderChar, BorderChar),
    pad: Sides<ColoredIndent>,
    align: AlignmentHorizontal,
    tab: usize,
) -> Result<(), fmt::Error>
where
    F: Write,
    I: Iterator<Item = (T, Option<C>, usize)>,
    T: AsRef<str>,
    C: Color,
{
    if let Some((c, color)) = borders.0 {
        print_char(f, c, color)?;
    }

    if let Some((text, color, width)) = columns.next() {
        let text = text.as_ref();
        let text = text.lines().next().unwrap_or("");
        print_cell(f, text, width, color, (pad.left, pad.right), align, tab)?;
    }

    for (text, color, width) in columns {
        if let Some((c, color)) = borders.1 {
            print_char(f, c, color)?;
        }

        let text = text.as_ref();
        let text = text.lines().next().unwrap_or("");
        print_cell(f, text, width, color, (pad.left, pad.right), align, tab)?;
    }

    if let Some((c, color)) = borders.2 {
        print_char(f, c, color)?;
    }

    Ok(())
}

fn print_columns_empty_colored<F: Write, I: Iterator<Item = usize>>(
    f: &mut F,
    mut columns: I,
    borders: (BorderChar, BorderChar, BorderChar),
    color: StaticColor,
) -> Result<(), fmt::Error> {
    if let Some((c, color)) = borders.0 {
        print_char(f, c, color)?;
    }

    if let Some(width) = columns.next() {
        color.fmt_prefix(f)?;
        repeat_char(f, ' ', width)?;
        color.fmt_suffix(f)?;
    }

    for width in columns {
        if let Some((c, color)) = borders.1 {
            print_char(f, c, color)?;
        }

        color.fmt_prefix(f)?;
        repeat_char(f, ' ', width)?;
        color.fmt_suffix(f)?;
    }

    if let Some((c, color)) = borders.2 {
        print_char(f, c, color)?;
    }

    Ok(())
}

fn print_columns_empty<F: Write, I: Iterator<Item = usize>>(
    f: &mut F,
    mut columns: I,
    borders: (BorderChar, BorderChar, BorderChar),
) -> Result<(), fmt::Error> {
    if let Some((c, color)) = borders.0 {
        print_char(f, c, color)?;
    }

    if let Some(width) = columns.next() {
        repeat_char(f, ' ', width)?;
    }

    for width in columns {
        if let Some((c, color)) = borders.1 {
            print_char(f, c, color)?;
        }

        repeat_char(f, ' ', width)?;
    }

    if let Some((c, color)) = borders.2 {
        print_char(f, c, color)?;
    }

    Ok(())
}

fn print_cell<F: Write, C: Color>(
    f: &mut F,
    text: &str,
    width: usize,
    color: Option<C>,
    (pad_l, pad_r): (ColoredIndent, ColoredIndent),
    align: AlignmentHorizontal,
    tab_size: usize,
) -> fmt::Result {
    let available = width - pad_l.0.size - pad_r.0.size;

    let text_width = string_width_tab(&text, tab_size);
    let (left, right) = if available < text_width {
        (0, 0)
    } else {
        calculate_indent(align, text_width, available)
    };

    print_indent(f, pad_l.0.fill, pad_l.0.size, pad_l.1)?;

    repeat_char(f, ' ', left)?;
    print_text(f, &text, tab_size, color)?;
    repeat_char(f, ' ', right)?;

    print_indent(f, pad_r.0.fill, pad_r.0.size, pad_r.1)?;

    Ok(())
}

fn print_split_line_colored<F: Write>(
    f: &mut F,
    chars: (char, Option<char>, Option<char>, Option<char>),
    colors: (StaticColor, StaticColor, StaticColor, StaticColor),
    dimension: impl Dimension,
    count_columns: usize,
) -> fmt::Result {
    let mut used_color = StaticColor::default();

    if let Some(c) = chars.1 {
        colors.1.fmt_prefix(f)?;
        f.write_char(c)?;
        used_color = colors.1;
    }

    let width = dimension.get_width(0);
    if width > 0 {
        prepare_coloring(f, &colors.0, &mut used_color)?;
        repeat_char(f, chars.0, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = &chars.2 {
            prepare_coloring(f, &colors.2, &mut used_color)?;
            f.write_char(*c)?;
        }

        let width = dimension.get_width(col);
        if width > 0 {
            prepare_coloring(f, &colors.0, &mut used_color)?;
            repeat_char(f, chars.0, width)?;
        }
    }

    if let Some(c) = &chars.3 {
        prepare_coloring(f, &colors.3, &mut used_color)?;
        f.write_char(*c)?;
    }

    used_color.fmt_suffix(f)?;

    Ok(())
}

fn print_split_line<F: Write>(
    f: &mut F,
    chars: (char, Option<char>, Option<char>, Option<char>),
    dimension: impl Dimension,
    count_columns: usize,
) -> fmt::Result {
    if let Some(c) = chars.1 {
        f.write_char(c)?;
    }

    let width = dimension.get_width(0);
    if width > 0 {
        repeat_char(f, chars.0, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = chars.2 {
            f.write_char(c)?;
        }

        let width = dimension.get_width(col);
        if width > 0 {
            repeat_char(f, chars.0, width)?;
        }
    }

    if let Some(c) = chars.3 {
        f.write_char(c)?;
    }

    Ok(())
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
    clr: &StaticColor,
    used: &mut StaticColor,
) -> fmt::Result {
    if *used != *clr {
        used.fmt_suffix(f)?;
        clr.fmt_prefix(f)?;
        *used = *clr;
    }

    Ok(())
}

fn top_indent(pad_top: usize, valign: AlignmentVertical, height: usize, available: usize) -> usize {
    let height = available - pad_top;
    let indent = indent_from_top(valign, height, height);

    indent + pad_top
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

            continue;
        }

        if is_empty {
            bottom += 1;
        } else {
            bottom = 0;
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

fn print_char<F: Write>(f: &mut F, c: char, color: Option<StaticColor>) -> fmt::Result {
    match color {
        Some(color) => {
            color.fmt_prefix(f)?;
            f.write_char(c)?;
            color.fmt_suffix(f)
        }
        None => f.write_char(c),
    }
}

fn print_indent_lines<F: Write>(
    f: &mut F,
    width: usize,
    indent: Indent,
    color: StaticColor,
) -> fmt::Result {
    print_indent(f, indent.fill, width, color)?;
    f.write_char('\n')?;

    for i in 1..indent.size {
        f.write_char('\n')?;
        print_indent(f, indent.fill, width, color)?;
    }

    Ok(())
}

fn print_indent<F: Write>(f: &mut F, c: char, n: usize, color: StaticColor) -> fmt::Result {
    color.fmt_prefix(f)?;
    repeat_char(f, c, n)?;
    color.fmt_suffix(f)?;

    Ok(())
}

fn print_indent2<F: Write>(f: &mut F, indent: Indent, color: StaticColor) -> fmt::Result {
    print_indent(f, indent.fill, indent.size, color)
}

fn convert_count_rows(row: usize, is_last: bool) -> usize {
    if is_last {
        row + 1
    } else {
        row + 2
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

// pub trait GridProjection1 {
//     fn print_margin_top<F: Write>(&self, width: usize, f: &mut F) -> &Margin;
//     fn print_margin_bottom<F: Write>(&self, width: usize, f: &mut F) -> &Margin;
//     fn print_margin_left<F: Write>(&self, width: usize, f: &mut F) -> &Margin;
//     fn print_margin_right<F: Write>(&self, width: usize, f: &mut F) -> &Margin;
//     fn print_horizontal_line<F: Write>(&self, width: usize, f: &mut F) -> &Margin;
//     fn print_vertical_char<F: Write>(&self, width: usize, f: &mut F) -> &Margin;

//     fn get_margin(&self) -> &Margin;
//     fn get_margin_color(&self) -> &Margin;
//     fn get_padding(&self, pos: Position) -> &Padding;
//     fn get_padding_color(&self, pos: Position) -> &PaddingColor<'_>;
//     fn get_vertical(&self, pos: Position, line: usize) -> Option<char>;
//     fn get_horizonal(&self, pos: Position, width: usize) -> Option<char>;
//     fn get_horizonal_line(&self, pos: Position, width: usize) -> Option<char>;
//     fn get_span_horizontal(&self, pos: Position) -> Option<usize>;
//     fn get_span_vertical(&self, pos: Position) -> Option<usize>;
//     fn has_horizontal(&self, line: usize) -> bool;
//     fn has_vertical(&self, line: usize) -> bool;
//     fn has_spans(&self) -> bool;

//     fn print_horizontal_border(&self)
// }
