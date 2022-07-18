//! This module contains a list of primitives which can be applied to change [`Table`] style.
//!
//! ## [`Style`]
//!
//! It is responsible for a table border style.
//! An individual cell border can be set by [`Border`].
//!  
//! ### Example
//!
//! ```
//! use tabled::{Table, Style};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data).with(Style::psql()).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         " &str  \n",
//!         "-------\n",
//!         " Hello \n",
//!         " 2022  ",
//!     )
//! )
//! ```
//!
//! ## [`BorderText`]
//!
//! It's used to override a border with a custom text.
//!
//! ### Example
//!
//! ```
//! use tabled::{Table, style::{BorderText, Style}};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .with(BorderText::new(1, "Santa"))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         " &str  \n",
//!         "Santa--\n",
//!         " Hello \n",
//!         " 2022  ",
//!     )
//! )
//! ```
//!
//! ## [`Border`]
//!
//! [`Border`] can be used to modify cell's borders.
//!
//! It's possible to set a collored border when `color` feature is on.
//! See [`Symbol`].
//!
//! ### Example
//!
//! ```
//! use tabled::{Table, Style, Modify, object::Cell};
//!
//! let data = vec!["Hello", "2022"];
//! let table = Table::new(&data)
//!     .with(Style::psql())
//!     .with(Modify::new(Cell(0, 0)).with(Style::modern().frame()))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "┌───────┐\n",
//!         "│ &str  │\n",
//!         "└───────┘\n",
//!         "  Hello  \n",
//!         "  2022   ",
//!     )
//! )
//! ```
//!
//! ## [`RawStyle`]
//!
//! A different representatio of [`Style`].
//! With no checks in place.
//!
//! It also contains a list of types to support colors.
//!
//! [`Table`]: crate::Table
//! [`Symbol`]: crate::style::Symbol

use std::collections::HashMap;
use std::{borrow::Cow, marker::PhantomData};

use papergrid::{Borders, Entity, Grid, Position};

use crate::{CellOption, TableOption};

/// Style is represents a theme of a [`Table`].
///
/// It tries to limit an controlling a valid state of it.
/// It doesn't allow to call method [`Style::top_left_corner`] unless [`Style::left`] and [`Style::top`] is set.
///
/// You can turn [`Style`] into [`RawStyle`] to have more controll using [`Into`] implementation.
///
/// # Example
///
/// ```rust,no_run
/// use tabled::{Table, Style};
///
/// let style = Style::ascii()
///                 .bottom('*')
///                 .inner_intersection(' ');
///
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(style).to_string();
///
/// println!("{}", table);
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone)]
pub struct Style<T, B, L, R, H, V, Lines = ConstLines<0>> {
    borders: Borders<char>,
    lines: Lines,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
    _horizontal: PhantomData<H>,
    _vertical: PhantomData<V>,
}

type ConstLines<const N: usize> = [(usize, Line); N];

/// A marker struct which is used in [`Style`].
#[derive(Debug, Clone)]
pub struct On;

impl Style<(), (), (), (), (), ()> {
    /// This style is a style with no styling options on,
    ///
    /// ```text
    ///      id  destribution            link
    ///      0      Fedora      https://getfedora.org/
    ///      2     OpenSUSE    https://www.opensuse.org/
    ///      3   Endeavouros   https://endeavouros.com/
    /// ```
    ///
    /// Note: The cells in the example have 1-left and 1-right indent.
    ///
    /// This style can be used as a base style to build a custom one.
    ///
    /// ```rust,no_run
    /// # use tabled::Style;
    /// let style = Style::empty()
    ///     .top('*')
    ///     .bottom('*')
    ///     .vertical('#')
    ///     .bottom_intersection('^')
    ///     .top_intersection('*');
    /// ```
    pub const fn empty() -> Style<(), (), (), (), (), ()> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                None,
                None,
                None,
            ),
            [],
        )
    }

    /// This style is analog of `empty` but with a vertical space(' ') line.
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const fn blank() -> Style<(), (), (), (), (), On> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                None,
                None,
                Some(' '),
            ),
            [],
        )
    }

    /// This is a style which relays only on ASCII charset.
    ///
    /// It has horizontal and vertical lines.
    ///
    /// ```text
    ///     +----+--------------+---------------------------+
    ///     | id | destribution |           link            |
    ///     +----+--------------+---------------------------+
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     +----+--------------+---------------------------+
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     +----+--------------+---------------------------+
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    ///     +----+--------------+---------------------------+
    /// ```
    pub const fn ascii() -> Style<On, On, On, On, On, On> {
        Style::new(
            create_borders(
                Line::full('-', '+', '+', '+'),
                Line::full('-', '+', '+', '+'),
                Line::full('-', '+', '+', '+'),
                Some('|'),
                Some('|'),
                Some('|'),
            ),
            [],
        )
    }

    /// `psql` style looks like a table style `PostgreSQL` uses.
    ///
    /// It has only 1 horizontal line which splits header.
    /// And no left and right vertical lines.
    ///
    /// ```text
    ///      id | destribution |           link
    ///     ----+--------------+---------------------------
    ///      0  |    Fedora    |  https://getfedora.org/
    ///      2  |   OpenSUSE   | https://www.opensuse.org/
    ///      3  | Endeavouros  | https://endeavouros.com/
    /// ```
    pub const fn psql() -> Style<(), (), (), (), (), On, ConstLines<1>> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                None,
                None,
                Some('|'),
            ),
            [(
                1,
                Line::empty().horizontal(Some('-')).intersection(Some('+')),
            )],
        )
    }

    /// `markdown` style mimics a `Markdown` table style.
    ///
    /// ```text
    ///     | id | destribution |           link            |
    ///     |----|--------------|---------------------------|
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    /// ```
    pub const fn markdown() -> Style<(), (), On, On, (), On, ConstLines<1>> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                Some('|'),
                Some('|'),
                Some('|'),
            ),
            [(1, Line::full('-', '|', '|', '|'))],
        )
    }

    /// This style is analog of [`Style::ascii`] which uses UTF-8 charset.
    ///
    /// It has vertical and horizontal split lines.
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const fn modern() -> Style<On, On, On, On, On, On> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '┌', '┐'),
                Line::full('─', '┴', '└', '┘'),
                Line::full('─', '┼', '├', '┤'),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [],
        )
    }

    /// This style looks like a [`Style::modern`] but with rounded corners and no horozizontal lines except a header.
    ///
    /// Beware: It uses UTF-8 characters.
    ///
    /// ```text
    ///     ╭────┬──────────────┬───────────────────────────╮
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     ╰────┴──────────────┴───────────────────────────╯
    /// ```
    pub const fn rounded() -> Style<On, On, On, On, (), On, ConstLines<1>> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '╭', '╮'),
                Line::full('─', '┴', '╰', '╯'),
                Line::empty(),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [(1, Line::full('─', '┼', '├', '┤'))],
        )
    }

    /// This style uses a chars which resembles '2 lines'.
    ///
    /// Beware: It uses UTF8 characters.
    ///
    /// ```text
    ///     ╔════╦══════════════╦═══════════════════════════╗
    ///     ║ id ║ destribution ║           link            ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 0  ║    Fedora    ║  https://getfedora.org/   ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 2  ║   OpenSUSE   ║ https://www.opensuse.org/ ║
    ///     ╠════╬══════════════╬═══════════════════════════╣
    ///     ║ 3  ║ Endeavouros  ║ https://endeavouros.com/  ║
    ///     ╚════╩══════════════╩═══════════════════════════╝
    /// ```
    pub const fn extended() -> Style<On, On, On, On, On, On> {
        Style::new(
            create_borders(
                Line::full('═', '╦', '╔', '╗'),
                Line::full('═', '╩', '╚', '╝'),
                Line::full('═', '╬', '╠', '╣'),
                Some('║'),
                Some('║'),
                Some('║'),
            ),
            [],
        )
    }

    /// This is a style uses only '.' and ':' chars.
    /// It has a vertical and horizontal split lines.
    ///
    /// ```text
    ///     .................................................
    ///     : id : destribution :           link            :
    ///     :....:..............:...........................:
    ///     : 0  :    Fedora    :  https://getfedora.org/   :
    ///     :....:..............:...........................:
    ///     : 2  :   OpenSUSE   : https://www.opensuse.org/ :
    ///     :....:..............:...........................:
    ///     : 3  : Endeavouros  : https://endeavouros.com/  :
    ///     :....:..............:...........................:
    /// ```
    pub const fn dots() -> Style<On, On, On, On, On, On> {
        Style::new(
            create_borders(
                Line::full('.', '.', '.', '.'),
                Line::full('.', ':', ':', ':'),
                Line::full('.', ':', ':', ':'),
                Some(':'),
                Some(':'),
                Some(':'),
            ),
            [],
        )
    }

    /// This style is one of table views in `ReStructuredText`.
    ///
    /// ```text
    ///     ==== ============== ===========================
    ///      id   destribution             link            
    ///     ==== ============== ===========================
    ///      0       Fedora       https://getfedora.org/   
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/  
    ///     ==== ============== ===========================
    /// ```
    pub const fn re_structured_text() -> Style<On, On, (), (), (), On, ConstLines<1>> {
        Style::new(
            create_borders(
                Line::empty().horizontal(Some('=')).intersection(Some(' ')),
                Line::empty().horizontal(Some('=')).intersection(Some(' ')),
                Line::empty(),
                None,
                None,
                Some(' '),
            ),
            [(1, Line::new(Some('='), Some(' '), None, None))],
        )
    }

    /// This is a theme analog of [`Style::rounded`], but in using ascii charset and
    /// with no horizontal lines.
    ///
    /// ```text
    ///     .-----------------------------------------------.
    ///     | id | destribution |           link            |
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    ///     '-----------------------------------------------'
    /// ```
    pub const fn ascii_rounded() -> Style<On, On, On, On, (), On> {
        Style::new(
            create_borders(
                Line::full('-', '-', '.', '.'),
                Line::full('-', '-', '\'', '\''),
                Line::empty(),
                Some('|'),
                Some('|'),
                Some('|'),
            ),
            [],
        )
    }

    /// Try to fix the style when table contains spans.
    ///
    /// By default [`Style`] doesn't implies any logic to better render split lines when
    /// [`Span`] is used.
    ///
    /// So this function can be used to set the split lines in regard of spans used.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{TableIteratorExt, Style, Modify, Format, Span, object::Cell};
    ///
    /// let data = vec![
    ///     ("09", "June", "2022"),
    ///     ("10", "July", "2022"),
    /// ];
    ///
    /// let table = data.table()
    ///     .with(
    ///         Modify::new(Cell(0, 0))
    ///             .with(Format::new(|_| String::from("date")))
    ///             .with(Span::column(3))
    ///     );
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     concat!(
    ///         "+----+------+------+\n",
    ///         "|       date       |\n",
    ///         "+----+------+------+\n",
    ///         "| 09 | June | 2022 |\n",
    ///         "+----+------+------+\n",
    ///         "| 10 | July | 2022 |\n",
    ///         "+----+------+------+",
    ///     )
    /// );
    ///
    /// let table = table.with(Style::correct_spans());
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     concat!(
    ///         "+------------------+\n",
    ///         "|       date       |\n",
    ///         "+----+------+------+\n",
    ///         "| 09 | June | 2022 |\n",
    ///         "+----+------+------+\n",
    ///         "| 10 | July | 2022 |\n",
    ///         "+----+------+------+",
    ///     )
    /// );
    /// ```
    ///
    /// [`Span`]: crate::Span
    pub const fn correct_spans() -> StyleCorrectSpan {
        StyleCorrectSpan
    }
}

const fn create_borders(
    top: Line,
    bottom: Line,
    horizontal: Line,
    left: Option<char>,
    right: Option<char>,
    vertical: Option<char>,
) -> Borders {
    Borders {
        top: top.0.horizontal,
        bottom: bottom.0.horizontal,
        top_left: top.0.left,
        top_right: top.0.right,
        bottom_left: bottom.0.left,
        bottom_right: bottom.0.right,
        top_intersection: top.0.intersection,
        bottom_intersection: bottom.0.intersection,
        horizontal_left: horizontal.0.left,
        horizontal_right: horizontal.0.right,
        horizontal: horizontal.0.horizontal,
        intersection: horizontal.0.intersection,
        vertical_left: left,
        vertical_right: right,
        vertical_intersection: vertical,
    }
}

impl<T, B, L, R, H, V, Lines> Style<T, B, L, R, H, V, Lines> {
    const fn new(borders: Borders, lines: Lines) -> Self {
        Self {
            borders,
            lines,
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _horizontal: PhantomData,
            _vertical: PhantomData,
        }
    }
}

impl<T, B, L, R, H, V, Lines> Style<T, B, L, R, H, V, Lines> {
    /// Frame function returns a frame as a border.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Style, Highlight, object::Rows};
    ///
    /// let data = [["10:52:19", "Hello"], ["10:52:20", "World"]];
    /// let table = Table::new(data)
    ///     .with(Highlight::new(Rows::first(), Style::modern().frame()));
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     concat!(
    ///         "┌──────────────────┐\n",
    ///         "│    0     |   1   │\n",
    ///         "└──────────────────┘\n",
    ///         "| 10:52:19 | Hello |\n",
    ///         "+----------+-------+\n",
    ///         "| 10:52:20 | World |\n",
    ///         "+----------+-------+",
    ///     )
    /// );
    /// ```
    pub const fn frame(&self) -> Border {
        Border {
            border: Some(papergrid::Border {
                top: self.borders.top,
                bottom: self.borders.bottom,
                left: self.borders.vertical_left,
                right: self.borders.vertical_right,
                left_top_corner: self.borders.top_left,
                right_top_corner: self.borders.top_right,
                left_bottom_corner: self.borders.bottom_left,
                right_bottom_corner: self.borders.bottom_right,
            }),
        }
    }

    /// Get a [`Style`]'s default horizontal line.
    ///
    /// It doesn't return an overloaded line via [`Style::lines`].
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{style::{Style, Line}, TableIteratorExt};
    ///
    /// let table = (0..3)
    ///    .map(|i| ("Hello", "World", i))
    ///    .table()
    ///    .with(Style::ascii().off_horizontal().lines([(1, Style::modern().get_horizontal())]))
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+-------+-------+-----+\n",
    ///         "| &str  | &str  | i32 |\n",
    ///         "├───────┼───────┼─────┤\n",
    ///         "| Hello | World |  0  |\n",
    ///         "| Hello | World |  1  |\n",
    ///         "| Hello | World |  2  |\n",
    ///         "+-------+-------+-----+",
    ///     )
    /// )
    /// ```
    pub const fn get_horizontal(&self) -> Line {
        Line::new(
            self.borders.horizontal,
            self.borders.intersection,
            self.borders.horizontal_left,
            self.borders.horizontal_right,
        )
    }

    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn top(mut self, c: char) -> Style<On, B, L, R, H, V, Lines> {
        self.borders.top = Some(c);

        if self.borders.has_left() {
            self.borders.top_left = Some(c)
        }

        if self.borders.has_right() {
            self.borders.top_right = Some(c)
        }

        if self.borders.has_vertical() {
            self.borders.top_intersection = Some(c)
        }

        Style::new(self.borders, self.lines)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn bottom(mut self, c: char) -> Style<T, On, L, R, H, V, Lines> {
        self.borders.bottom = Some(c);

        if self.borders.has_left() {
            self.borders.bottom_left = Some(c);
        }

        if self.borders.has_right() {
            self.borders.bottom_right = Some(c);
        }

        if self.borders.has_vertical() {
            self.borders.bottom_intersection = Some(c);
        }

        Style::new(self.borders, self.lines)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn left(mut self, c: char) -> Style<T, B, On, R, H, V, Lines>
    where
        for<'a> &'a mut Lines: IntoIterator<Item = &'a mut (usize, Line)>,
    {
        self.borders.vertical_left = Some(c);

        if self.borders.has_top() {
            self.borders.top_left = Some(c);
        }

        if self.borders.has_bottom() {
            self.borders.bottom_left = Some(c);
        }

        if self.borders.has_horizontal() {
            self.borders.horizontal_left = Some(c);
        }

        for (_, line) in &mut self.lines {
            line.0.left = Some(c);
        }

        Style::new(self.borders, self.lines)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn right(mut self, c: char) -> Style<T, B, L, On, H, V, Lines>
    where
        for<'a> &'a mut Lines: IntoIterator<Item = &'a mut (usize, Line)>,
    {
        self.borders.vertical_right = Some(c);

        if self.borders.has_top() {
            self.borders.top_right = Some(c);
        }

        if self.borders.has_bottom() {
            self.borders.bottom_right = Some(c);
        }

        if self.borders.has_horizontal() {
            self.borders.horizontal_right = Some(c);
        }

        for (_, line) in &mut self.lines {
            line.0.right = Some(c);
        }

        Style::new(self.borders, self.lines)
    }

    /// Sets a horizontal split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn horizontal(mut self, c: char) -> Style<T, B, L, R, On, V, Lines> {
        self.borders.horizontal = Some(c);

        if self.borders.has_vertical() {
            self.borders.intersection = Some(c);
        }

        if self.borders.has_left() {
            self.borders.horizontal_left = Some(c);
        }

        if self.borders.has_right() {
            self.borders.horizontal_right = Some(c);
        }

        Style::new(self.borders, self.lines)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn vertical(mut self, c: char) -> Style<T, B, L, R, H, On, Lines>
    where
        for<'a> &'a mut Lines: IntoIterator<Item = &'a mut (usize, Line)>,
    {
        self.borders.vertical_intersection = Some(c);

        if self.borders.has_horizontal() {
            self.borders.intersection = Some(c);
        }

        if self.borders.has_top() {
            self.borders.top_intersection = Some(c);
        }

        if self.borders.has_bottom() {
            self.borders.bottom_intersection = Some(c);
        }

        for (_, line) in &mut self.lines {
            line.0.intersection = Some(c);
        }

        Style::new(self.borders, self.lines)
    }

    /// Set border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{style::{Style, Line}, TableIteratorExt};
    ///
    /// let table = (0..3)
    ///    .map(|i| ("Hello", i))
    ///    .table()
    ///    .with(Style::rounded().lines((1..4).map(|i| (i, Line::filled('#')))))
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "╭───────┬─────╮\n",
    ///         "│ &str  │ i32 │\n",
    ///         "###############\n",
    ///         "│ Hello │  0  │\n",
    ///         "###############\n",
    ///         "│ Hello │  1  │\n",
    ///         "###############\n",
    ///         "│ Hello │  2  │\n",
    ///         "╰───────┴─────╯",
    ///     )
    /// )
    /// ```
    pub fn lines<NewLines>(self, lines: NewLines) -> Style<T, B, L, R, H, V, NewLines>
    where
        NewLines: IntoIterator<Item = (usize, Line)> + Clone,
    {
        Style::new(self.borders, lines)
    }

    /// Removes all lines set by [`Style::lines`]
    pub fn off_lines(self) -> Style<T, B, L, R, H, V> {
        Style::new(self.borders, [])
    }
}

impl<B, R, H, V, Lines> Style<On, B, On, R, H, V, Lines> {
    /// Sets a top left corner.
    pub fn top_left_corner(mut self, c: char) -> Self {
        self.borders.top_left = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<B, L, H, V, Lines> Style<On, B, L, On, H, V, Lines> {
    /// Sets a top right corner.
    pub fn top_right_corner(mut self, c: char) -> Self {
        self.borders.top_right = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, L, H, V, Lines> Style<T, On, L, On, H, V, Lines> {
    /// Sets a bottom right corner.
    pub fn bottom_right_corner(mut self, c: char) -> Self {
        self.borders.bottom_right = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, R, H, V, Lines> Style<T, On, On, R, H, V, Lines> {
    /// Sets a bottom left corner.
    pub fn bottom_left_corner(mut self, c: char) -> Self {
        self.borders.bottom_left = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, B, R, V, Lines> Style<T, B, On, R, On, V, Lines> {
    /// Sets a left intersection char.
    pub fn left_intersection(mut self, c: char) -> Self {
        self.borders.horizontal_left = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, B, L, V, Lines> Style<T, B, L, On, On, V, Lines> {
    /// Sets a right intersection char.
    pub fn right_intersection(mut self, c: char) -> Self {
        self.borders.horizontal_right = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<B, L, R, H, Lines> Style<On, B, L, R, H, On, Lines> {
    /// Sets a top intersection char.
    pub fn top_intersection(mut self, c: char) -> Self {
        self.borders.top_intersection = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, L, R, H, Lines> Style<T, On, L, R, H, On, Lines> {
    /// Sets a bottom intersection char.
    pub fn bottom_intersection(mut self, c: char) -> Self {
        self.borders.bottom_intersection = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<T, B, L, R, Lines> Style<T, B, L, R, On, On, Lines> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub fn inner_intersection(mut self, c: char) -> Self {
        self.borders.intersection = Some(c);

        Style::new(self.borders, self.lines)
    }
}

impl<B, L, R, H, V, Lines> Style<On, B, L, R, H, V, Lines> {
    /// Removes top border.
    pub fn off_top(mut self) -> Style<(), B, L, R, H, V, Lines> {
        self.borders.top = None;
        self.borders.top_intersection = None;
        self.borders.top_left = None;
        self.borders.top_right = None;

        Style::new(self.borders, self.lines)
    }
}

impl<T, L, R, H, V, Lines> Style<T, On, L, R, H, V, Lines> {
    /// Removes bottom border.
    pub fn off_bottom(mut self) -> Style<T, (), L, R, H, V, Lines> {
        self.borders.bottom = None;
        self.borders.bottom_intersection = None;
        self.borders.bottom_left = None;
        self.borders.bottom_right = None;

        Style::new(self.borders, self.lines)
    }
}

impl<T, B, R, H, V, Lines> Style<T, B, On, R, H, V, Lines> {
    /// Removes left border.
    pub fn off_left(mut self) -> Style<T, B, (), R, H, V, BorderLinesIntoIter<Lines>>
    where
        Lines: IntoIterator<Item = (usize, Line)> + Clone,
    {
        self.borders.vertical_left = None;
        self.borders.horizontal_left = None;
        self.borders.top_left = None;
        self.borders.bottom_left = None;

        let iter = BorderLinesIntoIter::new(self.lines, false, true, false);
        Style::new(self.borders, iter)
    }
}

impl<T, B, L, H, V, Lines> Style<T, B, L, On, H, V, Lines> {
    /// Removes right border.
    pub fn off_right(mut self) -> Style<T, B, L, (), H, V, BorderLinesIntoIter<Lines>>
    where
        Lines: IntoIterator<Item = (usize, Line)> + Clone,
    {
        self.borders.vertical_right = None;
        self.borders.horizontal_right = None;
        self.borders.top_right = None;
        self.borders.bottom_right = None;

        let iter = BorderLinesIntoIter::new(self.lines, false, false, true);
        Style::new(self.borders, iter)
    }
}

impl<T, B, L, R, V, Lines> Style<T, B, L, R, On, V, Lines> {
    /// Removes horizontal split lines.
    ///
    /// Not including 1st split line.
    pub fn off_horizontal(mut self) -> Style<T, B, L, R, (), V, Lines> {
        self.borders.horizontal = None;
        self.borders.horizontal_left = None;
        self.borders.horizontal_right = None;
        self.borders.intersection = None;

        Style::new(self.borders, self.lines)
    }
}

impl<T, B, L, R, H, Lines> Style<T, B, L, R, H, On, Lines> {
    /// Removes vertical split lines.
    pub fn off_vertical(mut self) -> Style<T, B, L, R, H, (), BorderLinesIntoIter<Lines>>
    where
        Lines: IntoIterator<Item = (usize, Line)> + Clone,
    {
        self.borders.vertical_intersection = None;
        self.borders.top_intersection = None;
        self.borders.bottom_intersection = None;
        self.borders.intersection = None;

        let iter = BorderLinesIntoIter::new(self.lines, true, false, false);
        Style::new(self.borders, iter)
    }
}

impl<T, B, L, R, H, V, Lines> TableOption for Style<T, B, L, R, H, V, Lines>
where
    Lines: IntoIterator<Item = (usize, Line)> + Clone + std::fmt::Debug,
{
    fn change(&mut self, grid: &mut Grid) {
        grid.clear_theme();
        grid.set_borders(self.borders.clone());

        if grid.count_rows() > 1 {
            for (row, line) in self.lines.clone() {
                if line.is_empty() {
                    grid.remove_split_line(row);
                } else {
                    grid.set_split_line(row, line.clone().into());
                }
            }
        }
    }
}

impl<T, B, L, R, H, V, Lines> From<Style<T, B, L, R, H, V, Lines>> for RawStyle
where
    Lines: IntoIterator<Item = (usize, Line)>,
{
    fn from(style: Style<T, B, L, R, H, V, Lines>) -> Self {
        Self {
            borders: style.borders,
            lines: style.lines.into_iter().collect(),
        }
    }
}

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone)]
pub struct RawStyle {
    borders: Borders<char>,
    lines: HashMap<usize, Line>,
}

impl RawStyle {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top = s;
        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom = s;
        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical_left = s;
        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical_right = s;
        self
    }

    /// Set a top split border character.
    pub fn set_top_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_intersection = s;
        self
    }

    /// Set a bottom split character.
    pub fn set_bottom_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_intersection = s;
        self
    }

    /// Set a left split character.
    pub fn set_left_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal_left = s;
        self
    }

    /// Set a right split character.
    pub fn set_right_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal_right = s;
        self
    }

    /// Set an internal character.
    pub fn set_internal_split(&mut self, s: Option<char>) -> &mut Self {
        self.borders.intersection = s;
        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical_intersection = s;
        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal = s;
        self
    }

    /// Set a character for a top left corner.
    pub fn set_top_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_left = s;
        self
    }

    /// Set a character for a top right corner.
    pub fn set_top_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_right = s;
        self
    }

    /// Set a character for a bottom left corner.
    pub fn set_bottom_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_left = s;
        self
    }

    /// Set a character for a bottom right corner.
    pub fn set_bottom_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_right = s;
        self
    }

    /// Set border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{style::{Style, Line, RawStyle}, TableIteratorExt};
    ///
    /// let mut style = RawStyle::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, Style::extended().get_horizontal());
    /// style.set_lines(lines);
    ///
    /// let table = (0..3)
    ///    .map(|i| ("Hello", i))
    ///    .table()
    ///    .with(style)
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         " ======= ===== \n",
    ///         "  &str    i32  \n",
    ///         "╠═══════╬═════╣\n",
    ///         "  Hello    0   \n",
    ///         "  Hello    1   \n",
    ///         "  Hello    2   \n",
    ///         " ======= ===== ",
    ///     ),
    /// )
    /// ```
    pub fn set_lines(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.lines = lines;
        self
    }
}

impl From<Borders<char>> for RawStyle {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            lines: HashMap::new(),
        }
    }
}

impl RawStyle {
    /// Returns a [`RawStyle`] version which can set colors.
    #[cfg(feature = "color")]
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub fn colored(self) -> RawStyleColored {
        RawStyleColored {
            style: self,
            colors: Borders::default(),
        }
    }
}

impl TableOption for RawStyle {
    fn change(&mut self, grid: &mut Grid) {
        grid.clear_theme();
        grid.set_borders(self.borders.clone());

        if grid.count_rows() > 1 {
            for (&row, line) in &self.lines {
                if line.is_empty() {
                    grid.remove_split_line(row);
                } else {
                    grid.set_split_line(row, line.clone().into());
                }
            }
        }
    }
}

/// [`BorderText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::BorderText};
///
/// let table = Table::new(["Hello World"])
///     .with(BorderText::first("+-.table"));
///
/// assert_eq!(
///     table.to_string(),
///     "+-.table------+\n\
///      |    &str     |\n\
///      +-------------+\n\
///      | Hello World |\n\
///      +-------------+"
/// );
/// ```
#[derive(Debug)]
pub struct BorderText<'a> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
    row: SplitLineIndex,
}

#[derive(Debug)]
enum SplitLineIndex {
    First,
    Last,
    Line(usize),
}

impl<'a> BorderText<'a> {
    /// Creates a [`BorderText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Line(line),
        }
    }

    /// Creates a [`BorderText`] instance for a top line.
    pub fn first<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::First,
        }
    }

    /// Creates a [`BorderText`] instance for a bottom line.
    pub fn last<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Last,
        }
    }
}

impl<'a> TableOption for BorderText<'a> {
    fn change(&mut self, grid: &mut Grid) {
        let row = match self.row {
            SplitLineIndex::First => 0,
            SplitLineIndex::Last => grid.count_rows(),
            SplitLineIndex::Line(row) => {
                if row > grid.count_rows() {
                    return;
                }

                row
            }
        };

        grid.override_split_line(row, self.text.as_ref());
    }
}

/// Border represents a border of a Cell.
///
/// ```rust,no_run
/// # use tabled::{style::{Style, Border}, object::Rows, Table, Modify};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Border {
    border: Option<papergrid::Border>,
}

impl Border {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self::from(papergrid::Border::new(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ))
    }

    /// Set a top border character.
    pub fn top(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.top = Some(c);
        Self::from(b)
    }

    /// Set a bottom border character.
    pub fn bottom(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.bottom = Some(c);
        Self::from(b)
    }

    /// Set a left border character.
    pub fn left(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left = Some(c);
        Self::from(b)
    }

    /// Set a right border character.
    pub fn right(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right = Some(c);
        Self::from(b)
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left_top_corner = Some(c);
        Self::from(b)
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right_top_corner = Some(c);
        Self::from(b)
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left_bottom_corner = Some(c);
        Self::from(b)
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right_bottom_corner = Some(c);
        Self::from(b)
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [`Border::new`] with the same character set to each side.
    pub fn filled(c: char) -> Self {
        Self::new(c, c, c, c, c, c, c, c)
    }

    /// Using this function you deconstruct the existing borders.
    pub fn none() -> Self {
        Self { border: None }
    }
}

impl From<papergrid::Border> for Border {
    fn from(b: papergrid::Border) -> Border {
        Border { border: Some(b) }
    }
}

impl From<Border> for Option<papergrid::Border> {
    fn from(val: Border) -> Self {
        val.border
    }
}

impl CellOption for Border {
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        match &self.border {
            Some(border) => {
                grid.set_border(entity, border.clone());
            }
            None => {
                grid.remove_border(entity);
            }
        }
    }
}

/// A correctnes function of style for [`Table`] which has [`Span`]s.
///
/// See [`Style::correct_spans`].
///
/// [`Table`]: crate::Table
/// [`Span`]: crate::Span
#[derive(Debug)]
pub struct StyleCorrectSpan;

impl TableOption for StyleCorrectSpan {
    fn change(&mut self, grid: &mut Grid) {
        correct_span_styles(grid);
    }
}

/// ColoredBorder represents a colored border of a Cell.
///
/// ```rust,no_run
/// # use owo_colors::OwoColorize;
/// # use tabled::{style::{Style, Symbol, ColoredBorder}, object::Rows, Table, Modify};
/// #
/// # let data: Vec<&'static str> = Vec::new();
/// #
/// let c = Symbol::ansi("#".red().to_string()).unwrap();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(ColoredBorder::default().top(c)));
/// ```
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ColoredBorder(pub(crate) papergrid::Border<Symbol>);

#[cfg(feature = "color")]
impl ColoredBorder {
    /// Set a top border character.
    pub fn top(self, c: Symbol) -> Self {
        Self(self.0.top(c))
    }

    /// Set a bottom border character.
    pub fn bottom(self, c: Symbol) -> Self {
        Self(self.0.bottom(c))
    }

    /// Set a left border character.
    pub fn left(self, c: Symbol) -> Self {
        Self(self.0.left(c))
    }

    /// Set a right border character.
    pub fn right(self, c: Symbol) -> Self {
        Self(self.0.right(c))
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(self, c: Symbol) -> Self {
        Self(self.0.top_left_corner(c))
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(self, c: Symbol) -> Self {
        Self(self.0.top_right_corner(c))
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(self, c: Symbol) -> Self {
        Self(self.0.bottom_left_corner(c))
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(self, c: Symbol) -> Self {
        Self(self.0.bottom_right_corner(c))
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [Border::new] with the same character set to each side.
    pub fn filled(c: Symbol) -> Self {
        Self(papergrid::Border {
            top: Some(c.clone()),
            bottom: Some(c.clone()),
            left: Some(c.clone()),
            right: Some(c.clone()),
            left_bottom_corner: Some(c.clone()),
            left_top_corner: Some(c.clone()),
            right_bottom_corner: Some(c.clone()),
            right_top_corner: Some(c),
        })
    }
}

#[cfg(feature = "color")]
impl CellOption for ColoredBorder {
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        grid.set_colored_border(entity, self.0.clone());
    }
}

/// Symbol represents a character of a border.
///
/// It's only needed when used with `color` feature flag.
///
/// ```rust,no_run
/// # use owo_colors::OwoColorize;
/// # use tabled::{style::{ColoredBorder, Symbol}, object::Rows, TableIteratorExt, Modify};
/// #
/// # let data: Vec<&'static str> = Vec::new();
/// #
/// let colored_char = "#".red().to_string();
/// let table = data.table()
///     .with(Modify::new(Rows::single(0)).with(ColoredBorder::filled(Symbol::ansi(colored_char).unwrap())));
/// ```
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use papergrid::Symbol;

/// BorderColor represents a color which can be set to a Border.
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use owo_colors::OwoColorize;
/// use tabled::{style::BorderColor, TableIteratorExt};
///
/// let data = [
///     (0u8, "Hello"),
///     (1u8, "World"),
/// ];
///
/// let color = BorderColor::try_from(" ".red().to_string()).unwrap();
///
/// let table = data.table().with(color);
///
/// println!("{}", table);
/// ```
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
pub use papergrid::BorderColor;

#[cfg(feature = "color")]
impl TableOption for BorderColor {
    fn change(&mut self, grid: &mut Grid) {
        grid.set_border_color(self.clone());
    }
}

/// A colored [`StyleConfig`] versions.
#[cfg(feature = "color")]
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone)]
pub struct RawStyleColored {
    style: RawStyle,
    colors: Borders<BorderColor>,
}

#[cfg(feature = "color")]
impl RawStyleColored {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_top(c);
        self.colors.top = color;

        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_bottom(c);
        self.colors.bottom = color;

        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_left(c);
        self.colors.vertical_left = color;

        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_right(c);
        self.colors.vertical_right = color;

        self
    }

    /// Set a top split border character.
    pub fn set_top_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_top_split(c);
        self.colors.top_intersection = color;

        self
    }

    /// Set a bottom split character.
    pub fn set_bottom_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_bottom_split(c);
        self.colors.bottom_intersection = color;

        self
    }

    /// Set a left split character.
    pub fn set_left_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_left_split(c);
        self.colors.horizontal_left = color;

        self
    }

    /// Set a right split character.
    pub fn set_right_split(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_right_split(c);
        self.colors.horizontal_right = color;

        self
    }

    /// Set an internal character.
    pub fn set_internal(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_internal_split(c);
        self.colors.intersection = color;

        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_vertical(c);
        self.colors.vertical_intersection = color;

        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_horizontal(c);
        self.colors.horizontal = color;

        self
    }

    /// Set a character for a top left corner.
    pub fn set_top_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_top_left(c);
        self.colors.top_left = color;

        self
    }

    /// Set a character for a top right corner.
    pub fn set_top_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_top_right(c);
        self.colors.top_right = color;

        self
    }

    /// Set a character for a bottom left corner.
    pub fn set_bottom_left(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_bottom_left(c);
        self.colors.bottom_left = color;

        self
    }

    /// Set a character for a bottom right corner.
    pub fn set_bottom_right(&mut self, s: Option<Symbol>) -> &mut Self {
        let c = s.as_ref().map(|s| s.c());
        let color = s.and_then(|s| s.color());

        self.style.set_bottom_right(c);
        self.colors.bottom_right = color;

        self
    }
}

#[cfg(feature = "color")]
impl TableOption for RawStyleColored {
    fn change(&mut self, grid: &mut Grid) {
        self.style.change(grid);
        grid.set_borders_color(self.colors.clone());
    }
}

fn correct_span_styles(grid: &mut Grid) {
    let spans = grid.iter_column_spans().collect::<Vec<_>>();

    for &((row, c), span) in &spans {
        for col in c..c + span {
            if col == 0 {
                continue;
            }

            let is_first = col == c;
            let has_up = row > 0 && has_vertical(grid, &spans, (row - 1, col));
            let has_down =
                row + 1 < grid.count_rows() && has_vertical(grid, &spans, (row + 1, col));

            let mut border = grid.get_border((row, col));

            let has_top_border = border.left_top_corner.is_some() && border.top.is_some();
            if has_top_border {
                if has_up && is_first {
                    border.left_top_corner = grid.get_borders().intersection;
                } else if has_up {
                    border.left_top_corner = grid.get_borders().bottom_intersection;
                } else if is_first {
                    border.left_top_corner = grid.get_borders().top_intersection;
                } else {
                    border.left_top_corner = border.top;
                }
            }

            let has_bottom_border = border.left_bottom_corner.is_some() && border.bottom.is_some();
            if has_bottom_border {
                if has_down && is_first {
                    border.left_bottom_corner = grid.get_borders().intersection;
                } else if has_down {
                    border.left_bottom_corner = grid.get_borders().top_intersection;
                } else if is_first {
                    border.left_bottom_corner = grid.get_borders().bottom_intersection;
                } else {
                    border.left_bottom_corner = border.bottom;
                }
            }

            grid.set_border(Entity::Cell(row, col), border);
        }
    }
}

fn has_vertical(grid: &Grid, spans: &[(Position, usize)], pos: Position) -> bool {
    if is_in_span_range(spans, pos) {
        return spans.iter().any(|&(p, _)| p == pos);
    }

    if grid.is_cell_visible(pos) {
        let border = grid.get_border(pos);
        return border.left.is_some()
            || border.left_top_corner.is_some()
            || border.left_bottom_corner.is_some();
    }

    false
}

fn is_in_span_range(spans: &[(Position, usize)], pos: Position) -> bool {
    spans
        .iter()
        .any(|&((row, col), span)| row == pos.0 && pos.1 > col && pos.1 < col + span)
}

/// An helper for a [`BorderLinesIter`].
#[derive(Debug, Clone)]
pub struct BorderLinesIntoIter<I> {
    iter: I,
    intersection: bool,
    left: bool,
    right: bool,
}

impl<I> BorderLinesIntoIter<I> {
    const fn new(iter: I, intersection: bool, left: bool, right: bool) -> Self {
        Self {
            iter,
            intersection,
            left,
            right,
        }
    }
}

impl<I> IntoIterator for BorderLinesIntoIter<I>
where
    I: IntoIterator<Item = (usize, Line)>,
{
    type Item = (usize, Line);
    type IntoIter = BorderLinesIter<I::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        BorderLinesIter::new(
            self.iter.into_iter(),
            self.intersection,
            self.left,
            self.right,
        )
    }
}

/// An interator which limits [`Line`] influence on iterations over lines for in [`Style`].
#[derive(Debug, Clone)]
pub struct BorderLinesIter<I> {
    iter: I,
    intersection: bool,
    left: bool,
    right: bool,
}

impl<I> BorderLinesIter<I> {
    fn new(iter: I, intersection: bool, left: bool, right: bool) -> Self {
        Self {
            iter,
            intersection,
            left,
            right,
        }
    }
}

impl<I> Iterator for BorderLinesIter<I>
where
    I: Iterator<Item = (usize, Line)>,
{
    type Item = (usize, Line);

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.iter.next()?;

        if self.intersection {
            line.1 = line.1.intersection(None);
        }

        if self.left {
            line.1 = line.1.left(None);
        }

        if self.right {
            line.1 = line.1.right(None);
        }

        Some(line)
    }
}

/// Line is a horizontal line which can be used when setting style in [`Style::lines`].
#[derive(Debug, Clone, Default)]
pub struct Line(papergrid::Line<char>);

impl Line {
    /// Creates a new [`Line`].
    pub const fn new(
        horizontal: Option<char>,
        intersection: Option<char>,
        left: Option<char>,
        right: Option<char>,
    ) -> Self {
        Self(papergrid::Line {
            horizontal,
            intersection,
            left,
            right,
        })
    }

    /// Creates a new [`Line`] with horinzontal, left, right and vertical intersection be set on.
    pub const fn full(horizontal: char, intersection: char, left: char, right: char) -> Self {
        Self::new(
            Some(horizontal),
            Some(intersection),
            Some(left),
            Some(right),
        )
    }

    /// Creates a new [`Line`] with horinzontal, left, right and vertical intersection set to the given character.
    pub const fn filled(c: char) -> Self {
        Self::full(c, c, c, c)
    }

    /// Creates a new empty [`Line`].
    pub const fn empty() -> Self {
        Self(papergrid::Line {
            horizontal: None,
            intersection: None,
            left: None,
            right: None,
        })
    }

    /// Sets a horizontal character.
    pub const fn horizontal(mut self, c: Option<char>) -> Self {
        self.0.horizontal = c;
        self
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: Option<char>) -> Self {
        self.0.intersection = c;
        self
    }

    /// Sets a left character.
    pub const fn left(mut self, c: Option<char>) -> Self {
        self.0.left = c;
        self
    }

    /// Sets a right character.
    pub const fn right(mut self, c: Option<char>) -> Self {
        self.0.right = c;
        self
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Line> for papergrid::Line<char> {
    fn from(line: Line) -> Self {
        line.0
    }
}
