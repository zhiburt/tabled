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
//!     .with(MaxWidth::wrapping(7))
//!     .with(MinWidth::new(7))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+-----+\n",
//!         "| &st |\n",
//!         "| r   |\n",
//!         "+-----+\n",
//!         "| Hel |\n",
//!         "| lo  |\n",
//!         "| Wor |\n",
//!         "| ld! |\n",
//!         "+-----+\n",
//!     )
//! );
//! ```

use std::collections::{HashMap, HashSet};

use crate::{CellOption, TableOption};
use papergrid::{string_width, string_width_multiline, Entity, Grid, Settings, Style};

/// MaxWidth allows you to set a max width of an object on a [Table],
/// using different strategies.
/// It also allows you to set a MaxWidth for a whole table.
///
/// It is an abstract factory.
///
/// Beware that borders are not removed when you set a size value to very small.
/// For example if you set size to 0 the table still be rendered but with all content removed.
///
/// Also be aware that it doesn't changes [Padding] settings.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Examples
///
/// Cell change
///
/// ```
/// use tabled::{object::Segment, MaxWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Segment::all()).with(MaxWidth::truncating(3).suffix("...")));
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
/// [Padding]: crate::Padding
/// [Table]: crate::Table
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

/// Truncate cut the string to a given width if its length exceeds it.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///    
/// ## Example
///
/// ```
/// use tabled::{object::Segment, Truncate, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Truncate::new(3)));
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
        let content = grid.get_cell_content_styled(row, column);
        let striped_content = strip(&content, self.width);
        if striped_content.len() < content.len() {
            let new_content = format!("{}{}", striped_content, self.suffix.as_ref());
            grid.set(Entity::Cell(row, column), Settings::new().text(new_content))
        }
    }
}

/// Wrap wraps a string to a new line in case it exceeds the provided max boundary.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Example
///
/// ```
/// use tabled::{object::Segment, Wrap, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Wrap::new(3)));
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
    /// If a wrapping point will be in a word, [Wrap] will
    /// preserve a word (if possible) and wrap the string before it.
    pub fn keep_words(mut self) -> Self {
        self.keep_words = true;
        self
    }
}

impl CellOption for Wrap {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content_styled(row, column);
        let wrapped_content = if self.width == 0 {
            String::new()
        } else if !self.keep_words {
            split(&content, self.width)
        } else {
            split_keeping_words(&content, self.width)
        };

        grid.set(
            Entity::Cell(row, column),
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

/// MinWidth changes a content in case if it's length is lower then the boundary.
///
/// It can be applied to a whole table.
///
/// It does anything in case if the content's length is bigger then the boundary.
/// It doesn't include a [Padding] settings.
///
/// ## Examples
///
/// Cell change
///
/// ```
/// use tabled::{object::Segment, MinWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Segment::all()).with(MinWidth::new(10)));
/// ```
/// Table change
///
/// ```
/// use tabled::{MinWidth, Table};
///
/// let table = Table::new(&["Hello World!"]).with(MinWidth::new(5));
/// ```
///
/// [Padding]: crate::Padding
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
    /// when increasing the length of the string to the set boundary.
    pub fn fill_with(mut self, c: char) -> Self {
        self.fill = c;
        self
    }
}

impl CellOption for MinWidth {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content_styled(row, column);
        let new_content = increase_width(&content, self.size, self.fill);
        grid.set(Entity::Cell(row, column), Settings::new().text(new_content))
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

        // loop is necessary because increase_total_width may not work properly in 1 call.
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
    let mut increase_list = HashMap::new();
    let mut size = expected_width;
    for col in (0..grid.count_columns()).cycle() {
        if size == total_width {
            break;
        }

        let mut increased = false;
        #[allow(clippy::needless_range_loop)]
        for row in 0..grid.count_rows() {
            if !grid.is_cell_visible((row, col)) {
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
        let content = grid.get_cell_content_styled(row, col);
        let content_width = string_width_multiline(&content);

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
    let min_widths = build_min_widths(grid);
    let borders = build_borders_list(grid);

    let mut empty_columns = HashSet::new();
    let mut widths = build_cell_widths(grid);
    let mut total_width = new_total_width(grid, &widths, &borders);
    let mut columns = (0..count_columns).cycle();
    while total_width != width {
        // all cells are zero width.
        let reached_the_end = empty_columns.len() == count_columns;
        if reached_the_end {
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

        update_widths_column(grid, &orig_widths, &mut widths, col);

        total_width = new_total_width(grid, &widths, &borders);
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

fn new_total_width(grid: &Grid, widths: &[Vec<usize>], borders: &[usize]) -> usize {
    (0..grid.count_rows())
        .map(|row| {
            (0..grid.count_columns())
                .filter(|&col| grid.is_cell_visible((row, col)))
                .map(|col| {
                    let padding = &grid.style(Entity::Cell(row, col)).padding;
                    widths[row][col] + padding.left.size + padding.right.size
                })
                .sum::<usize>()
                + borders[row]
        })
        .max()
        .unwrap_or(0)
        + grid.get_margin().left.size
        + grid.get_margin().right.size
}

fn build_borders_list(grid: &Grid) -> Vec<usize> {
    let mut borders_count = Vec::with_capacity(grid.count_rows());

    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.count_rows() {
        let mut count = 0;
        for col in 0..grid.count_columns() {
            let border = grid.get_border(row, col);

            if grid.is_cell_visible((row, col)) && border.left.is_some() {
                count += 1;
            }

            if col + 1 == grid.count_columns() && border.right.is_some() {
                count += 1;
            }
        }

        borders_count.push(count);
    }

    borders_count
}

fn build_min_widths(grid: &Grid) -> Vec<Vec<usize>> {
    let mut grid = grid.clone();
    grid.set(Entity::Global, Settings::default().text(""));

    build_cell_widths(&grid)
}

fn build_cell_widths(grid: &Grid) -> Vec<Vec<usize>> {
    let mut widths = grid.build_cells_widths();
    correct_widths(grid, &mut widths);
    widths
}

fn correct_widths(grid: &Grid, widths: &mut [Vec<usize>]) {
    (0..grid.count_rows()).for_each(|row| {
        (0..grid.count_columns()).for_each(|col| {
            widths[row][col] = correct_width(grid.style(Entity::Cell(row, col)), widths[row][col])
        })
    });
}

fn update_widths_column(
    grid: &Grid,
    orig_widths: &[Vec<usize>],
    widths: &mut [Vec<usize>],
    col: usize,
) -> bool {
    let mut some_content_was_changed = false;
    for row in 0..grid.count_rows() {
        let mut col = col;
        while !grid.is_cell_visible((row, col)) {
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

fn correct_width(style: &Style, width: usize) -> usize {
    let padding = style.padding.left.size + style.padding.right.size;
    width.saturating_sub(padding)
}

fn grid_widths(grid: &Grid) -> Vec<Vec<usize>> {
    (0..grid.count_rows())
        .map(|row| {
            (0..grid.count_columns())
                .map(|col| {
                    let content = grid.get_cell_content_styled(row, col);
                    string_width_multiline(&content)
                })
                .collect()
        })
        .collect()
}

fn is_zero_spanned_grid(grid: &Grid) -> bool {
    (0..grid.count_rows())
        .all(|row| (0..grid.count_columns()).all(|col| !grid.is_cell_visible((row, col))))
}

/// Justify sets all columns widths to the set value.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [Padding] to set it to 0.
///
/// ## Examples
///
/// ```
/// use tabled::{Justify, Style, Modify, object::Segment, Padding, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Segment::all()).with(Padding::zero()))
///     .with(Justify::new(3));
/// ```
///
/// [Max] usage to justify by a max column width.
///
/// ```
/// use tabled::{Justify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Justify::max());
/// ```
///
/// [Padding]: crate::Padding
pub struct Justify<W> {
    width: W,
}

impl Justify<usize> {
    /// Creates a new Justify instance.
    ///
    /// Be aware that [Padding] is not considered when comparing the width.
    ///
    /// [Padding]: crate::Padding
    pub fn new(width: usize) -> Self {
        Self { width }
    }
}

impl Justify<Max> {
    /// Creates a new Justify instance with a Max width used as a value.
    pub fn max() -> Self {
        Self { width: Max }
    }
}

impl Justify<Min> {
    /// Creates a new Justify instance with a Min width used as a value.
    pub fn min() -> Self {
        Self { width: Min }
    }
}

impl<W> TableOption for Justify<W>
where
    W: Width,
{
    fn change(&mut self, grid: &mut Grid) {
        let width = self.width.width(grid);

        for row in 0..grid.count_rows() {
            for col in 0..grid.count_columns() {
                MinWidth::new(width).change_cell(grid, row, col);
                MaxWidth::truncating(width).change_cell(grid, row, col);
            }
        }
    }
}

/// A width value which can be obtained on behalf of [Table].
///
/// [Table]: crate::Table
trait Width {
    /// Returns a width value.
    fn width(&self, grid: &Grid) -> usize;
}

impl Width for usize {
    fn width(&self, _: &Grid) -> usize {
        *self
    }
}

/// Max width value.
pub struct Max;

impl Width for Max {
    fn width(&self, grid: &Grid) -> usize {
        grid_widths(grid)
            .into_iter()
            .map(|r| r.into_iter().max().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }
}

/// Min width value.
pub struct Min;

impl Width for Min {
    fn width(&self, grid: &Grid) -> usize {
        grid_widths(grid)
            .into_iter()
            .map(|r| r.into_iter().min().unwrap_or(0))
            .min()
            .unwrap_or(0)
    }
}

#[cfg(feature = "color")]
#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::{colors::Yellow, OwoColorize};

    #[test]
    fn test_color_strip() {
        let s = "Collored string"
            .fg::<Yellow>()
            .on_truecolor(12, 200, 100)
            .blink()
            .to_string();
        assert_eq!(
            strip(&s, 1),
            "\u{1b}[5m\u{1b}[48;2;12;200;100m\u{1b}[33mC\u{1b}[25m\u{1b}[39m\u{1b}[49m"
        )
    }
}
