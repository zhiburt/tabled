//! This module contains a list of primitives which can be applied to change [Table] style.
//!
//! ## [Style]
//!
//! It is responsible for a table border style.
//! An individual cell border can be set by [Border].
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
//!         " 2022  \n",
//!     )
//! )
//! ```
//!
//! ## [BorderText]
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
//!         " 2022  \n",
//!     )
//! )
//! ```
//!
//! ## [Border]
//!
//! [Border] can be used to modify cell's borders.
//!
//! It's possible to set a collored border when `color` feature is on.
//! See [Symbol].
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
//!         "  2022   \n",
//!     )
//! )
//! ```
//!
//! [Table]: crate::Table

use std::{borrow::Cow, marker::PhantomData};

use crate::{CellOption, TableOption};
use papergrid::{Borders, Entity, Grid, Settings};

/// Style is represents a theme of a [Table].
///
/// It can be Mofified extensively, look at [CustomStyle] methods.
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
/// [Table]: crate::Table
pub struct Style;

impl Style {
    /// Empty style is a style with no styling options on,
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
    /// It's usefull as a scratch style to build a custom one.
    ///
    /// ```rust,no_run
    /// # use tabled::Style;
    /// let style = Style::empty()
    ///     .top('*')
    ///     .bottom('*')
    ///     .header('x')
    ///     .vertical('#')
    ///     .bottom_intersection('^')
    ///     .top_intersection('*');
    /// ```
    pub const fn empty() -> CustomStyle<(), (), (), (), (), (), ()> {
        CustomStyle::new(Self::EMPTY)
    }

    /// Blank style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const fn blank() -> CustomStyle<(), (), (), (), (), On, ()> {
        CustomStyle::new(Self::BLANK)
    }

    /// Default style looks like the following table
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
    pub const fn ascii() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::ASCII)
    }

    /// Dots style looks like the following table
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
    pub const fn dots() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::DOTS)
    }

    /// Psql style looks like the following table
    ///
    /// ```text
    ///      id | destribution |           link
    ///     ----+--------------+---------------------------
    ///      0  |    Fedora    |  https://getfedora.org/
    ///      2  |   OpenSUSE   | https://www.opensuse.org/
    ///      3  | Endeavouros  | https://endeavouros.com/
    /// ```
    pub const fn psql() -> CustomStyle<(), (), (), (), (), On, On> {
        CustomStyle::new(Self::PSQL)
    }

    /// Github_markdown style looks like the following table
    ///
    /// ```text
    ///     | id | destribution |           link            |
    ///     |----+--------------+---------------------------|
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    /// ```
    pub const fn github_markdown() -> CustomStyle<(), (), On, On, (), On, On> {
        CustomStyle::new(Self::GITHUB_MARKDOWN)
    }

    /// Modern style looks like the following table.
    ///
    /// Beware: It uses UTF8 characters.
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
    pub const fn modern() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::MODERN)
    }

    /// Rounded style looks like the following table.
    ///
    /// Beware: It uses UTF8 characters.
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
    pub const fn rounded() -> CustomStyle<On, On, On, On, (), On, On> {
        CustomStyle::new(Self::MODERN_ROUNDED)
    }

    /// Extended style looks like the following table
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
    pub const fn extended() -> CustomStyle<On, On, On, On, On, On, On> {
        CustomStyle::new(Self::EXTENDED)
    }

    /// ReStructuredText style looks like the following table
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
    pub const fn re_structured_text() -> CustomStyle<On, On, (), (), (), On, On> {
        CustomStyle::new(Self::RE_STRUCTURED_TEXT)
    }

    const EMPTY: StyleSettings =
        StyleSettings::new(Frame::empty(), Line::empty(), Line::empty(), None);

    const ASCII: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('-', '+'),
            Line::new('-', '+'),
            Line::new('|', '+'),
            Line::new('|', '+'),
            ('+', '+', '+', '+'),
        ),
        Line::new('-', '+'),
        Line::new('-', '+'),
        Some('|'),
    );

    const BLANK: StyleSettings =
        StyleSettings::new(Frame::empty(), Line::empty(), Line::empty(), Some(' '));

    const PSQL: StyleSettings = StyleSettings::new(
        Frame::empty(),
        Line::empty(),
        Line::new('-', '+'),
        Some('|'),
    );

    const GITHUB_MARKDOWN: StyleSettings = StyleSettings::new(
        Frame::bordered(
            Line::empty(),
            Line::empty(),
            Line::new('|', '|'),
            Line::new('|', '|'),
        ),
        Line::empty(),
        Line::new('-', '+'),
        Some('|'),
    );

    const MODERN: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('─', '┬'),
            Line::new('─', '┴'),
            Line::new('│', '├'),
            Line::new('│', '┤'),
            ('┌', '┐', '└', '┘'),
        ),
        Line::new('─', '┼'),
        Line::new('─', '┼'),
        Some('│'),
    );

    const MODERN_ROUNDED: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('─', '┬'),
            Line::new('─', '┴'),
            Line::new('│', '├'),
            Line::new('│', '┤'),
            ('╭', '╮', '╰', '╯'),
        ),
        Line::empty(),
        Line::new('─', '┼'),
        Some('│'),
    );

    const EXTENDED: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('═', '╦'),
            Line::new('═', '╩'),
            Line::new('║', '╠'),
            Line::new('║', '╣'),
            ('╔', '╗', '╚', '╝'),
        ),
        Line::new('═', '╬'),
        Line::new('═', '╬'),
        Some('║'),
    );

    const DOTS: StyleSettings = StyleSettings::new(
        Frame::full(
            Line::new('.', '.'),
            Line::new('.', ':'),
            Line::new(':', ':'),
            Line::new(':', ':'),
            ('.', '.', ':', ':'),
        ),
        Line::new('.', ':'),
        Line::new('.', ':'),
        Some(':'),
    );

    const RE_STRUCTURED_TEXT: StyleSettings = StyleSettings::new(
        Frame::bordered(
            Line::new('=', ' '),
            Line::new('=', ' '),
            Line::empty(),
            Line::empty(),
        ),
        Line::empty(),
        Line::new('=', ' '),
        Some(' '),
    );
}

#[derive(Debug, Clone)]
struct StyleSettings {
    frame: Frame,
    horizontal: Line,
    header: Line,
    vertical: Option<char>,
}

impl StyleSettings {
    const fn new(frame: Frame, horizontal: Line, header: Line, vertical: Option<char>) -> Self {
        Self {
            frame,
            horizontal,
            header,
            vertical,
        }
    }

    const fn has_vertical(&self) -> bool {
        self.horizontal.intersection.is_some() || self.vertical.is_some()
    }
}

/// Line represents a horizontal line on a [Table].
#[derive(Debug, Clone, Default)]
struct Line {
    main: Option<char>,
    intersection: Option<char>,
}

impl Line {
    /// Create a new line.
    const fn new(main: char, intersection: char) -> Self {
        Self {
            main: Some(main),
            intersection: Some(intersection),
        }
    }

    /// A line which doesn't exists.
    const fn empty() -> Self {
        Self {
            main: None,
            intersection: None,
        }
    }

    /// Chechks if any symbol is set in line.
    const fn is_empty(&self) -> bool {
        self.main.is_none() && self.intersection.is_none()
    }
}

#[derive(Debug, Clone, Default)]
struct Frame {
    top: Line,
    bottom: Line,
    left: Line,
    right: Line,
    corner_top_left: Option<char>,
    corner_top_right: Option<char>,
    corner_bottom_left: Option<char>,
    corner_bottom_right: Option<char>,
}

impl Frame {
    /// Creates a full border.
    const fn full(
        top: Line,
        bottom: Line,
        left: Line,
        right: Line,
        (top_left, top_right, bottom_left, bottom_right): (char, char, char, char),
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            corner_top_left: Some(top_left),
            corner_top_right: Some(top_right),
            corner_bottom_left: Some(bottom_left),
            corner_bottom_right: Some(bottom_right),
        }
    }

    /// Creates a frame top, bottom, left and right borders.
    const fn bordered(top: Line, bottom: Line, left: Line, right: Line) -> Self {
        Self {
            bottom,
            top,
            left,
            right,
            corner_top_left: None,
            corner_top_right: None,
            corner_bottom_left: None,
            corner_bottom_right: None,
        }
    }

    /// Creates an empty border.
    const fn empty() -> Self {
        Self {
            bottom: Line::empty(),
            top: Line::empty(),
            left: Line::empty(),
            right: Line::empty(),
            corner_top_left: None,
            corner_top_right: None,
            corner_bottom_left: None,
            corner_bottom_right: None,
        }
    }
}

impl TableOption for StyleSettings {
    fn change(&mut self, grid: &mut Grid) {
        let borders = Borders {
            top: self.frame.top.main.map(Symbol::from_char),
            top_intersection: self.frame.top.intersection.map(Symbol::from_char),
            bottom: self.frame.bottom.main.map(Symbol::from_char),
            bottom_intersection: self.frame.bottom.intersection.map(Symbol::from_char),
            horizontal_left: self.frame.left.intersection.map(Symbol::from_char),
            horizontal_right: self.frame.right.intersection.map(Symbol::from_char),
            top_left: self.frame.corner_top_left.map(Symbol::from_char),
            top_right: self.frame.corner_top_right.map(Symbol::from_char),
            bottom_left: self.frame.corner_bottom_left.map(Symbol::from_char),
            bottom_right: self.frame.corner_bottom_right.map(Symbol::from_char),
            horizontal: self.horizontal.main.map(Symbol::from_char),
            intersection: self.horizontal.intersection.map(Symbol::from_char),
            vertical_left: self.frame.left.main.map(Symbol::from_char),
            vertical_intersection: self.vertical.map(Symbol::from_char),
            vertical_right: self.frame.right.main.map(Symbol::from_char),
        };

        grid.clear_theme();
        grid.set_borders(borders);

        if grid.count_rows() > 1 {
            grid.set_split_line(
                1,
                papergrid::Line {
                    horizontal: self.header.main.map(Symbol::from_char),
                    intersection: self.header.intersection.map(Symbol::from_char),
                    ..Default::default()
                },
            );
        }
    }
}

/// TopBorderText writes a custom text on a top border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::BorderText};
/// let table = Table::new(["Hello World"])
///     .with(BorderText::first("+-.table"));
///
/// assert_eq!(
///     table.to_string(),
///     "+-.table------+\n\
///      |    &str     |\n\
///      +-------------+\n\
///      | Hello World |\n\
///      +-------------+\n"
/// );
/// ```
pub struct BorderText<'a> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
    row: SplitLineIndex,
}

enum SplitLineIndex {
    First,
    Last,
    Line(usize),
}

impl<'a> BorderText<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Line(line),
        }
    }

    pub fn first<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::First,
        }
    }

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

        grid.override_split_line(row, self.text.as_ref())
    }
}

/// CustomStyle represents a style controlling a valid state of it.
///
/// For example.
/// It doesn't allow to call method [CustomStyle::top_left_corner] unless [CustomStyle::left] and [CustomStyle::top] is set.
#[derive(Debug, Clone)]
pub struct CustomStyle<Top, Bottom, Left, Right, Horizontal, Vertical, Header> {
    inner: StyleSettings,
    _l_border: PhantomData<Left>,
    _r_border: PhantomData<Right>,
    _t_border: PhantomData<Top>,
    _b_border: PhantomData<Bottom>,
    _i_h_border: PhantomData<Horizontal>,
    _i_v_border: PhantomData<Vertical>,
    _h_border: PhantomData<Header>,
}

#[derive(Debug, Clone)]
pub struct On;

impl<Top, Bottom, Left, Rright, Horizontal, Vertical, Header>
    CustomStyle<Top, Bottom, Left, Rright, Horizontal, Vertical, Header>
{
    const fn new(style: StyleSettings) -> Self {
        Self {
            inner: style,
            _b_border: PhantomData,
            _l_border: PhantomData,
            _r_border: PhantomData,
            _t_border: PhantomData,
            _i_h_border: PhantomData,
            _i_v_border: PhantomData,
            _h_border: PhantomData,
        }
    }

    /// Frame function returns a frame as a border.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Style, Highlight, object::Rows};
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
    ///         "+----------+-------+\n",
    ///     )
    /// );
    /// ```
    pub const fn frame(&self) -> Border {
        Border {
            top: char_to_symbol(self.inner.frame.top.main),
            bottom: char_to_symbol(self.inner.frame.bottom.main),
            left: char_to_symbol(self.inner.frame.left.main),
            right: char_to_symbol(self.inner.frame.right.main),
            left_top_corner: char_to_symbol(self.inner.frame.corner_top_left),
            left_bottom_corner: char_to_symbol(self.inner.frame.corner_bottom_left),
            right_top_corner: char_to_symbol(self.inner.frame.corner_top_right),
            right_bottom_corner: char_to_symbol(self.inner.frame.corner_bottom_right),
        }
    }
}

const fn char_to_symbol(c: Option<char>) -> Option<Symbol> {
    match c {
        Some(c) => Some(Symbol::from_char(c)),
        None => None,
    }
}

impl<T, B, L, R, IH, IV, H> CustomStyle<T, B, L, R, IH, IV, H> {
    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn top(self, c: char) -> CustomStyle<On, B, L, R, IH, IV, H> {
        let mut style = self.inner;

        style.frame.top.main = Some(c);

        if !style.frame.left.is_empty() {
            style.frame.corner_top_left = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_top_right = Some(c);
        }

        if style.has_vertical() {
            style.frame.top.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn bottom(self, c: char) -> CustomStyle<T, On, L, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.bottom.main = Some(c);

        if !style.frame.left.is_empty() {
            style.frame.corner_bottom_left = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_bottom_right = Some(c);
        }

        if style.has_vertical() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn left(self, c: char) -> CustomStyle<T, B, On, R, IH, IV, H> {
        let mut style = self.inner;
        style.frame.left.main = Some(c);

        if !style.frame.top.is_empty() {
            style.frame.corner_top_left = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_left = Some(c);
        }

        if !style.horizontal.is_empty() {
            style.frame.left.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn right(self, c: char) -> CustomStyle<T, B, L, On, IH, IV, H> {
        let mut style = self.inner;
        style.frame.right.main = Some(c);

        if !style.frame.top.is_empty() {
            style.frame.corner_top_right = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_right = Some(c);
        }

        if !style.horizontal.is_empty() {
            style.frame.right.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a horizontal split line.
    ///
    /// It doesn't include a header split line.
    /// It must be set via its own method [Self::header].
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn horizontal(self, c: char) -> CustomStyle<T, B, L, R, On, IV, H> {
        let mut style = self.inner;
        style.horizontal.main = Some(c);

        if style.horizontal.intersection.is_some() {
            style.horizontal.intersection = Some(c);
        }

        if style.vertical.is_some() {
            style.horizontal.intersection = Some(c);
        }

        if !style.frame.left.is_empty() {
            style.frame.left.intersection = Some(c);
        }

        if !style.frame.right.is_empty() {
            style.frame.right.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn vertical(self, c: char) -> CustomStyle<T, B, L, R, IH, On, H> {
        let mut style = self.inner;
        style.vertical = Some(c);

        if !style.horizontal.is_empty() {
            style.horizontal.intersection = Some(c);
        }

        if !style.header.is_empty() {
            style.header.intersection = Some(c);
        }

        if !style.frame.top.is_empty() {
            style.frame.top.intersection = Some(c);
        }

        if !style.frame.bottom.is_empty() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a 1st horizontal split line.
    ///
    /// Any corners and intersections which were set will be overriden.
    pub const fn header(self, c: char) -> CustomStyle<T, B, L, R, IH, IV, On> {
        let mut style = self.inner;
        style.header.main = Some(c);

        if style.vertical.is_some() {
            style.header.intersection = Some(c);
        }

        CustomStyle::new(style)
    }
}

impl<B, R, IH, IV, H> CustomStyle<On, B, On, R, IH, IV, H> {
    /// Sets a top left corner.
    pub const fn top_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_top_left = Some(c);

        CustomStyle::new(style)
    }
}

impl<B, L, IH, IV, H> CustomStyle<On, B, L, On, IH, IV, H> {
    /// Sets a top right corner.
    pub const fn top_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_top_right = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, L, IH, IV, H> CustomStyle<T, On, L, On, IH, IV, H> {
    /// Sets a bottom right corner.
    pub const fn bottom_right_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_bottom_right = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, R, IH, IV, H> CustomStyle<T, On, On, R, IH, IV, H> {
    /// Sets a bottom left corner.
    pub const fn bottom_left_corner(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.corner_bottom_left = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, R, IV, H> CustomStyle<T, B, On, R, On, IV, H> {
    /// Sets a left intersection char.
    pub const fn left_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.left.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, L, IV, H> CustomStyle<T, B, L, On, On, IV, H> {
    /// Sets a right intersection char.
    pub const fn right_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.right.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<B, L, R, IH, H> CustomStyle<On, B, L, R, IH, On, H> {
    /// Sets a top intersection char.
    pub const fn top_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.top.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, H> CustomStyle<T, On, L, R, IH, On, H> {
    /// Sets a bottom intersection char.
    pub const fn bottom_intersection(self, c: char) -> Self {
        let mut style = self.inner;
        style.frame.bottom.intersection = Some(c);

        CustomStyle::new(style)
    }
}

impl<T, B, L, R, H> CustomStyle<T, B, L, R, On, On, H> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub const fn inner_intersection(mut self, c: char) -> Self {
        self.inner.horizontal.intersection = Some(c);
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH> CustomStyle<T, B, L, R, IH, On, On> {
    /// Sets an intersection char of a 1st horizontal split line.
    pub const fn header_intersection(mut self, c: char) -> Self {
        self.inner.header.intersection = Some(c);
        CustomStyle::new(self.inner)
    }
}

impl<B, L, R, IH, IV, H> CustomStyle<On, B, L, R, IH, IV, H> {
    /// Removes top border.
    pub const fn top_off(mut self) -> CustomStyle<(), B, L, R, IH, IV, H> {
        self.inner.frame.top = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_top_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, L, R, IH, IV, H> CustomStyle<T, On, L, R, IH, IV, H> {
    /// Removes bottom border.
    pub const fn bottom_off(mut self) -> CustomStyle<T, (), L, R, IH, IV, H> {
        self.inner.frame.bottom = Line::empty();
        self.inner.frame.corner_bottom_left = None;
        self.inner.frame.corner_bottom_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, R, IH, IV, H> CustomStyle<T, B, On, R, IH, IV, H> {
    /// Removes left border.
    pub const fn left_off(mut self) -> CustomStyle<T, B, (), R, IH, IV, H> {
        self.inner.frame.left = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_bottom_left = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, IH, IV, H> CustomStyle<T, B, L, On, IH, IV, H> {
    /// Removes right border.
    pub const fn right_off(mut self) -> CustomStyle<T, B, L, (), IH, IV, H> {
        self.inner.frame.right = Line::empty();
        self.inner.frame.corner_top_right = None;
        self.inner.frame.corner_bottom_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IV, H> CustomStyle<T, B, L, R, On, IV, H> {
    /// Removes horizontal split lines.
    ///
    /// Not including 1st split line.
    pub const fn horizontal_off(mut self) -> CustomStyle<T, B, L, R, (), IV, H> {
        self.inner.horizontal = Line::empty();

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, H> CustomStyle<T, B, L, R, IH, On, H> {
    /// Removes vertical split lines.
    pub const fn vertical_off(mut self) -> CustomStyle<T, B, L, R, IH, (), H> {
        self.inner.vertical = None;
        self.inner.horizontal.intersection = None;
        self.inner.header.intersection = None;
        self.inner.frame.top.intersection = None;
        self.inner.frame.bottom.intersection = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV> CustomStyle<T, B, L, R, IH, IV, On> {
    /// Removes 1st horizontal split line.
    pub const fn header_off(mut self) -> CustomStyle<T, B, L, R, IH, IV, ()> {
        self.inner.header = Line::empty();
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV, H> TableOption for CustomStyle<T, B, L, R, IH, IV, H> {
    fn change(&mut self, grid: &mut Grid) {
        self.inner.change(grid);
    }
}

/// Border represents a border of a Cell.
///
/// ```rust,no_run
///   # use tabled::{style::{Style, Border}, object::Rows, Table, Modify};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Style::ascii())
///         .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
pub use papergrid::Border;

impl CellOption for Border {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, col: usize) {
        grid.set(
            Entity::Cell(row, col),
            Settings::default().border(self.clone()),
        );
    }
}

/// Symbol represents a character of a border.
///
/// It's only needed when used with `color` feature flag.
///
/// ```rust,no_run
///   # use owo_colors::OwoColorize;
///   # use tabled::{style::{Border, Symbol}, object::Rows, TableIteratorExt, Modify};
///   #
///   # let data: Vec<&'static str> = Vec::new();
///     let colored_char = "#".red().to_string();
///     let table = data.table()
///         .with(Modify::new(Rows::single(0)).with(Border::filled(Symbol::ansi(colored_char).unwrap())));
/// ```
#[cfg(feature = "color")]
pub use papergrid::Symbol;

#[cfg(not(feature = "color"))]
use papergrid::Symbol;
