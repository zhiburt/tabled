//! The module contains a [`CompactGrid`] structure,
//! which is a relatively strict grid.

use core::{
    borrow::Borrow,
    fmt::{self, Display, Write},
};

use crate::{
    color::{Color, StaticColor},
    colors::{Colors, NoColors},
    config::{AlignmentHorizontal, Borders, Indent, Line, Sides},
    dimension::Dimension,
    records::Records,
    util::string::string_width,
};

use crate::config::compact::CompactConfig;

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
    #[cfg(feature = "std")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String
    where
        R: Records,
        D: Dimension,
        G: Borrow<CompactConfig>,
        C: Colors,
    {
        let mut buf = String::new();
        self.build(&mut buf).expect("It's guaranteed to never happen otherwise it's considered an stdlib error or impl error");
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
    let has_horizontal_second = cfg.get_first_horizontal_line().is_some();

    let vert = (
        borders.left.map(|c| (c, bcolors.left)),
        borders.vertical.map(|c| (c, bcolors.vertical)),
        borders.right.map(|c| (c, bcolors.right)),
    );

    let margin = cfg.get_margin();
    let margin_color = cfg.get_margin_color();
    let pad = create_padding(cfg);
    let align = cfg.get_alignment_horizontal();
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
        } else if row == 1 && has_horizontal_second {
            if new_line {
                f.write_char('\n')?;
            }

            print_indent2(f, margin.left, margin_color.left)?;

            let h_chars = cfg.get_first_horizontal_line().expect("must be here");

            if has_horizontal_colors {
                print_split_line_colored(f, h_chars, h_colors, dims, count_columns)?;
            } else {
                print_split_line(f, h_chars, dims, count_columns)?;
            }

            print_indent2(f, margin.right, margin_color.right)?;
        }

        if new_line {
            f.write_char('\n')?;
        }

        let columns = columns
            .enumerate()
            .map(|(col, text)| (text, colors.get_color((row, col))));

        let widths = widths.clone();
        print_grid_row(f, columns, widths, mar, pad, vert, align)?;

        new_line = true;
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

#[allow(clippy::too_many_arguments)]
fn print_grid_row<F, I, T, C, D>(
    f: &mut F,
    columns: I,
    widths: D,
    mar: (ColoredIndent, ColoredIndent),
    pad: Sides<ColoredIndent>,
    vert: (BorderChar, BorderChar, BorderChar),
    align: AlignmentHorizontal,
) -> fmt::Result
where
    F: Write,
    I: Iterator<Item = (T, Option<C>)>,
    T: AsRef<str>,
    C: Color,
    D: Iterator<Item = usize> + Clone,
{
    if pad.top.0.size > 0 {
        for _ in 0..pad.top.0.size {
            print_indent2(f, mar.0 .0, mar.0 .1)?;
            print_columns_empty_colored(f, widths.clone(), vert, pad.top.1)?;
            print_indent2(f, mar.1 .0, mar.1 .1)?;

            f.write_char('\n')?;
        }
    }

    let mut widths1 = widths.clone();
    let columns = columns.map(move |(text, color)| {
        let width = widths1.next().expect("must be here");
        (text, color, width)
    });

    print_indent2(f, mar.0 .0, mar.0 .1)?;
    print_row_columns(f, columns, vert, pad, align)?;
    print_indent2(f, mar.1 .0, mar.1 .1)?;

    for _ in 0..pad.bottom.0.size {
        f.write_char('\n')?;

        print_indent2(f, mar.0 .0, mar.0 .1)?;
        print_columns_empty_colored(f, widths.clone(), vert, pad.bottom.1)?;
        print_indent2(f, mar.1 .0, mar.1 .1)?;
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

fn create_horizontal(b: &Borders<char>) -> Line<char> {
    Line::new(b.horizontal.unwrap_or(' '), b.intersection, b.left, b.right)
}

fn create_horizontal_top(b: &Borders<char>) -> Line<char> {
    Line::new(
        b.top.unwrap_or(' '),
        b.top_intersection,
        b.top_left,
        b.top_right,
    )
}

fn create_horizontal_bottom(b: &Borders<char>) -> Line<char> {
    Line::new(
        b.bottom.unwrap_or(' '),
        b.bottom_intersection,
        b.bottom_left,
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

fn total_width<D: Dimension>(cfg: &CompactConfig, dims: &D, count_columns: usize) -> usize {
    let content_width = total_columns_width(count_columns, dims);
    let count_verticals = count_verticals(cfg, count_columns);

    content_width + count_verticals
}

fn total_columns_width<D: Dimension>(count_columns: usize, dims: &D) -> usize {
    (0..count_columns).map(|i| dims.get_width(i)).sum::<usize>()
}

fn count_verticals(cfg: &CompactConfig, count_columns: usize) -> usize {
    assert!(count_columns > 0);

    let count_verticals = count_columns - 1;
    let borders = cfg.get_borders();
    borders.has_vertical() as usize * count_verticals
        + borders.has_left() as usize
        + borders.has_right() as usize
}

type BorderChar = Option<(char, Option<StaticColor>)>;

fn print_row_columns<F, I, T, C>(
    f: &mut F,
    mut columns: I,
    borders: (BorderChar, BorderChar, BorderChar),
    pad: Sides<ColoredIndent>,
    align: AlignmentHorizontal,
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
        print_cell(f, text, width, color, (pad.left, pad.right), align)?;
    }

    for (text, color, width) in columns {
        if let Some((c, color)) = borders.1 {
            print_char(f, c, color)?;
        }

        let text = text.as_ref();
        let text = text.lines().next().unwrap_or("");
        print_cell(f, text, width, color, (pad.left, pad.right), align)?;
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

fn print_cell<F: Write, C: Color>(
    f: &mut F,
    text: &str,
    width: usize,
    color: Option<C>,
    (pad_l, pad_r): (ColoredIndent, ColoredIndent),
    align: AlignmentHorizontal,
) -> fmt::Result {
    let available = width - pad_l.0.size - pad_r.0.size;

    let text_width = string_width(text);
    let (left, right) = if available < text_width {
        (0, 0)
    } else {
        calculate_indent(align, text_width, available)
    };

    print_indent(f, pad_l.0.fill, pad_l.0.size, pad_l.1)?;

    repeat_char(f, ' ', left)?;
    print_text(f, text, color)?;
    repeat_char(f, ' ', right)?;

    print_indent(f, pad_r.0.fill, pad_r.0.size, pad_r.1)?;

    Ok(())
}

fn print_split_line_colored<F: Write>(
    f: &mut F,
    chars: Line<char>,
    colors: (StaticColor, StaticColor, StaticColor, StaticColor),
    dimension: impl Dimension,
    count_columns: usize,
) -> fmt::Result {
    let mut used_color = StaticColor::default();

    if let Some(c) = chars.connect1 {
        colors.1.fmt_prefix(f)?;
        f.write_char(c)?;
        used_color = colors.1;
    }

    let width = dimension.get_width(0);
    if width > 0 {
        prepare_coloring(f, &colors.0, &mut used_color)?;
        repeat_char(f, chars.main, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = &chars.intersection {
            prepare_coloring(f, &colors.2, &mut used_color)?;
            f.write_char(*c)?;
        }

        let width = dimension.get_width(col);
        if width > 0 {
            prepare_coloring(f, &colors.0, &mut used_color)?;
            repeat_char(f, chars.main, width)?;
        }
    }

    if let Some(c) = &chars.connect2 {
        prepare_coloring(f, &colors.3, &mut used_color)?;
        f.write_char(*c)?;
    }

    used_color.fmt_suffix(f)?;

    Ok(())
}

fn print_split_line<F: Write>(
    f: &mut F,
    chars: Line<char>,
    dimension: impl Dimension,
    count_columns: usize,
) -> fmt::Result {
    if let Some(c) = chars.connect1 {
        f.write_char(c)?;
    }

    let width = dimension.get_width(0);
    if width > 0 {
        repeat_char(f, chars.main, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = chars.intersection {
            f.write_char(c)?;
        }

        let width = dimension.get_width(col);
        if width > 0 {
            repeat_char(f, chars.main, width)?;
        }
    }

    if let Some(c) = chars.connect2 {
        f.write_char(c)?;
    }

    Ok(())
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

fn prepare_coloring<F: Write>(f: &mut F, clr: &StaticColor, used: &mut StaticColor) -> fmt::Result {
    if *used != *clr {
        used.fmt_suffix(f)?;
        clr.fmt_prefix(f)?;
        *used = *clr;
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

    for _ in 1..indent.size {
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
