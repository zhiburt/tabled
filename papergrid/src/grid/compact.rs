//! The module contains a [`CompactGrid`] structure,
//! which is a relatively strict grid.

use core::{
    borrow::Borrow,
    fmt::{self, Display, Write},
};

use crate::{
    ansi::{ANSIFmt, ANSIStr},
    colors::Colors,
    config::{AlignmentHorizontal, Borders, HorizontalLine, Indent, Sides},
    dimension::Dimension,
    records::{IntoRecords, Records},
    util::string::get_line_width,
};

use crate::config::compact::CompactConfig;

type ANSIString = ANSIStr<'static>;

/// Grid provides a set of methods for building a text-based table.
#[derive(Debug, Clone)]
pub struct CompactGrid<R, D, G, C> {
    records: R,
    config: G,
    dimension: D,
    colors: C,
}

impl<R, D, G, C> CompactGrid<R, D, G, C> {
    /// The new method creates a grid instance with default styles.
    pub fn new(records: R, config: G, dimension: D, colors: C) -> Self {
        CompactGrid {
            records,
            config,
            dimension,
            colors,
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
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
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
        <R::Iter as IntoRecords>::Cell: AsRef<str>,
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
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
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

#[derive(Debug, Clone)]
struct RowConfig<D, C> {
    margin: Sides<ColoredIndent>,
    pad: Sides<ColoredIndent>,
    verticals: HorizontalLine<ColoredIndent>,
    alignment: AlignmentHorizontal,
    dims: D,
    colors: C,
    count_columns: usize,
}

impl<D, C> RowConfig<D, C> {
    fn new(cfg: &CompactConfig, dims: D, colors: C, count_columns: usize) -> Self {
        let borders_chars = cfg.get_borders();
        let borders_colors = cfg.get_borders_color();
        let verticals = create_vertical_borders(borders_chars, borders_colors);
        let margin = create_margin(cfg);
        let pad = create_padding(cfg);
        let alignment = cfg.get_alignment_horizontal();

        Self {
            margin,
            pad,
            alignment,
            verticals,
            colors,
            dims,
            count_columns,
        }
    }
}

#[derive(Debug, Clone)]
struct RowIter<I> {
    iter: I,
    row: usize,
}

impl<I> RowIter<I> {
    fn new(iter: I, row: usize) -> Self {
        Self { iter, row }
    }
}

fn print_grid<F, R, D, C>(
    f: &mut F,
    records: R,
    cfg: &CompactConfig,
    dims: &D,
    colors: &C,
) -> fmt::Result
where
    F: Write,
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    let count_columns = records.count_columns();
    let count_rows = records.hint_count_rows();

    if count_columns == 0 || matches!(count_rows, Some(0)) {
        return Ok(());
    }

    let mut records = records.iter_rows().into_iter();
    let records_first = match records.next() {
        Some(row) => row,
        None => return Ok(()),
    };

    let wtotal = total_width(cfg, dims, count_columns);

    let borders_chars = cfg.get_borders();
    let borders_colors = cfg.get_borders_color();

    let horizontal_borders = create_horizontal(borders_chars);
    let horizontal_colors = create_horizontal_colors(borders_colors);

    let margin = create_margin(cfg);

    let rowcfg = RowConfig::new(cfg, dims, colors, count_columns);

    let mut new_line = false;

    if margin.top.space.size > 0 {
        let width_total = wtotal + margin.left.space.size + margin.right.space.size;
        let indent = ColoredIndent::new(width_total, margin.top.space.fill, margin.top.color);
        print_indent_lines(f, indent)?;
        new_line = true;
    }

    if borders_chars.has_top() {
        if new_line {
            f.write_char('\n')?
        }

        let borders = create_horizontal_top(borders_chars);
        let borders_colors = create_horizontal_top_colors(borders_colors);
        print_horizontal_line(f, dims, &borders, &borders_colors, &margin, count_columns)?;

        new_line = true;
    }

    if borders_chars.has_horizontal() {
        if new_line {
            f.write_char('\n')?;
        }

        let cells = records_first.into_iter();
        let iter = RowIter::new(cells, 0);
        print_grid_row(f, iter, &rowcfg)?;

        for (row, cells) in records.enumerate() {
            f.write_char('\n')?;

            print_horizontal_line(
                f,
                dims,
                &horizontal_borders,
                &horizontal_colors,
                &margin,
                count_columns,
            )?;

            f.write_char('\n')?;

            let cells = cells.into_iter();
            let iter = RowIter::new(cells, row + 1);
            print_grid_row(f, iter, &rowcfg)?;
        }
    } else {
        if new_line {
            f.write_char('\n')?;
        }

        let cells = records_first.into_iter();
        let iter = RowIter::new(cells, 0);
        print_grid_row(f, iter, &rowcfg)?;

        for (row, cells) in records.enumerate() {
            f.write_char('\n')?;

            let cells = cells.into_iter();
            let iter = RowIter::new(cells, row + 1);
            print_grid_row(f, iter, &rowcfg)?;
        }
    }

    if borders_chars.has_bottom() {
        f.write_char('\n')?;

        let borders = create_horizontal_bottom(borders_chars);
        let colors = create_horizontal_bottom_colors(borders_colors);
        print_horizontal_line(f, dims, &borders, &colors, &margin, count_columns)?;
    }

    if cfg.get_margin().bottom.size > 0 {
        f.write_char('\n')?;

        let width_total = wtotal + margin.left.space.size + margin.right.space.size;
        let indent = ColoredIndent::new(width_total, margin.bottom.space.fill, margin.bottom.color);
        print_indent_lines(f, indent)?;
    }

    Ok(())
}

fn create_margin(cfg: &CompactConfig) -> Sides<ColoredIndent> {
    let margin = cfg.get_margin();
    let margin_color = cfg.get_margin_color();
    Sides::new(
        ColoredIndent::from_indent(margin.left, margin_color.left),
        ColoredIndent::from_indent(margin.right, margin_color.right),
        ColoredIndent::from_indent(margin.top, margin_color.top),
        ColoredIndent::from_indent(margin.bottom, margin_color.bottom),
    )
}

fn create_vertical_borders(
    borders: &Borders<char>,
    colors: &Borders<ANSIString>,
) -> HorizontalLine<ColoredIndent> {
    let intersect = borders
        .vertical
        .map(|c| ColoredIndent::new(0, c, colors.vertical));
    let left = borders.left.map(|c| ColoredIndent::new(0, c, colors.left));
    let right = borders
        .right
        .map(|c| ColoredIndent::new(0, c, colors.right));

    HorizontalLine::new(None, intersect, left, right)
}

fn print_horizontal_line<F, D>(
    f: &mut F,
    dims: &D,
    borders: &HorizontalLine<char>,
    borders_colors: &HorizontalLine<ANSIString>,
    margin: &Sides<ColoredIndent>,
    count_columns: usize,
) -> fmt::Result
where
    F: fmt::Write,
    D: Dimension,
{
    let is_not_colored = borders_colors.is_empty();

    print_indent(f, margin.left)?;

    if is_not_colored {
        print_split_line(f, dims, borders, count_columns)?;
    } else {
        print_split_line_colored(f, dims, borders, borders_colors, count_columns)?;
    }

    print_indent(f, margin.right)?;

    Ok(())
}

fn print_grid_row<F, I, D, C>(f: &mut F, iter: RowIter<I>, rowcfg: &RowConfig<D, C>) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    for _ in 0..rowcfg.pad.top.space.size {
        print_indent(f, rowcfg.margin.left)?;
        print_row_columns_empty(f, rowcfg, rowcfg.pad.top.color)?;
        print_indent(f, rowcfg.margin.right)?;

        f.write_char('\n')?;
    }

    print_indent(f, rowcfg.margin.left)?;
    print_row_columns(f, iter, rowcfg)?;
    print_indent(f, rowcfg.margin.right)?;

    for _ in 0..rowcfg.pad.bottom.space.size {
        f.write_char('\n')?;

        print_indent(f, rowcfg.margin.left)?;
        print_row_columns_empty(f, rowcfg, rowcfg.pad.bottom.color)?;
        print_indent(f, rowcfg.margin.right)?;
    }

    Ok(())
}

fn create_padding(cfg: &CompactConfig) -> Sides<ColoredIndent> {
    let pad = cfg.get_padding();
    let colors = cfg.get_padding_color();
    Sides::new(
        ColoredIndent::new(pad.left.size, pad.left.fill, create_color(colors.left)),
        ColoredIndent::new(pad.right.size, pad.right.fill, create_color(colors.right)),
        ColoredIndent::new(pad.top.size, pad.top.fill, create_color(colors.top)),
        ColoredIndent::new(
            pad.bottom.size,
            pad.bottom.fill,
            create_color(colors.bottom),
        ),
    )
}

fn create_horizontal(b: &Borders<char>) -> HorizontalLine<char> {
    HorizontalLine::new(b.horizontal, b.intersection, b.left, b.right)
}

fn create_horizontal_top(b: &Borders<char>) -> HorizontalLine<char> {
    HorizontalLine::new(b.top, b.top_intersection, b.top_left, b.top_right)
}

fn create_horizontal_bottom(b: &Borders<char>) -> HorizontalLine<char> {
    HorizontalLine::new(
        b.bottom,
        b.bottom_intersection,
        b.bottom_left,
        b.bottom_right,
    )
}

fn create_horizontal_colors(b: &Borders<ANSIString>) -> HorizontalLine<ANSIString> {
    HorizontalLine::new(b.horizontal, b.intersection, b.left, b.right)
}

fn create_horizontal_top_colors(b: &Borders<ANSIString>) -> HorizontalLine<ANSIString> {
    HorizontalLine::new(b.top, b.top_intersection, b.top_left, b.top_right)
}

fn create_horizontal_bottom_colors(b: &Borders<ANSIString>) -> HorizontalLine<ANSIString> {
    HorizontalLine::new(
        b.bottom,
        b.bottom_intersection,
        b.bottom_left,
        b.bottom_right,
    )
}

fn total_width<D>(cfg: &CompactConfig, dims: &D, count_columns: usize) -> usize
where
    D: Dimension,
{
    let content_width = total_columns_width(dims, count_columns);
    let count_verticals = count_verticals(cfg, count_columns);

    content_width + count_verticals
}

fn total_columns_width<D>(dims: &D, count_columns: usize) -> usize
where
    D: Dimension,
{
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

fn print_row_columns<F, I, D, C>(
    f: &mut F,
    mut iter: RowIter<I>,
    rowcfg: &RowConfig<D, C>,
) -> fmt::Result
where
    F: Write,
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
    C: Colors,
{
    if let Some(indent) = rowcfg.verticals.left {
        print_char(f, indent.space.fill, indent.color)?;
    }

    let text = iter
        .iter
        .next()
        .expect("we check in the beginning that size must be at least 1 column");
    let width = rowcfg.dims.get_width(0);
    let color = rowcfg.colors.get_color((iter.row, 0).into());

    let text = text.as_ref();
    let text = text.lines().next().unwrap_or("");
    print_cell(f, text, color, &rowcfg.pad, rowcfg.alignment, width)?;

    match rowcfg.verticals.intersection {
        Some(indent) => {
            for (col, text) in iter.iter.enumerate() {
                let col = col + 1;

                let width = rowcfg.dims.get_width(col);
                let color = rowcfg.colors.get_color((iter.row, col).into());
                let text = text.as_ref();
                let text = text.lines().next().unwrap_or("");

                print_char(f, indent.space.fill, indent.color)?;
                print_cell(f, text, color, &rowcfg.pad, rowcfg.alignment, width)?;
            }
        }
        None => {
            for (col, text) in iter.iter.enumerate() {
                let col = col + 1;

                let width = rowcfg.dims.get_width(col);
                let color = rowcfg.colors.get_color((iter.row, col).into());
                let text = text.as_ref();
                let text = text.lines().next().unwrap_or("");

                print_cell(f, text, color, &rowcfg.pad, rowcfg.alignment, width)?;
            }
        }
    }

    if let Some(indent) = rowcfg.verticals.right {
        print_char(f, indent.space.fill, indent.color)?;
    }

    Ok(())
}

fn print_row_columns_empty<F, D, C>(
    f: &mut F,
    rowcfg: &RowConfig<D, C>,
    color: Option<ANSIString>,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
{
    if let Some(indent) = rowcfg.verticals.left {
        print_char(f, indent.space.fill, indent.color)?;
    }

    let width = rowcfg.dims.get_width(0);
    print_indent(f, ColoredIndent::new(width, ' ', color))?;

    match rowcfg.verticals.intersection {
        Some(indent) => {
            for column in 1..rowcfg.count_columns {
                let width = rowcfg.dims.get_width(column);

                print_char(f, indent.space.fill, indent.color)?;
                print_indent(f, ColoredIndent::new(width, ' ', color))?;
            }
        }
        None => {
            for column in 1..rowcfg.count_columns {
                let width = rowcfg.dims.get_width(column);
                print_indent(f, ColoredIndent::new(width, ' ', color))?;
            }
        }
    }

    if let Some(indent) = rowcfg.verticals.right {
        print_char(f, indent.space.fill, indent.color)?;
    }

    Ok(())
}

fn print_cell<F, C>(
    f: &mut F,
    text: &str,
    color: Option<C>,
    padding: &Sides<ColoredIndent>,
    alignment: AlignmentHorizontal,
    width: usize,
) -> fmt::Result
where
    F: Write,
    C: ANSIFmt,
{
    let available = width - (padding.left.space.size + padding.right.space.size);

    let text_width = get_line_width(text);
    let (left, right) = if available > text_width {
        calculate_indent(alignment, text_width, available)
    } else {
        (0, 0)
    };

    print_indent(f, padding.left)?;

    repeat_char(f, ' ', left)?;
    print_text(f, text, color)?;
    repeat_char(f, ' ', right)?;

    print_indent(f, padding.right)?;

    Ok(())
}

fn print_split_line_colored<F, D>(
    f: &mut F,
    dimension: &D,
    borders: &HorizontalLine<char>,
    borders_colors: &HorizontalLine<ANSIString>,
    count_columns: usize,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
{
    let mut used_color = ANSIStr::default();
    let chars_main = borders.main.unwrap_or(' ');

    if let Some(c) = borders.left {
        if let Some(color) = &borders_colors.right {
            prepare_coloring(f, color, &mut used_color)?;
        }

        f.write_char(c)?;
    }

    let width = dimension.get_width(0);
    if width > 0 {
        if let Some(color) = borders_colors.main {
            prepare_coloring(f, &color, &mut used_color)?;
        }

        repeat_char(f, chars_main, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = borders.intersection {
            if let Some(color) = borders_colors.intersection {
                prepare_coloring(f, &color, &mut used_color)?;
            }

            f.write_char(c)?;
        }

        let width = dimension.get_width(col);
        if width > 0 {
            if let Some(color) = borders_colors.main {
                prepare_coloring(f, &color, &mut used_color)?;
            }

            repeat_char(f, chars_main, width)?;
        }
    }

    if let Some(c) = borders.right {
        if let Some(color) = &borders_colors.right {
            prepare_coloring(f, color, &mut used_color)?;
        }

        f.write_char(c)?;
    }

    used_color.fmt_ansi_suffix(f)?;

    Ok(())
}

fn print_split_line<F, D>(
    f: &mut F,
    dims: &D,
    chars: &HorizontalLine<char>,
    count_columns: usize,
) -> fmt::Result
where
    F: Write,
    D: Dimension,
{
    let chars_main = chars.main.unwrap_or(' ');

    if let Some(c) = chars.left {
        f.write_char(c)?;
    }

    let width = dims.get_width(0);
    if width > 0 {
        repeat_char(f, chars_main, width)?;
    }

    for col in 1..count_columns {
        if let Some(c) = chars.intersection {
            f.write_char(c)?;
        }

        let width = dims.get_width(col);
        if width > 0 {
            repeat_char(f, chars_main, width)?;
        }
    }

    if let Some(c) = chars.right {
        f.write_char(c)?;
    }

    Ok(())
}

fn print_text<F, C>(f: &mut F, text: &str, color: Option<C>) -> fmt::Result
where
    F: Write,
    C: ANSIFmt,
{
    match color {
        Some(color) => {
            color.fmt_ansi_prefix(f)?;
            f.write_str(text)?;
            color.fmt_ansi_suffix(f)?;
        }
        None => {
            f.write_str(text)?;
        }
    };

    Ok(())
}

fn prepare_coloring<F>(f: &mut F, clr: &ANSIString, used: &mut ANSIString) -> fmt::Result
where
    F: Write,
{
    if *used != *clr {
        used.fmt_ansi_suffix(f)?;
        clr.fmt_ansi_prefix(f)?;
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

fn repeat_char<F>(f: &mut F, c: char, n: usize) -> fmt::Result
where
    F: Write,
{
    for _ in 0..n {
        f.write_char(c)?;
    }

    Ok(())
}

// todo: replace Option<StaticColor> to StaticColor and check performance
fn print_char<F>(f: &mut F, c: char, color: Option<ANSIString>) -> fmt::Result
where
    F: Write,
{
    match color {
        Some(color) => {
            color.fmt_ansi_prefix(f)?;
            f.write_char(c)?;
            color.fmt_ansi_suffix(f)
        }
        None => f.write_char(c),
    }
}

fn print_indent_lines<F>(f: &mut F, indent: ColoredIndent) -> fmt::Result
where
    F: Write,
{
    print_indent(f, indent)?;
    f.write_char('\n')?;

    for _ in 1..indent.space.size {
        f.write_char('\n')?;
        print_indent(f, indent)?;
    }

    Ok(())
}

fn print_indent<F>(f: &mut F, indent: ColoredIndent) -> fmt::Result
where
    F: Write,
{
    match indent.color {
        Some(color) => {
            color.fmt_ansi_prefix(f)?;
            repeat_char(f, indent.space.fill, indent.space.size)?;
            color.fmt_ansi_suffix(f)?;
        }
        None => {
            repeat_char(f, indent.space.fill, indent.space.size)?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct ColoredIndent {
    space: Indent,
    color: Option<ANSIString>,
}

impl ColoredIndent {
    fn new(width: usize, c: char, color: Option<ANSIString>) -> Self {
        Self {
            space: Indent::new(width, c),
            color,
        }
    }

    fn from_indent(indent: Indent, color: ANSIString) -> Self {
        Self {
            space: indent,
            color: create_color(color),
        }
    }
}

fn create_color(color: ANSIString) -> Option<ANSIString> {
    if color.is_empty() {
        None
    } else {
        Some(color)
    }
}
