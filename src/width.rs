//! This module contains object which can be used to limit a cell to a given width:
//!
//! - [Truncate] cuts a cell content to limit width.
//! - [Wrap] split the content via new lines in order to fit max width.

use std::collections::HashMap;

use crate::{CellOption, TableOption};
use papergrid::{string_width, Entity, Grid, Settings};

/// MaxWidth allows you to set a max width of an object on a [Grid],
/// using different strategies.
///
/// It is an abstract factory.
///
/// ## Example
///
/// ```
/// use tabled::{Full, MaxWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(MaxWidth::truncating(3).suffix("...")));
/// ```
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
/// use tabled::{Full, Truncate, Modify, Table};
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
/// use tabled::{Full, Wrap, Modify, Table};
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
        let wrapped_content = if !self.keep_words {
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
        if i != 0 && i % width == 0 {
            let prev_c = buf.chars().last().unwrap();
            let is_splitting_word = !prev_c.is_whitespace() && !c.is_whitespace();
            if is_splitting_word {
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
                            buf.insert(buf.len() - range_len, '\n');
                            i = range_len;
                        } else {
                            // The words is too long to be moved,
                            // we can't move it any way so just leave everything as it is
                            buf.push('\n');
                        }
                    }
                    None => {
                        // We don't find a whitespace
                        // so its a long word so we can do nothing about it
                        buf.push('\n');
                    }
                }
            } else {
                // This place doesn't separate a word
                // So we just do a general split.
                buf.push('\n');
            }
        }

        buf.push(c);

        i += 1;
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
        if is_splitting_word {
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
        } else {
            buf.push_str(&lhs);
            buf.push('\n');
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
/// It does anything in case if the content's length is bigger then the boundry.
/// It doesn't include a [crate::Indent] settings.
///
/// ## Example
///
/// ```
/// use tabled::{Full, MinWidth, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Full).with(MinWidth(10)));
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
                if length < width {
                    let remain = width - length;
                    let mut new_line = String::with_capacity(width);
                    new_line.push_str(line);
                    new_line.extend(std::iter::repeat(fill_with).take(remain));
                    std::borrow::Cow::Owned(new_line)
                } else {
                    std::borrow::Cow::Borrowed(line)
                }
            })
            .collect::<String>()
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
            .collect::<String>()
    }
}

pub struct TotalWidth {
    size: usize,
}

impl TotalWidth {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl TableOption for TotalWidth {
    fn change(&mut self, grid: &mut Grid) {
        let total_width = grid.total_width();
        let mut size = self.size;
        if size == total_width {
            return;
        }

        if size > total_width {
            // inc each column 1 by 1

            let mut increase_list = HashMap::new();

            for col in (0..grid.count_columns()).cycle() {
                if size == total_width {
                    break;
                }

                let mut increased = false;
                for row in 0..grid.count_rows() {
                    let style = grid.style(&Entity::Cell(row, col));
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

            return;
        }

        // dec the biggest cells 1 by 1
        let mut biggest = Vec::new();
        while size != total_width {
            println!("size={} total_width={}", size, total_width);

            match find_the_biggest_cell_by_width(grid, &mut biggest) {
                Some(width) => {
                    if width == 0 {
                        break;
                    }

                    println!("+++++ width={} {:?}", width, biggest);

                    for &(row, col) in &biggest {
                        Truncate::new(width - 1).change_cell(grid, row, col)
                    }

                    biggest.clear();
                }
                None => break,
            }

            size += 1;
        }
    }
}

fn find_the_biggest_cell_by_width(grid: &Grid, cells: &mut Vec<(usize, usize)>) -> Option<usize> {
    let mut max_row = 0;
    let mut max_col = 0;
    let mut max_width = 0;
    let mut max_span = 0;

    for col in 0..grid.count_columns() {
        for row in 0..grid.count_rows() {
            let style = grid.style(&Entity::Cell(row, col));
            if style.span == 0 {
                continue;
            }

            let content = grid.get_cell_content(row, col);
            let content_width = string_width(content);

            // let this_col_width =
            //     (col..col + max_span)
            //         .filter(|&c| c < grid.count_columns())
            //         .map(|column| {
            //             let content = grid.get_cell_content(max_row, column);
            //             let content_width = string_width(content);
            //             content_width
            //         })
            //         .sum::<usize>();

            // // need to compare with different spans
            // let max_overall_width = if max_is_set {
            //     (max_col..max_col + style.span)
            //         .filter(|&c| c < grid.count_columns())
            //         .map(|column| {
            //             let content = grid.get_cell_content(max_row, column);
            //             let content_width = string_width(content);
            //             content_width
            //         })
            //         .sum::<usize>()
            // } else {
            //     0
            // };

            if content_width > max_width {
                cells.clear();
                cells.push((row, col));

                max_col = col;
                max_row = row;
                max_width = content_width;
            } else if content_width == max_width && max_col == col {
                cells.push((row, col));
            }
        }
    }

    if grid.count_columns() == 0 || grid.count_rows() == 0 {
        return None;
    }

    Some(max_width)
}
