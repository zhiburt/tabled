//! This module contains object which can be used to limit a cell to a given width:
//!
//! - [Truncate] cuts a cell content to limit width.
//! - [Wrap] split the content via new lines in order to fit max width.
//! - [Justify] sets columns width to the same value.
//!
//! To set a a table width a combination of [MaxWidth] and [MinWidth] can be set.
//!
//! ## Example
//!
//! ```
//! use tabled::{MaxWidth, MinWidth, Table};
//!
//! let table = Table::new(&["Hello World!"])
//!     .with(MaxWidth::wrapping(5))
//!     .with(MinWidth::new(5));
//! ```
//!

use std::collections::{HashMap, HashSet};

use crate::{CellOption, TableOption};
use papergrid::{string_width, Entity, Grid, Margin, Settings, Style};

/// MaxWidth allows you to set a max width of an object on a [Grid],
/// using different strategies.
/// It also allows you to set a MaxWidth for a whole table.
///
/// It is an abstract factory.
///
/// Beware that borders are not removed when you set a size value to very small.
/// For example if you set size to 0 the table still be rendered but with all content removed.
///
/// Also be aware that it doesn't changes [crate::Padding] settings.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Examples
///
/// Cell change
///
/// ```
/// use tabled::{object::Full, MaxWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(MaxWidth::truncating(3).suffix("...")));
/// ```
///
/// Table change
///
/// ```
/// use tabled::{MaxWidth, Table};
///
/// let table = Table::new(&["Hello World!"]).with(MaxWidth::wrapping(5));
/// ```
///
pub struct MaxWidth;

impl MaxWidth {
    /// Returns a [Truncate] object.
    pub fn truncating(width: usize) -> Truncate<&'static str> {
        Truncate::new(width)
    }

    /// Returns a [Wrap] object.
    pub fn wrapping(width: usize) -> Wrap {
        Wrap::new(width)
    }
}

/// Truncate cut the string to a given width if its length exeeds it.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///    
/// ## Example
///
/// ```
/// use tabled::{object::Full, Truncate, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Full).with(Truncate::new(3)));
/// ```
pub struct Truncate<S> {
    width: usize,
    suffix: S,
}

impl Truncate<&'static str> {
    /// Creates a [Truncate] object
    pub fn new(width: usize) -> Self {
        Self { width, suffix: "" }
    }
}

impl<T> Truncate<T> {
    /// Sets a suffix which will be appended to a resultant string
    /// in case a truncate is applied.
    pub fn suffix<S>(self, suffix: S) -> Truncate<S> {
        Truncate {
            width: self.width,
            suffix,
        }
    }
}

impl<S> CellOption for Truncate<S>
where
    S: AsRef<str>,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let striped_content = strip(content, self.width);
        if striped_content.len() < content.len() {
            let new_content = format!("{}{}", striped_content, self.suffix.as_ref());
            grid.set(
                &Entity::Cell(row, column),
                Settings::new().text(new_content),
            )
        }
    }
}

/// Wrap wraps a string to a new line in case it exeeds the provided max boundry.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Example
///
/// ```
/// use tabled::{object::Full, Wrap, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Full).with(Wrap::new(3)));
/// ```
pub struct Wrap {
    width: usize,
    keep_words: bool,
}

impl Wrap {
    /// Creates a [Wrap] object
    pub fn new(width: usize) -> Self {
        Self {
            width,
            keep_words: false,
        }
    }

    /// Set the keep words option.
    ///
    /// If a wrapping poing will be in a word, [Wrap] will
    /// preserve a word (if possible) and wrap the string before it.
    pub fn keep_words(mut self) -> Self {
        self.keep_words = true;
        self
    }
}

impl CellOption for Wrap {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let wrapped_content = if self.width == 0 {
            String::new()
        } else if !self.keep_words {
            split(content, self.width)
        } else {
            split_keeping_words(content, self.width)
        };

        grid.set(
            &Entity::Cell(row, column),
            Settings::new().text(wrapped_content),
        )
    }
}

pub(crate) fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let width = to_byte_length(s, width);
        ansi_str::AnsiStr::ansi_cut(s, ..width)
    }
}

pub(crate) fn split(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % width == 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        if width == 0 {
            s.to_string()
        } else {
            chunks(s, width).join("\n")
        }
    }
}

#[cfg(not(feature = "color"))]
fn split_keeping_words(s: &str, width: usize) -> String {
    let mut buf = String::new();
    let mut i = 0;
    for c in s.chars() {
        let is_splitting_pos = i == width;
        if !is_splitting_pos {
            i += 1;
            buf.push(c);
            continue;
        }

        i = 1;

        let prev_c = buf.chars().last().unwrap();
        let is_splitting_word = !prev_c.is_whitespace() && !c.is_whitespace();
        if !is_splitting_word {
            // This place doesn't separate a word
            // So we just do a general split.
            buf.push('\n');
            buf.push(c);
            continue;
        }

        let pos = buf.chars().rev().position(|c| c.is_whitespace());
        match pos {
            Some(pos) => {
                if pos < width {
                    // it's a part of a word which we is ok to move to the next line;
                    // we know that there will be enough space for this part + next character.
                    //
                    // todo: test about this next char space
                    let range_len = buf
                        .chars()
                        .rev()
                        .take(pos)
                        .map(|c| c.len_utf8())
                        .sum::<usize>();

                    // put an spaces in order to not limit widths and keep it correct.
                    for i in 0..range_len {
                        buf.insert(buf.len() - range_len - i, ' ');
                    }

                    buf.insert(buf.len() - range_len, '\n');

                    i = range_len + 1;
                } else {
                    // The words is too long to be moved,
                    // we can't move it any way so just leave everything as it is
                    buf.push('\n');
                }

                buf.push(c);
            }
            None => {
                // We don't find a whitespace
                // so its a long word so we can do nothing about it
                buf.push('\n');
                buf.push(c);
            }
        }
    }

    buf
}

#[cfg(feature = "color")]
fn split_keeping_words(s: &str, width: usize) -> String {
    use ansi_str::AnsiStr;

    let mut buf = String::new();
    let mut s = s.to_string();
    while !s.is_empty() {
        let width = to_byte_length(&s, width);
        let (mut lhs, mut rhs) = s.ansi_split_at(width);

        let lhs_stripped = lhs.ansi_strip();
        let left_ends_with_letter = lhs_stripped
            .chars()
            .last()
            .map_or(false, |c| !c.is_whitespace());
        let right_starts_with_letter = rhs
            .ansi_strip()
            .chars()
            .next()
            .map_or(false, |c| !c.is_whitespace());

        let is_splitting_word = left_ends_with_letter && right_starts_with_letter;
        if !is_splitting_word {
            buf.push_str(&lhs);
            buf.push('\n');
            s = rhs;
            continue;
        }

        let pos = lhs_stripped.chars().rev().position(|c| c.is_whitespace());
        match pos {
            Some(pos) => {
                if pos < width {
                    // it's a part of a word which we is ok to move to the next line;
                    // we know that there will be enough space for this part + next character.
                    //
                    // todo: test about this next char space
                    let range_len = lhs_stripped
                        .chars()
                        .rev()
                        .take(pos)
                        .map(|c| c.len_utf8())
                        .sum::<usize>();

                    let move_part = lhs.ansi_get(lhs_stripped.len() - range_len..).unwrap();
                    lhs = lhs.ansi_get(..lhs_stripped.len() - range_len).unwrap();
                    rhs = move_part + &rhs;

                    // put an spaces in order to not limit widths and keep it correct.
                    lhs.extend(std::iter::repeat(' ').take(range_len));

                    buf.push_str(&lhs);
                    buf.push('\n');
                } else {
                    // The words is too long to be moved,
                    // we can't move it any way so just leave everything as it is
                    buf.push_str(&lhs);
                    buf.push('\n');
                }
            }
            None => {
                // We don't find a whitespace
                // so its a long word so we can do nothing about it
                buf.push_str(&lhs);
                buf.push('\n');
            }
        }

        s = rhs;
    }

    buf
}

#[cfg(feature = "color")]
fn to_byte_length(s: &str, width: usize) -> usize {
    s.chars().take(width).map(|c| c.len_utf8()).sum::<usize>()
}

#[cfg(feature = "color")]
fn chunks(s: &str, width: usize) -> Vec<String> {
    use ansi_str::AnsiStr;

    let mut v = Vec::new();
    let mut s = s.to_string();
    while !s.is_empty() {
        let width = to_byte_length(&s, width);
        let (lhs, rhs) = s.ansi_split_at(width);
        s = rhs;
        v.push(lhs);
    }

    v
}

/// MinWidth changes a content in case if it's length is lower then the boundry.
///
/// It can be applied to a whole table.
///
/// It does anything in case if the content's length is bigger then the boundry.
/// It doesn't include a [crate::Padding] settings.
///
/// ## Examples
///
/// Cell change
///
/// ```
/// use tabled::{object::Full, MinWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(MinWidth::new(10)));
/// ```
/// Table change
///
/// ```
/// use tabled::{MinWidth, Table};
///
/// let table = Table::new(&["Hello World!"]).with(MinWidth::new(5));
/// ```
pub struct MinWidth {
    size: usize,
    fill: char,
}

impl MinWidth {
    /// Creates a new instance of MinWidth.
    pub fn new(size: usize) -> Self {
        Self { size, fill: ' ' }
    }

    /// Set's a fill character which will be used to fill the space
    /// when increasing the length of the string to the set boundry.
    pub fn fill_with(mut self, c: char) -> Self {
        self.fill = c;
        self
    }
}

impl CellOption for MinWidth {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let new_content = increase_width(content, self.size, self.fill);
        grid.set(
            &Entity::Cell(row, column),
            Settings::new().text(new_content),
        )
    }
}

fn increase_width(s: &str, width: usize, fill_with: char) -> String {
    let has_big_lines = s.lines().any(|line| string_width(line) < width);
    if !has_big_lines {
        return s.to_owned();
    }

    #[cfg(not(feature = "color"))]
    {
        s.lines()
            .map(|line| {
                let length = string_width(line);
                if width > length {
                    let remain = width - length;
                    let mut new_line = String::with_capacity(width);
                    new_line.push_str(line);
                    new_line.extend(std::iter::repeat(fill_with).take(remain));
                    std::borrow::Cow::Owned(new_line)
                } else {
                    std::borrow::Cow::Borrowed(line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[cfg(feature = "color")]
    {
        ansi_str::AnsiStr::ansi_split(s, "\n")
            .map(|mut line| {
                let length = string_width(&line);
                if length < width {
                    let remain = width - length;
                    line.extend(std::iter::repeat(fill_with).take(remain));
                    line
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<S> TableOption for Truncate<S>
where
    S: AsRef<str>,
{
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        let total_width = grid.total_width();
        if total_width == self.width {
            return;
        }

        if self.width < total_width {
            truncate_total_width(grid, self.width);
        }
    }
}

impl TableOption for Wrap {
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        let total_width = grid.total_width();
        if total_width == self.width {
            return;
        }

        if self.width < total_width {
            wrap_total_width(grid, self.width, self.keep_words);
        }
    }
}

impl TableOption for MinWidth {
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        // loop is neccessary because increase_total_width may not work properly in 1 call.
        //
        // todo: Try to fix it in other way?
        loop {
            let total_width = grid.total_width();
            if total_width >= self.size {
                break;
            }

            increase_total_width(grid, total_width, self.size);
        }
    }
}

fn increase_total_width(grid: &mut Grid, total_width: usize, expected_width: usize) {
    let (_, styles) = grid.build_widths();

    let mut increase_list = HashMap::new();
    let mut size = expected_width;
    for col in (0..grid.count_columns()).cycle() {
        if size == total_width {
            break;
        }

        let mut increased = false;
        #[allow(clippy::needless_range_loop)]
        for row in 0..grid.count_rows() {
            let style = &styles[row][col];
            if style.span == 0 {
                continue;
            }

            increase_list
                .entry((row, col))
                .and_modify(|e| *e += 1)
                .or_insert(1);

            increased = true;
        }

        if increased {
            size -= 1;
        }
    }

    for ((row, col), inc) in increase_list {
        let content = grid.get_cell_content(row, col);
        let content_width = string_width(content);

        MinWidth::new(content_width + inc).change_cell(grid, row, col);
    }
}

fn truncate_total_width(grid: &mut Grid, width: usize) {
    let points = decrease_total_width(grid, width);

    for ((row, col), width) in points {
        Truncate::new(width).change_cell(grid, row, col);
    }
}

fn wrap_total_width(grid: &mut Grid, width: usize, keep_words: bool) {
    let points = decrease_total_width(grid, width);

    let mut wrap = Wrap::new(0);
    wrap.keep_words = keep_words;
    for ((row, col), width) in points {
        wrap.width = width;
        wrap.change_cell(grid, row, col);
    }
}

fn decrease_total_width(grid: &Grid, width: usize) -> HashMap<(usize, usize), usize> {
    let mut points = HashMap::new();

    let count_columns = grid.count_columns();
    let count_rows = grid.count_rows();

    if count_columns == 0 || count_rows == 0 {
        return points;
    }

    let orig_widths = grid_widths(grid);

    let mut min_widths = build_min_widths(grid);

    let (mut widths, styles) = grid.build_widths();

    correct_widths(&mut widths, &styles, count_rows, count_columns);
    correct_widths(&mut min_widths, &styles, count_rows, count_columns);

    let borders = build_borders_list(grid, &styles, count_rows, count_columns);

    let mut total_width = new_total_width(
        &widths,
        &styles,
        &borders,
        grid.get_margin(),
        count_rows,
        count_columns,
    );

    let mut empty_columns = HashSet::new();
    let mut columns = (0..count_columns).cycle();
    while total_width != width {
        let is_zeroed_table = empty_columns.len() == count_columns;
        if is_zeroed_table {
            break;
        }

        let col = columns.next().unwrap();

        if empty_columns.contains(&col) {
            continue;
        }

        let is_empty_column = (0..count_rows).all(|row| widths[row][col] == 0);
        if is_empty_column {
            empty_columns.insert(col);
            continue;
        }

        update_widths_column(&mut widths, &orig_widths, &styles, count_rows, col);

        total_width = new_total_width(
            &widths,
            &styles,
            &borders,
            grid.get_margin(),
            count_rows,
            count_columns,
        );
    }

    for col in 0..count_columns {
        for row in 0..count_rows {
            let width = std::cmp::max(widths[row][col], min_widths[row][col]);
            let orig_width = orig_widths[row][col];

            if width < orig_width {
                points.insert((row, col), width);
            }
        }
    }

    points
}

fn new_total_width(
    widths: &[Vec<usize>],
    styles: &[Vec<Style>],
    count_borders: &[usize],
    margin: &Margin,
    count_rows: usize,
    count_columns: usize,
) -> usize {
    (0..count_rows)
        .map(|row| {
            (0..count_columns)
                .filter(|&col| styles[row][col].span > 0)
                .map(|col| {
                    let padding = &styles[row][col].padding;
                    widths[row][col] + padding.left.size + padding.right.size
                })
                .sum::<usize>()
                + count_borders[row]
        })
        .max()
        .unwrap_or(0)
        + margin.left.size
        + margin.right.size
}

fn build_borders_list(
    grid: &Grid,
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<usize> {
    let mut borders_count = Vec::with_capacity(count_rows);

    #[allow(clippy::needless_range_loop)]
    for row in 0..count_rows {
        let mut count = 0;
        for col in 0..count_columns {
            if styles[row][col].span == 0 {
                continue;
            }

            let border = grid.get_border(row, col);
            if border.left.is_some() {
                count += 1;
            }

            if col + 1 == count_columns && border.right.is_some() {
                count += 1;
            }
        }

        borders_count.push(count);
    }

    borders_count
}

fn correct_widths(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) {
    (0..count_rows).for_each(|row| {
        (0..count_columns)
            .for_each(|col| widths[row][col] = correct_width(&styles[row][col], widths[row][col]))
    });
}

fn build_min_widths(grid: &Grid) -> Vec<Vec<usize>> {
    let mut grid = grid.clone();
    grid.set(&Entity::Global, Settings::default().text(""));

    grid.build_widths().0
}

fn update_widths_column(
    widths: &mut [Vec<usize>],
    orig_widths: &[Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    col: usize,
) -> bool {
    let mut some_content_was_changed = false;
    for row in 0..count_rows {
        let mut col = col;
        while styles[row][col].span == 0 {
            // todo:
            // well it may happen, in a grid with all cells being with span 0.
            // not sure how to handle it in a good way.
            if col == 0 {
                break;
            }

            col -= 1;
        }

        if widths[row][col] == 0 {
            continue;
        }

        widths[row][col] -= 1;

        let orig_content_was_changed = orig_widths[row][col] > widths[row][col];
        if orig_content_was_changed {
            some_content_was_changed = true;
        }
    }

    some_content_was_changed
}

fn correct_width(style: &Style, mut width: usize) -> usize {
    let mut padding = style.padding.left.size + style.padding.right.size;
    while padding != 0 {
        if width == 0 {
            break;
        }

        width -= 1;
        padding -= 1;
    }

    width
}

fn grid_widths(grid: &Grid) -> Vec<Vec<usize>> {
    (0..grid.count_rows())
        .map(|row| {
            (0..grid.count_columns())
                .map(|col| {
                    let content = grid.get_cell_content(row, col);
                    string_width(content)
                })
                .collect()
        })
        .collect()
}

fn is_zero_spanned_grid(grid: &Grid) -> bool {
    (0..grid.count_rows())
        .map(|row| {
            (0..grid.count_columns()).all(|col| grid.style(&Entity::Cell(row, col)).span == 0)
        })
        .all(|b| b)
}

/// Justify sets all columns widths to the set value.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [crate::Padding] to set it to 0.
///
/// ## Example
///
/// ```
/// use tabled::{Justify, Style, Modify, Full, Padding, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(Padding::zero()))
///     .with(Justify::new(3));
/// ```
pub struct Justify {
    width: usize,
}

impl Justify {
    /// Creates a new Justify instance.
    ///
    /// Be aware that [crate::Padding] is not considered when comparing the width.
    pub fn new(width: usize) -> Self {
        Self { width }
    }
}

impl TableOption for Justify {
    fn change(&mut self, grid: &mut Grid) {
        let width = self.width;

        for row in 0..grid.count_rows() {
            for col in 0..grid.count_columns() {
                MinWidth::new(width).change_cell(grid, row, col);
                MaxWidth::truncating(width).change_cell(grid, row, col);
            }
        }
    }
}
