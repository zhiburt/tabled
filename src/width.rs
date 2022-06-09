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
//! use tabled::{Width, Table};
//!
//! let table = Table::new(&["Hello World!"])
//!     .with(Width::wrap(7))
//!     .with(Width::increase(7))
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

use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

use papergrid::{
    count_borders_in_range, cut_str, string_width, string_width_multiline, wrap_text, Entity, Grid,
    Settings,
};

use crate::{CellOption, TableOption};

/// Width allows you to set a min and max width of an object on a [Table]
/// using different strategies.
///
/// It also allows you to set a min and max width for a whole table.
///
/// You can apply a min and max strategy at the same time with the same value,
/// the value will be a total table width.
///
/// It is an abstract factory.
///
/// Beware that borders are not removed when you set a size value to very small.
/// For example if you set size to 0 the table still be rendered but with all content removed.
///
/// Also be aware that it doesn't changes [Padding] settings nor it considers them.
///
/// The function is color aware if a `color` feature is on.
///
/// ## Examples
///
/// ### Cell change
///
/// ```
/// use tabled::{object::Segment, Width, Modify, Style, Table};
///
/// let data = ["Hello", "World", "!"];
///
/// let table = Table::new(&data)
///     .with(Style::github_markdown())
///     .with(Modify::new(Segment::all()).with(Width::truncate(3).suffix("...")));
/// ```
///
/// ### Table change
///
/// ```
/// use tabled::{Width, Table};
///
/// let table = Table::new(&["Hello World!"]).with(Width::wrap(5));
/// ```
///
/// ### Total width
///
/// ```
/// use tabled::{Width, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Width::wrap(5))
///     .with(Width::increase(5));
/// ```
///
/// [Padding]: crate::Padding
/// [Table]: crate::Table
pub struct Width;

impl Width {
    /// Returns a [Wrap] structure.
    pub fn wrap<W>(width: W) -> Wrap<W>
    where
        W: WidthValue,
    {
        Wrap::new(width)
    }

    /// Returns a [Truncate] structure.
    pub fn truncate<W>(width: W) -> Truncate<'static, W>
    where
        W: WidthValue,
    {
        Truncate::new(width)
    }

    /// Returns a [MinWidth] structure.
    pub fn increase<W>(width: W) -> MinWidth<W>
    where
        W: WidthValue,
    {
        MinWidth::new(width)
    }

    /// Returns a [Justify] structure.
    pub fn justify<W>(width: W) -> Justify<W>
    where
        W: WidthValue,
    {
        Justify::new(width)
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
/// use tabled::{object::Segment, width::Truncate, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Truncate::new(3)));
/// ```
pub struct Truncate<'a, W = usize, P = PriorityNone> {
    width: W,
    suffix: Cow<'a, str>,
    _priority: PhantomData<P>,
}

impl<W> Truncate<'static, W>
where
    W: WidthValue,
{
    /// Creates a [Truncate] object
    pub fn new(width: W) -> Truncate<'static, W> {
        Self {
            width,
            suffix: Cow::Borrowed(""),
            _priority: Default::default(),
        }
    }
}

impl<W, P> Truncate<'_, W, P> {
    /// Sets a suffix which will be appended to a resultant string
    /// in case a truncate is applied.
    pub fn suffix<'a, S: Into<Cow<'a, str>>>(self, suffix: S) -> Truncate<'a, W, P> {
        Truncate {
            width: self.width,
            suffix: suffix.into(),
            _priority: Default::default(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Priority defines the logic by which a truncate will be applied when is done for the whole table.
    ///
    /// - [PriorityNone] which cuts the columns one after another.
    /// - [PriorityMax] cuts the biggest columns first.
    /// - [PriorityMin] cuts the lowest columns first.
    pub fn priority<PP: ColumnPeaker>(self) -> Truncate<'a, W, PP> {
        Truncate {
            width: self.width,
            suffix: self.suffix,
            _priority: Default::default(),
        }
    }
}

impl<W, P> CellOption for Truncate<'_, W, P>
where
    W: WidthValue,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let width = self.width.width(grid);

        let content = grid.get_cell_content_styled(row, column);
        let striped_content = cut_str(&content, width);
        if string_width(&striped_content) < string_width(&content) {
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
/// use tabled::{object::Segment, width::Wrap, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Wrap::new(3)));
/// ```
#[derive(Debug, Clone)]
pub struct Wrap<W = usize, P = PriorityNone> {
    width: W,
    keep_words: bool,
    _priority: PhantomData<P>,
}

impl<W> Wrap<W>
where
    W: WidthValue,
{
    /// Creates a [Wrap] object
    pub fn new(width: W) -> Self {
        Self {
            width,
            keep_words: false,
            _priority: Default::default(),
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

impl<W, P> Wrap<W, P> {
    /// Priority defines the logic by which a truncate will be applied when is done for the whole table.
    ///
    /// - [PriorityNone] which cuts the columns one after another.
    /// - [PriorityMax] cuts the biggest columns first.
    /// - [PriorityMin] cuts the lowest columns first.
    pub fn priority<PP>(self) -> Wrap<W, PP> {
        Wrap {
            width: self.width,
            keep_words: self.keep_words,
            _priority: Default::default(),
        }
    }
}

impl<W> CellOption for Wrap<W>
where
    W: WidthValue,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let width = self.width.width(grid);
        let content = grid.get_cell_content_styled(row, column);

        let wrapped_content = wrap_text(&content, width, self.keep_words);
        assert!(
            width >= string_width_multiline(&wrapped_content),
            "width{:?}\n\n content={:?}\n\n wrap={:?}\n",
            width,
            content,
            wrapped_content
        );

        grid.set(
            Entity::Cell(row, column),
            Settings::new().text(wrapped_content),
        )
    }
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
/// use tabled::{object::Segment, width::MinWidth, Modify, Style, Table};
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
/// use tabled::{width::MinWidth, Table};
///
/// let table = Table::new(&["Hello World!"]).with(MinWidth::new(5));
/// ```
///
/// [Padding]: crate::Padding
pub struct MinWidth<W = usize, P = PriorityNone> {
    size: W,
    fill: char,
    _priority: PhantomData<P>,
}

impl<W> MinWidth<W>
where
    W: WidthValue,
{
    /// Creates a new instance of MinWidth.
    pub fn new(size: W) -> Self {
        Self {
            size,
            fill: ' ',
            _priority: Default::default(),
        }
    }
}

impl<W, P> MinWidth<W, P> {
    /// Set's a fill character which will be used to fill the space
    /// when increasing the length of the string to the set boundary.
    pub fn fill_with(mut self, c: char) -> Self {
        self.fill = c;
        self
    }

    /// Priority defines the logic by which a increase of width will be applied when is done for the whole table.
    ///
    /// - [PriorityNone] which inc the columns one after another.
    /// - [PriorityMax] inc the biggest columns first.
    /// - [PriorityMin] inc the lowest columns first.
    pub fn priority<PP: ColumnPeaker>(self) -> MinWidth<W, PP> {
        MinWidth {
            fill: self.fill,
            size: self.size,
            _priority: Default::default(),
        }
    }
}

impl<W> CellOption for MinWidth<W>
where
    W: WidthValue,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let width = self.size.width(grid);
        let content = grid.get_cell_content_styled(row, column);
        let new_content = increase_width(&content, width, self.fill);
        grid.set(Entity::Cell(row, column), Settings::new().text(new_content))
    }
}

impl<W, P> TableOption for Truncate<'_, W, P>
where
    W: WidthValue,
    P: ColumnPeaker,
{
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        let width = self.width.width(grid);

        let total_width = grid.total_width();
        if total_width == width {
            return;
        }

        if width < total_width {
            truncate_total_width(grid, total_width, width, self.suffix.as_ref(), P::create());
        }
    }
}

impl<W, P> TableOption for Wrap<W, P>
where
    W: WidthValue,
    P: ColumnPeaker,
{
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        let width = self.width.width(grid);

        let total_width = grid.total_width();
        if total_width == width {
            return;
        }

        if width < total_width {
            wrap_total_width(grid, total_width, width, self.keep_words, P::create());
        }
    }
}

impl<W, P> TableOption for MinWidth<W, P>
where
    W: WidthValue,
    P: ColumnPeaker,
{
    fn change(&mut self, grid: &mut Grid) {
        if grid.count_columns() == 0 || grid.count_rows() == 0 {
            return;
        }

        if is_zero_spanned_grid(grid) {
            return;
        }

        let width = self.size.width(grid);

        let total_width = grid.total_width();
        if total_width >= width {
            return;
        }

        increase_total_width(grid, total_width, width, P::create());
    }
}

/// Justify sets all columns widths to the set value.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [Padding] to set it to 0.
///
/// ## Examples
///
/// ```
/// use tabled::{width::Justify, Style, Modify, object::Segment, Padding, Table};
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
/// use tabled::{width::Justify, Style, Table};
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

impl<W> Justify<W>
where
    W: WidthValue,
{
    /// Creates a new Justify instance.
    ///
    /// Be aware that [Padding] is not considered when comparing the width.
    ///
    /// [Padding]: crate::Padding
    pub fn new(width: W) -> Self {
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
    W: WidthValue,
{
    fn change(&mut self, grid: &mut Grid) {
        let width = self.width.width(grid);

        for row in 0..grid.count_rows() {
            for col in 0..grid.count_columns() {
                Width::increase(width).change_cell(grid, row, col);
                Width::truncate(width).change_cell(grid, row, col);
            }
        }
    }
}

/// A width value which can be obtained on behalf of [Table].
///
/// [Table]: crate::Table
pub trait WidthValue {
    /// Returns a width value.
    fn width(&self, grid: &Grid) -> usize;
}

impl WidthValue for usize {
    fn width(&self, _: &Grid) -> usize {
        *self
    }
}

/// Max width value.
pub struct Max;

impl WidthValue for Max {
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

impl WidthValue for Min {
    fn width(&self, grid: &Grid) -> usize {
        grid_widths(grid)
            .into_iter()
            .map(|r| r.into_iter().min().unwrap_or(0))
            .min()
            .unwrap_or(0)
    }
}

/// Percent from a total table width.
pub struct Percent(pub usize);

impl WidthValue for Percent {
    fn width(&self, grid: &Grid) -> usize {
        let total = grid.total_width();
        (total * self.0) / 100
    }
}

/// A strategy of width function.
/// It determines the order how the function is applied.
pub trait ColumnPeaker {
    /// Creates a new instance.
    fn create() -> Self;
    /// This function returns a column index which will be changed.
    /// Or `None` if no changes are necessary.
    fn peak(&mut self, min_widths: &[usize], widths: &[usize]) -> Option<usize>;
}

/// A Peaker which goes over column 1 by 1.
pub struct PriorityNone {
    i: usize,
}

impl ColumnPeaker for PriorityNone {
    fn create() -> Self {
        Self { i: 0 }
    }

    fn peak(&mut self, _: &[usize], widths: &[usize]) -> Option<usize> {
        let mut i = self.i;
        while widths[i] == 0 {
            i += 1;
            if i >= widths.len() {
                i = 0;
            }
        }

        let col = i;

        i += 1;
        if i >= widths.len() {
            i = 0;
        }

        self.i = i;

        Some(col)
    }
}

/// A Peaker which goes over the biggest column first.
pub struct PriorityMax;

impl ColumnPeaker for PriorityMax {
    fn create() -> Self {
        Self
    }

    fn peak(&mut self, _: &[usize], widths: &[usize]) -> Option<usize> {
        let col = (0..widths.len()).max_by_key(|&i| widths[i]).unwrap();
        if widths[col] == 0 {
            None
        } else {
            Some(col)
        }
    }
}

/// A Peaker which goes over the smallest column first.
pub struct PriorityMin;

impl ColumnPeaker for PriorityMin {
    fn create() -> Self {
        Self
    }

    fn peak(&mut self, min_widths: &[usize], widths: &[usize]) -> Option<usize> {
        let col = (0..widths.len())
            .filter(|&i| min_widths.is_empty() || widths[i] > min_widths[i])
            .min_by_key(|&i| widths[i])
            .unwrap();
        if widths[col] == 0 {
            None
        } else {
            Some(col)
        }
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

fn increase_total_width<P: ColumnPeaker>(
    grid: &mut Grid,
    total_width: usize,
    expected_width: usize,
    priority: P,
) {
    let increase_list = increase_total_width_fn(grid, expected_width, total_width, priority);

    for ((row, col), width) in increase_list {
        MinWidth::new(width).change_cell(grid, row, col);
    }
}

fn truncate_total_width<P: ColumnPeaker>(
    grid: &mut Grid,
    total_width: usize,
    width: usize,
    suffix: &str,
    priority: P,
) {
    let points = decrease_total_width_fn(grid, total_width, width, priority);

    for ((row, col), width) in points {
        Truncate::new(width)
            .suffix(suffix)
            .change_cell(grid, row, col);
        MinWidth::new(width).change_cell(grid, row, col);
    }
}

fn wrap_total_width<P: ColumnPeaker>(
    grid: &mut Grid,
    total_width: usize,
    width: usize,
    keep_words: bool,
    priority: P,
) {
    let points = decrease_total_width_fn(grid, total_width, width, priority);

    let mut wrap = Wrap::new(0);
    wrap.keep_words = keep_words;
    for ((row, col), width) in points {
        wrap.width = width;
        wrap.change_cell(grid, row, col);
    }
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

fn decrease_total_width_fn<F>(
    grid: &Grid,
    total_width: usize,
    mut width: usize,
    mut cmp_fn: F,
) -> HashMap<(usize, usize), usize>
where
    F: ColumnPeaker,
{
    let min_widths = build_min_widths(grid);
    let mut widths = grid.build_widths();

    let mut empty_list = 0;
    for col in 0..widths.len() {
        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }
    }

    while width != total_width {
        if empty_list == widths.len() {
            break;
        }

        let col = match cmp_fn.peak(&min_widths, &widths) {
            Some(col) => col,
            None => break,
        };

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            continue;
        }

        widths[col] -= 1;

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }

        width += 1;
    }

    let mut points = HashMap::new();
    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.count_rows() {
        let mut col = 0;
        while col < widths.len() {
            match grid.get_column_span((row, col)) {
                Some(span) => {
                    let width = (col..col + span)
                        .map(|i| std::cmp::max(widths[i], min_widths[i]))
                        .sum::<usize>();
                    let count_borders = count_borders_in_range(grid, col, col + span);

                    let left_padding = grid.style(Entity::Cell(row, col)).padding.left.size;
                    let right_padding = grid
                        .style(Entity::Cell(row, col + span - 1))
                        .padding
                        .right
                        .size;
                    let width = width.saturating_sub(left_padding + right_padding);

                    let width = width + count_borders;

                    points.insert((row, col), width);
                    col += span;
                }
                None => {
                    let style = grid.style(Entity::Cell(row, col));
                    let width = std::cmp::max(widths[col], min_widths[col]);
                    let width =
                        width.saturating_sub(style.padding.left.size + style.padding.right.size);

                    points.insert((row, col), width);
                    col += 1;
                }
            }
        }
    }

    points
}

fn increase_total_width_fn<F>(
    grid: &Grid,
    total_width: usize,
    mut width: usize,
    mut cmp_fn: F,
) -> HashMap<(usize, usize), usize>
where
    F: ColumnPeaker,
{
    let mut widths = grid.build_widths();
    while width != total_width {
        let col = match cmp_fn.peak(&[], &widths) {
            Some(col) => col,
            None => break,
        };

        widths[col] += 1;

        width += 1;
    }

    let mut points = HashMap::new();
    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.count_rows() {
        let mut col = 0;
        while col < widths.len() {
            match grid.get_column_span((row, col)) {
                Some(span) => {
                    let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                    let count_borders = count_borders_in_range(grid, col, col + span);

                    let left_padding = grid.style(Entity::Cell(row, col)).padding.left.size;
                    let right_padding = grid
                        .style(Entity::Cell(row, col + span - 1))
                        .padding
                        .right
                        .size;
                    let width = width.saturating_sub(left_padding + right_padding);

                    let width = width + count_borders;

                    points.insert((row, col), width);
                    col += span;
                }
                None => {
                    let style = grid.style(Entity::Cell(row, col));
                    let width = widths[col];
                    let width =
                        width.saturating_sub(style.padding.left.size + style.padding.right.size);

                    points.insert((row, col), width);
                    col += 1;
                }
            }
        }
    }

    points
}

fn build_min_widths(grid: &Grid) -> Vec<usize> {
    let mut grid = grid.clone();
    grid.set(Entity::Global, Settings::default().text(""));

    grid.build_widths()
}

#[cfg(feature = "color")]
#[cfg(test)]
mod tests {
    use owo_colors::{colors::Yellow, OwoColorize};
    use papergrid::cut_str;

    #[test]
    fn test_color_strip() {
        let s = "Collored string"
            .fg::<Yellow>()
            .on_truecolor(12, 200, 100)
            .blink()
            .to_string();
        assert_eq!(
            cut_str(&s, 1),
            "\u{1b}[5m\u{1b}[48;2;12;200;100m\u{1b}[33mC\u{1b}[25m\u{1b}[39m\u{1b}[49m"
        )
    }
}
