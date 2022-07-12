//! This module contains object which can be used to limit a cell to a given width:
//!
//! - [Truncate] cuts a cell content to limit width.
//! - [Wrap] split the content via new lines in order to fit max width.
//! - [Justify] sets columns width to the same value.
//!
//! To set a a table width, a combination of [Width::truncate] or [Width::wrap] and [Width::increase] can be used.
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
//!         "+-----+",
//!     )
//! );
//! ```

use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

use papergrid::{
    count_borders_in_range, cut_str, string_width, string_width_multiline, Grid, Settings,
};

use crate::{object::Entity, CellOption, TableOption};

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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Truncate<'a, W = usize, P = PriorityNone> {
    width: W,
    suffix: Option<TruncateSuffix<'a>>,
    _priority: PhantomData<P>,
}

#[derive(Debug)]
struct TruncateSuffix<'a> {
    text: Cow<'a, str>,
    limit: SuffixLimit,
}

/// A suffix limit settings.
#[derive(Debug)]
pub enum SuffixLimit {
    /// Cut the suffix.
    Cut,
    /// Don't show the suffix.
    Ignore,
    /// Use a string with n chars instead.
    Replace(char),
}

impl<W> Truncate<'static, W>
where
    W: WidthValue,
{
    /// Creates a [Truncate] object
    pub fn new(width: W) -> Truncate<'static, W> {
        Self {
            width,
            suffix: None,
            _priority: Default::default(),
        }
    }
}

impl<W, P> Truncate<'_, W, P> {
    /// Sets a suffix which will be appended to a resultant string.
    ///
    /// The suffix is used in 3 circamstances:
    ///     1. If original string is *bigger* than the suffix.
    ///        We cut more of the original string and append the suffix.
    ///     2. If suffix is bigger than the original string.
    ///        We cut the suffix to fit in the width by default.
    ///        But you can peak the behaviour by using [Truncate::suffix_limit]
    pub fn suffix<'a, S: Into<Cow<'a, str>>>(self, suffix: S) -> Truncate<'a, W, P> {
        let used_limit = self.suffix.map_or(SuffixLimit::Cut, |s| s.limit);

        Truncate {
            width: self.width,
            suffix: Some(TruncateSuffix {
                text: suffix.into(),
                limit: used_limit,
            }),
            _priority: Default::default(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Sets a suffix limit, which is used when the suffix is too big to be used.
    pub fn suffix_limit(self, limit: SuffixLimit) -> Truncate<'a, W, P> {
        let text = self.suffix.map_or(Cow::Borrowed(""), |s| s.text);

        Truncate {
            width: self.width,
            suffix: Some(TruncateSuffix { text, limit }),
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
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        let orig_width = self.width.width(grid);

        let mut width = orig_width;
        let suffix = match self.suffix.as_ref() {
            Some(suffix) => {
                let suffix_length = string_width(&suffix.text);
                if width > suffix_length {
                    width -= suffix_length;
                    Cow::Borrowed(suffix.text.as_ref())
                } else {
                    match suffix.limit {
                        SuffixLimit::Ignore => Cow::Borrowed(""),
                        SuffixLimit::Cut => {
                            width = 0;
                            cut_str(&suffix.text, orig_width)
                        }
                        SuffixLimit::Replace(c) => {
                            width = 0;
                            Cow::Owned(std::iter::repeat(c).take(orig_width).collect())
                        }
                    }
                }
            }
            None => Cow::Borrowed(""),
        };

        for (row, col) in entity.iter(grid.count_rows(), grid.count_columns()) {
            let content = grid.get_cell_content_styled(row, col);
            if width < string_width_multiline(&content) {
                let text = if width == 0 {
                    if orig_width == 0 {
                        Cow::Borrowed("")
                    } else {
                        Cow::Borrowed(suffix.as_ref())
                    }
                } else {
                    let content = cut_str(&content, width);
                    if !suffix.is_empty() {
                        let mut content = content.into_owned();
                        content.push_str(&suffix);
                        Cow::Owned(content)
                    } else {
                        content
                    }
                };

                grid.set(Entity::Cell(row, col), Settings::new().text(text));
            }
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

    /// Set the keep words option.
    ///
    /// If a wrapping point will be in a word, [Wrap] will
    /// preserve a word (if possible) and wrap the string before it.
    pub fn keep_words(mut self) -> Self {
        self.keep_words = true;
        self
    }
}

impl<W> CellOption for Wrap<W>
where
    W: WidthValue,
{
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        let width = self.width.width(grid);

        for (row, col) in entity.iter(grid.count_rows(), grid.count_columns()) {
            let content = grid.get_cell_content_styled(row, col);
            if width < string_width_multiline(&content) {
                let wrapped = wrap_text(&content, width, self.keep_words);

                debug_assert!(
                    width >= string_width_multiline(&wrapped),
                    "width={:?}\n\n content={:?}\n\n wrap={:?}\n",
                    width,
                    content,
                    wrapped
                );

                grid.set(Entity::Cell(row, col), Settings::new().text(wrapped))
            }
        }
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
#[derive(Debug)]
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
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        let width = self.size.width(grid);

        for (row, col) in entity.iter(grid.count_rows(), grid.count_columns()) {
            let content = grid.get_cell_content_styled(row, col);
            let new_content = increase_width(&content, width, self.fill);
            grid.set(Entity::Cell(row, col), Settings::new().text(new_content))
        }
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
        if width < total_width {
            let suffix = self.suffix.as_ref().map_or("", |s| &s.text);
            truncate_total_width(grid, total_width, width, suffix, P::create());
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
#[derive(Debug)]
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
                Width::increase(width).change_cell(grid, Entity::Cell(row, col));
                Width::truncate(width).change_cell(grid, Entity::Cell(row, col));
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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

    __increase_width(s, width, fill_with)
}

#[cfg(not(feature = "color"))]
fn __increase_width(s: &str, width: usize, fill_with: char) -> String {
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
fn __increase_width(s: &str, width: usize, fill_with: char) -> String {
    ansi_str::AnsiStr::ansi_split(s, "\n")
        .map(|line| {
            let length = string_width(&line);
            if length < width {
                let mut line = line.into_owned();
                let remain = width - length;
                line.extend(std::iter::repeat(fill_with).take(remain));
                std::borrow::Cow::Owned(line)
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn increase_total_width<P: ColumnPeaker>(
    grid: &mut Grid,
    total_width: usize,
    expected_width: usize,
    priority: P,
) {
    let increase_list = increase_total_width_fn(grid, expected_width, total_width, priority);

    for ((row, col), width) in increase_list {
        MinWidth::new(width).change_cell(grid, Entity::Cell(row, col));
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
            .change_cell(grid, Entity::Cell(row, col));
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
        wrap.change_cell(grid, Entity::Cell(row, col));
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
) -> Vec<((usize, usize), usize)>
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

    let mut points = Vec::with_capacity(grid.count_columns() * grid.count_rows());
    (0..grid.count_columns()).for_each(|col| {
        (0..grid.count_rows())
            .filter(|&row| grid.is_cell_visible((row, col)))
            .for_each(|row| {
                let style = grid.style(Entity::Cell(row, col));
                match grid.get_column_span((row, col)) {
                    Some(span) => {
                        let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                        let min_width = (col..col + span).map(|i| min_widths[i]).sum::<usize>();
                        if width >= min_width {
                            let count_borders = count_borders_in_range(grid, col, col + span);
                            let width = width + count_borders;

                            let width = width
                                .saturating_sub(style.padding.left.size + style.padding.right.size);

                            points.push(((row, col), width));
                        }
                    }
                    None => {
                        if widths[col] >= min_widths[col] {
                            let width = std::cmp::max(widths[col], min_widths[col]);
                            let width = width
                                .saturating_sub(style.padding.left.size + style.padding.right.size);

                            points.push(((row, col), width));
                        }
                    }
                }
            })
    });

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

pub(crate) fn wrap_text(text: &str, width: usize, keep_words: bool) -> String {
    if width == 0 {
        String::new()
    } else if keep_words {
        split_keeping_words(text, width, "\n")
    } else {
        split(text, width, "\n")
    }
}

fn split(s: &str, width: usize, sep: &str) -> String {
    if width == 0 {
        return String::new();
    }

    chunks(s, width).join(sep)
}

#[cfg(not(feature = "color"))]
fn chunks(s: &str, width: usize) -> Vec<String> {
    const REPLACEMENT: char = '\u{FFFD}';

    let mut buf = String::with_capacity(width);
    let mut list = Vec::new();
    let mut i = 0;
    for c in s.chars() {
        let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        if i + c_width > width {
            let count_unknowns = width - i;
            buf.extend(std::iter::repeat(REPLACEMENT).take(count_unknowns));
            i += count_unknowns;
        } else {
            buf.push(c);
            i += c_width;
        }

        if i == width {
            list.push(buf);
            buf = String::with_capacity(width);
            i = 0;
        }
    }

    if !buf.is_empty() {
        list.push(buf);
    }

    list
}

#[cfg(feature = "color")]
fn chunks(s: &str, width: usize) -> Vec<String> {
    use std::fmt::Write;

    if width == 0 {
        return Vec::new();
    }

    let mut list = Vec::new();
    let mut line = String::with_capacity(width);
    let mut line_width = 0;

    for b in ansi_str::get_blocks(s) {
        if b.text().is_empty() {
            continue;
        }

        let _ = write!(&mut line, "{}", b.start());

        let mut part = b.text();
        while !part.is_empty() {
            let available_space = width - line_width;

            let part_width = unicode_width::UnicodeWidthStr::width(part);
            if part_width <= available_space {
                line.push_str(part);
                line_width += part_width;

                if line_width == available_space {
                    let _ = write!(&mut line, "{}", b.end());
                    list.push(line);
                    line = String::with_capacity(width);
                    line_width = 0;
                    let _ = write!(&mut line, "{}", b.start());
                }

                break;
            }

            let (lhs, rhs, (unknowns, split_char)) = split_string_at(part, available_space);

            part = &rhs[split_char..];

            line.push_str(lhs);
            line_width += unicode_width::UnicodeWidthStr::width(lhs);

            const REPLACEMENT: char = '\u{FFFD}';
            line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));
            line_width += unknowns;

            if line_width == width {
                let _ = write!(&mut line, "{}", b.end());
                list.push(line);
                line = String::with_capacity(width);
                line_width = 0;
                let _ = write!(&mut line, "{}", b.start());
            }
        }

        if line_width > 0 {
            let _ = write!(&mut line, "{}", b.end());
        }
    }

    if line_width > 0 {
        list.push(line);
    }

    list
}

#[cfg(not(feature = "color"))]
fn split_keeping_words(s: &str, width: usize, sep: &str) -> String {
    let mut lines = Vec::new();
    let mut line = String::with_capacity(width);
    let mut line_width = 0;

    let mut is_first_word = true;

    for word in s.split(' ') {
        if !is_first_word {
            let line_has_space = line_width < width;
            if line_has_space {
                line.push(' ');
                line_width += 1;
                is_first_word = false;
            }
        }

        if is_first_word {
            is_first_word = false;
        }

        let word_width = unicode_width::UnicodeWidthStr::width(word);

        let line_has_space = line_width + word_width <= width;
        if line_has_space {
            line.push_str(word);
            line_width += word_width;
            continue;
        }

        if word_width <= width {
            // the word can be fit to 'width' so we put it on new line

            line.extend(std::iter::repeat(' ').take(width - line_width));
            lines.push(line);

            line = String::with_capacity(width);
            line_width = 0;

            line.push_str(word);
            line_width += word_width;
            is_first_word = true;
        } else {
            // the word is too long any way so we split it

            let mut word_part = word;
            while !word_part.is_empty() {
                let available_space = width - line_width;
                let (lhs, rhs, (unknowns, split_char)) =
                    split_string_at(word_part, available_space);

                word_part = &rhs[split_char..];
                line_width += unicode_width::UnicodeWidthStr::width(lhs) + unknowns;

                line.push_str(lhs);
                const REPLACEMENT: char = '\u{FFFD}';
                line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));

                if line_width == width {
                    lines.push(line);
                    line = String::with_capacity(width);
                    line_width = 0;
                    is_first_word = true;
                }
            }
        }
    }

    if line_width > 0 {
        line.extend(std::iter::repeat(' ').take(width - line_width));
        lines.push(line);
    }

    lines.join(sep)
}

#[cfg(feature = "color")]
fn split_keeping_words(s: &str, width: usize, sep: &str) -> String {
    use std::fmt::Write;

    let mut lines = Vec::new();
    let mut line = String::with_capacity(width);
    let mut line_width = 0;

    let mut is_first_word = true;

    for b in ansi_str::get_blocks(s) {
        if b.text().is_empty() {
            continue;
        }

        let _ = write!(&mut line, "{}", b.start());
        for word in b.text().split(' ') {
            if !is_first_word {
                let line_has_space = line_width < width;
                if line_has_space {
                    line.push(' ');
                    line_width += 1;
                    is_first_word = false;
                }
            }
            if is_first_word {
                is_first_word = false;
            }

            let word_width = unicode_width::UnicodeWidthStr::width(word);

            let line_has_space = line_width + word_width <= width;
            if line_has_space {
                line.push_str(word);
                line_width += word_width;
                continue;
            }

            if word_width <= width {
                // the word can be fit to 'width' so we put it on new line

                let _ = write!(&mut line, "{}", b.end());
                line.extend(std::iter::repeat(' ').take(width - line_width));
                lines.push(line);

                line = String::with_capacity(width);

                let _ = write!(&mut line, "{}", b.start());
                line.push_str(word);
                line_width = word_width;
                is_first_word = false;
            } else {
                // the word is too long any way so we split it

                let mut word_part = word;
                while !word_part.is_empty() {
                    let available_space = width - line_width;
                    let (lhs, rhs, (unknowns, split_char)) =
                        split_string_at(word_part, available_space);

                    word_part = &rhs[split_char..];
                    line_width += unicode_width::UnicodeWidthStr::width(lhs) + unknowns;

                    let _ = write!(&mut line, "{}", lhs);
                    const REPLACEMENT: char = '\u{FFFD}';
                    line.extend(std::iter::repeat(REPLACEMENT).take(unknowns));

                    if line_width == width {
                        let _ = write!(&mut line, "{}", b.end());

                        lines.push(line);
                        line = String::with_capacity(width);
                        line_width = 0;
                        is_first_word = true;
                        let _ = write!(&mut line, "{}", b.start());
                    }
                }
            }
        }

        if !line.is_empty() {
            let _ = write!(&mut line, "{}", b.end());
        }
    }

    if line_width > 0 {
        line.extend(std::iter::repeat(' ').take(width - line_width));
        lines.push(line);
    }

    lines.join(sep)
}

fn split_string_at(text: &str, at: usize) -> (&str, &str, (usize, usize)) {
    use papergrid::string_split_at_length;

    let (length, count_unknowns, split_char_size) = string_split_at_length(text, at);
    let (lhs, rhs) = text.split_at(length);

    (lhs, rhs, (count_unknowns, split_char_size))
}

#[cfg(feature = "color")]
#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn split_test() {
        assert_eq!(split("123456", 0, "\n"), "");

        assert_eq!(split("123456", 1, "\n"), "1\n2\n3\n4\n5\n6");
        assert_eq!(split("123456", 2, "\n"), "12\n34\n56");
        assert_eq!(split("12345", 2, "\n"), "12\n34\n5");
        assert_eq!(split("123456", 6, "\n"), "123456");
        assert_eq!(split("123456", 10, "\n"), "123456");

        assert_eq!(split("😳😳😳😳😳", 1, "\n"), "�\n�\n�\n�\n�");
        assert_eq!(split("😳😳😳😳😳", 2, "\n"), "😳\n😳\n😳\n😳\n😳");
        assert_eq!(split("😳😳😳😳😳", 3, "\n"), "😳�\n😳�\n😳");
        assert_eq!(split("😳😳😳😳😳", 6, "\n"), "😳😳😳\n😳😳");
        assert_eq!(split("😳😳😳😳😳", 20, "\n"), "😳😳😳😳😳");

        assert_eq!(split("😳123😳", 1, "\n"), "�\n1\n2\n3\n�");
        assert_eq!(split("😳12😳3", 1, "\n"), "�\n1\n2\n�\n3");
    }

    #[cfg(feature = "color")]
    #[test]
    fn chunks_test() {
        assert_eq!(chunks("123456", 0), [""; 0]);

        assert_eq!(chunks("123456", 1), ["1", "2", "3", "4", "5", "6"]);
        assert_eq!(chunks("123456", 2), ["12", "34", "56"]);
        assert_eq!(chunks("12345", 2), ["12", "34", "5"]);

        assert_eq!(chunks("😳😳😳😳😳", 1), ["�", "�", "�", "�", "�"]);
        assert_eq!(chunks("😳😳😳😳😳", 2), ["😳", "😳", "😳", "😳", "😳"]);
        assert_eq!(chunks("😳😳😳😳😳", 3), ["😳�", "😳�", "😳"]);
    }

    #[test]
    fn split_by_line_keeping_words_test() {
        assert_eq!(split_keeping_words("123456", 1, "\n"), "1\n2\n3\n4\n5\n6");
        assert_eq!(split_keeping_words("123456", 2, "\n"), "12\n34\n56");
        assert_eq!(split_keeping_words("12345", 2, "\n"), "12\n34\n5 ");

        assert_eq!(split_keeping_words("😳😳😳😳😳", 1, "\n"), "�\n�\n�\n�\n�");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_test() {
        let text = "\u{1b}[37mJapanese “vacancy” button\u{1b}[0m";

        assert_eq!(split_keeping_words(text, 2, "\n"), "\u{1b}[37mJa\u{1b}[39m\n\u{1b}[37mpa\u{1b}[39m\n\u{1b}[37mne\u{1b}[39m\n\u{1b}[37mse\u{1b}[39m\n\u{1b}[37m“v\u{1b}[39m\n\u{1b}[37mac\u{1b}[39m\n\u{1b}[37man\u{1b}[39m\n\u{1b}[37mcy\u{1b}[39m\n\u{1b}[37m”b\u{1b}[39m\n\u{1b}[37mut\u{1b}[39m\n\u{1b}[37mto\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m ");
        assert_eq!(split_keeping_words(text, 1, "\n"), "\u{1b}[37mJ\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mp\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37ms\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37m“\u{1b}[39m\n\u{1b}[37mv\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37my\u{1b}[39m\n\u{1b}[37m”\u{1b}[39m\n\u{1b}[37mb\u{1b}[39m\n\u{1b}[37mu\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mo\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m");
    }

    #[cfg(feature = "color")]
    #[test]
    fn split_by_line_keeping_words_color_2_test() {
        use ansi_str::AnsiStr;

        let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

        assert_eq!(
            split_keeping_words(text, 2, "\n")
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "\u{1b}[37mTi\u{1b}[39m",
                "\u{1b}[37mgr\u{1b}[39m",
                "\u{1b}[37meE\u{1b}[39m",
                "\u{1b}[37mcu\u{1b}[39m",
                "\u{1b}[37mad\u{1b}[39m",
                "\u{1b}[37mor\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mOM\u{1b}[39m",
                "\u{1b}[37mYA\u{1b}[39m",
                "\u{1b}[37mAn\u{1b}[39m",
                "\u{1b}[37mdi\u{1b}[39m",
                "\u{1b}[37mna\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37m38\u{1b}[39m",
                "\u{1b}[37m24\u{1b}[39m",
                "\u{1b}[37m90\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m99\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mCa\u{1b}[39m",
                "\u{1b}[37mlc\u{1b}[39m",
                "\u{1b}[37miu\u{1b}[39m",
                "\u{1b}[37mmc\u{1b}[39m",
                "\u{1b}[37mar\u{1b}[39m",
                "\u{1b}[37mbo\u{1b}[39m",
                "\u{1b}[37mna\u{1b}[39m",
                "\u{1b}[37mte\u{1b}[39m",
                "\u{1b}[37m  \u{1b}[39m",
                "\u{1b}[37mCo\u{1b}[39m",
                "\u{1b}[37mlo\u{1b}[39m",
                "\u{1b}[37mmb\u{1b}[39m",
                "\u{1b}[37mia\u{1b}[39m",
            ]
        );

        assert_eq!(
            split_keeping_words(text, 1, "\n")
                .ansi_split("\n")
                .collect::<Vec<_>>(),
            [
                "\u{1b}[37mT\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mg\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m",
                "\u{1b}[37mE\u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37mu\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37md\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mO\u{1b}[39m",
                "\u{1b}[37mM\u{1b}[39m",
                "\u{1b}[37mY\u{1b}[39m",
                "\u{1b}[37mA\u{1b}[39m",
                "\u{1b}[37mA\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37md\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37m3\u{1b}[39m",
                "\u{1b}[37m8\u{1b}[39m",
                "\u{1b}[37m2\u{1b}[39m",
                "\u{1b}[37m4\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m0\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m9\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mC\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37ml\u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37mu\u{1b}[39m",
                "\u{1b}[37mm\u{1b}[39m",
                "\u{1b}[37mc\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37mr\u{1b}[39m",
                "\u{1b}[37mb\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mn\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
                "\u{1b}[37mt\u{1b}[39m",
                "\u{1b}[37me\u{1b}[39m",
                "\u{1b}[37m \u{1b}[39m",
                "\u{1b}[37mC\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37ml\u{1b}[39m",
                "\u{1b}[37mo\u{1b}[39m",
                "\u{1b}[37mm\u{1b}[39m",
                "\u{1b}[37mb\u{1b}[39m",
                "\u{1b}[37mi\u{1b}[39m",
                "\u{1b}[37ma\u{1b}[39m",
            ]
        )
    }

    // #[cfg(feature = "color")]
    // #[test]
    // fn split_by_line_keeping_words_color_2_test() {
    //     use ansi_str::AnsiStr;

    //     let text = "\u{1b}[37mTigre Ecuador   OMYA Andina     3824909999      Calcium carbonate       Colombia\u{1b}[0m";

    //     panic!(
    //         "{:#?}",
    //         split_by_line_keeping_words(text, 10)
    //             .ansi_split("\n")
    //             .collect::<Vec<_>>()
    //     );

    //     assert_eq!(split_by_line_keeping_words(text, 2), "\u{1b}[37mJa\u{1b}[39m\n\u{1b}[37mpa\u{1b}[39m\n\u{1b}[37mne\u{1b}[39m\n\u{1b}[37mse\u{1b}[39m\n\u{1b}[37m \u{1b}[39m \n\u{1b}[37m“\u{1b}[39m\u{1b}[37mv\u{1b}[39m\n\u{1b}[37mac\u{1b}[39m\n\u{1b}[37man\u{1b}[39m\n\u{1b}[37mcy\u{1b}[39m\n\u{1b}[37m” \u{1b}[39m\n\u{1b}[37mbu\u{1b}[39m\n\u{1b}[37mtt\u{1b}[39m\n\u{1b}[37mon\u{1b}[39m");
    //     assert_eq!(split_by_line_keeping_words(text, 1), "\u{1b}[37mJ\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mp\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37ms\u{1b}[39m\n\u{1b}[37me\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\n\u{1b}[37m“\u{1b}[39m\n\u{1b}[37mv\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37ma\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m\n\u{1b}[37mc\u{1b}[39m\n\u{1b}[37my\u{1b}[39m\n\u{1b}[37m”\u{1b}[39m\n\u{1b}[37m \u{1b}[39m\n\u{1b}[37mb\u{1b}[39m\n\u{1b}[37mu\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mt\u{1b}[39m\n\u{1b}[37mo\u{1b}[39m\n\u{1b}[37mn\u{1b}[39m");
    // }
}
