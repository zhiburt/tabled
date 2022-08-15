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
//! use tabled::{Table, BorderText, Style};
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

use std::marker::PhantomData;

use papergrid::{records::Records, Borders};

use crate::border::Border;
use crate::span_border_correction::StyleCorrectSpan;
use crate::table::{Table, TableOption};

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
    pub(crate) borders: Borders<char>,
    pub(crate) lines: Lines,
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
    /// use tabled::{TableIteratorExt, Style, Modify, format::Format, Span, object::Cell};
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
    ///         "| date             |\n",
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
    ///         "| date             |\n",
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
        vertical,
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
    ///         "│ 0        | 1     │\n",
    ///         "└──────────────────┘\n",
    ///         "| 10:52:19 | Hello |\n",
    ///         "+----------+-------+\n",
    ///         "| 10:52:20 | World |\n",
    ///         "+----------+-------+",
    ///     )
    /// );
    /// ```
    pub const fn frame(&self) -> Border {
        Border::new_raw(Some(papergrid::Border {
            top: self.borders.top,
            bottom: self.borders.bottom,
            left: self.borders.vertical_left,
            right: self.borders.vertical_right,
            left_top_corner: self.borders.top_left,
            right_top_corner: self.borders.top_right,
            left_bottom_corner: self.borders.bottom_left,
            right_bottom_corner: self.borders.bottom_right,
        }))
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
    ///         "| Hello | World | 0   |\n",
    ///         "| Hello | World | 1   |\n",
    ///         "| Hello | World | 2   |\n",
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
            self.borders.top_left = Some(c);
        }

        if self.borders.has_right() {
            self.borders.top_right = Some(c);
        }

        if self.borders.has_vertical() {
            self.borders.top_intersection = Some(c);
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
        self.borders.vertical = Some(c);

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
    ///         "│ Hello │ 0   │\n",
    ///         "###############\n",
    ///         "│ Hello │ 1   │\n",
    ///         "###############\n",
    ///         "│ Hello │ 2   │\n",
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
        self.borders.vertical = None;
        self.borders.top_intersection = None;
        self.borders.bottom_intersection = None;
        self.borders.intersection = None;

        let iter = BorderLinesIntoIter::new(self.lines, true, false, false);
        Style::new(self.borders, iter)
    }
}

impl<T, B, L, R, H, V, Lines, I> TableOption<I> for Style<T, B, L, R, H, V, Lines>
where
    Lines: IntoIterator<Item = (usize, Line)> + Clone,
    I: Records,
{
    fn change(&mut self, table: &mut Table<I>) {
        table.get_config_mut().clear_theme();
        table.get_config_mut().set_borders(self.borders.clone());

        if table.shape().0 > 1 {
            for (row, line) in self.lines.clone() {
                if line.is_empty() {
                    table.get_config_mut().remove_split_line(row);
                } else {
                    table
                        .get_config_mut()
                        .set_split_line(row, line.clone().into());
                }
            }
        }
    }
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

    /// Checks if it's an empty line.
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Line> for papergrid::Line<char> {
    fn from(line: Line) -> Self {
        line.0
    }
}
