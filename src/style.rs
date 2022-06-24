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
//!         " 2022  ",
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
//!         " 2022  ",
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
//!         "  2022   ",
//!     )
//! )
//! ```
//!
//! [Table]: crate::Table

use std::{borrow::Cow, marker::PhantomData};

use papergrid::{Borders, Entity, Grid, Position};

use crate::{CellOption, TableOption};

/// Style is represents a theme of a [Table].
///
/// It can be Modified extensively, look at [CustomStyle] methods.
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
    /// It's useful as a scratch style to build a custom one.
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
        CustomStyle::new(EMPTY)
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
        CustomStyle::new(BLANK)
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
        CustomStyle::new(ASCII)
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
        CustomStyle::new(DOTS)
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
        CustomStyle::new(PSQL)
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
        CustomStyle::new(GITHUB_MARKDOWN)
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
        CustomStyle::new(MODERN)
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
        CustomStyle::new(MODERN_ROUNDED)
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
        CustomStyle::new(EXTENDED)
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
        CustomStyle::new(RE_STRUCTURED_TEXT)
    }

    /// Try to fix the style when table contains spans.
    ///
    /// By default [Style] doesn't implies any logic to better render split lines when
    /// [crate::Span] is used.
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
    pub const fn correct_spans() -> StyleCorrectSpan {
        StyleCorrectSpan
    }
}

const EMPTY: StyleSettings = StyleSettings::new(Frame::empty(), Line::empty(), Line::empty(), None);

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

/// A raw style data, which can be produced safely from [CustomStyle].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone)]
pub struct StyleSettings {
    frame: Frame,
    horizontal: Line,
    header: Line,
    vertical: Option<Symbol>,
}

impl StyleSettings {
    const fn new(frame: Frame, horizontal: Line, header: Line, vertical: Option<char>) -> Self {
        Self {
            frame,
            horizontal,
            header,
            vertical: char_to_symbol(vertical),
        }
    }

    const fn has_vertical(&self) -> bool {
        self.horizontal.intersection.is_some() || self.vertical.is_some()
    }

    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.top.main = s;
        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.bottom.main = s;
        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.left.main = s;
        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.right.main = s;
        self
    }

    /// Set a top split border character.
    pub fn set_top_split(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.top.intersection = s;
        self
    }

    /// Set a bottom split character.
    pub fn set_bottom_split(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.bottom.intersection = s;
        self
    }

    /// Set a left split character.
    pub fn set_left_split(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.left.intersection = s;
        self
    }

    /// Set a right split character.
    pub fn set_right_split(&mut self, s: Option<Symbol>) -> &mut Self {
        self.frame.right.intersection = s;
        self
    }

    /// Set an internal character.
    pub fn set_internal(&mut self, s: Option<Symbol>) -> &mut Self {
        self.horizontal.intersection = s.clone();
        self.header.intersection = s;
        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<Symbol>) -> &mut Self {
        self.vertical = s;
        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<Symbol>) -> &mut Self {
        self.horizontal.main = s;
        self
    }

    /// This function runs a function for each border character and changes it accordingly.
    ///
    /// See [CustomStyle::try_map]
    #[cfg(feature = "color")]
    pub fn try_map<F, S>(mut self, f: F) -> Self
    where
        F: Fn(Symbol) -> S,
        S: Into<Symbol>,
    {
        self.frame.left.main = map_symbol(self.frame.left.main, &f);
        self.frame.left.intersection = map_symbol(self.frame.left.intersection, &f);
        self.frame.right.main = map_symbol(self.frame.right.main, &f);
        self.frame.right.intersection = map_symbol(self.frame.right.intersection, &f);
        self.frame.top.main = map_symbol(self.frame.top.main, &f);
        self.frame.top.intersection = map_symbol(self.frame.top.intersection, &f);
        self.frame.bottom.main = map_symbol(self.frame.bottom.main, &f);
        self.frame.bottom.intersection = map_symbol(self.frame.bottom.intersection, &f);
        self.frame.corner_bottom_left = map_symbol(self.frame.corner_bottom_left, &f);
        self.frame.corner_top_left = map_symbol(self.frame.corner_top_left, &f);
        self.frame.corner_bottom_right = map_symbol(self.frame.corner_bottom_right, &f);
        self.frame.corner_top_right = map_symbol(self.frame.corner_top_right, &f);

        self.header.main = map_symbol(self.header.main, &f);
        self.header.intersection = map_symbol(self.header.intersection, &f);

        self.horizontal.main = map_symbol(self.horizontal.main, &f);
        self.horizontal.intersection = map_symbol(self.horizontal.intersection, &f);

        self.vertical = map_symbol(self.vertical, &f);

        self
    }
}

/// Line represents a horizontal line on a [Table].
#[derive(Debug, Clone, Default)]
struct Line {
    main: Option<Symbol>,
    intersection: Option<Symbol>,
}

impl Line {
    /// Create a new line.
    const fn new(main: char, intersection: char) -> Self {
        Self {
            main: Some(Symbol::from_char(main)),
            intersection: Some(Symbol::from_char(intersection)),
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
    corner_top_left: Option<Symbol>,
    corner_top_right: Option<Symbol>,
    corner_bottom_left: Option<Symbol>,
    corner_bottom_right: Option<Symbol>,
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
            corner_top_left: Some(Symbol::from_char(top_left)),
            corner_top_right: Some(Symbol::from_char(top_right)),
            corner_bottom_left: Some(Symbol::from_char(bottom_left)),
            corner_bottom_right: Some(Symbol::from_char(bottom_right)),
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
            top: self.frame.top.main.clone(),
            top_intersection: self.frame.top.intersection.clone(),
            bottom: self.frame.bottom.main.clone(),
            bottom_intersection: self.frame.bottom.intersection.clone(),
            horizontal_left: self.frame.left.intersection.clone(),
            horizontal_right: self.frame.right.intersection.clone(),
            top_left: self.frame.corner_top_left.clone(),
            top_right: self.frame.corner_top_right.clone(),
            bottom_left: self.frame.corner_bottom_left.clone(),
            bottom_right: self.frame.corner_bottom_right.clone(),
            horizontal: self.horizontal.main.clone(),
            intersection: self.horizontal.intersection.clone(),
            vertical_left: self.frame.left.main.clone(),
            vertical_intersection: self.vertical.clone(),
            vertical_right: self.frame.right.main.clone(),
        };

        grid.clear_theme();
        grid.set_borders(borders);

        if grid.count_rows() > 1 {
            grid.set_split_line(
                1,
                papergrid::Line {
                    horizontal: self.header.main.clone(),
                    intersection: self.header.intersection.clone(),
                    ..Default::default()
                },
            );
        }
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

impl<Top, Bottom, Left, Right, Horizontal, Vertical, Header>
    From<CustomStyle<Top, Bottom, Left, Right, Horizontal, Vertical, Header>> for StyleSettings
{
    fn from(val: CustomStyle<Top, Bottom, Left, Right, Horizontal, Vertical, Header>) -> Self {
        val.inner
    }
}

/// A marker struct which is used in [CustomStyle].
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
    ///         "+----------+-------+",
    ///     )
    /// );
    /// ```
    pub fn frame(&self) -> Border {
        Border {
            top: self.inner.frame.top.main.clone(),
            bottom: self.inner.frame.bottom.main.clone(),
            left: self.inner.frame.left.main.clone(),
            right: self.inner.frame.right.main.clone(),
            left_top_corner: self.inner.frame.corner_top_left.clone(),
            left_bottom_corner: self.inner.frame.corner_bottom_left.clone(),
            right_top_corner: self.inner.frame.corner_top_right.clone(),
            right_bottom_corner: self.inner.frame.corner_bottom_right.clone(),
        }
    }

    /// This function runs a function for each border character and changes it accordingly.
    ///
    /// It may be useful when you wan't to colorize the borders.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, style::{Style, Symbol}};
    /// use owo_colors::OwoColorize;
    ///
    /// let data = [["10:52:19", "Hello"], ["10:52:20", "World"]];
    /// let table = Table::new(data)
    ///     .with(Style::modern().try_map(|s| Symbol::ansi(s.blue().to_string()).unwrap()));
    ///
    /// println!("{}", table);
    /// ```
    #[cfg(feature = "color")]
    pub fn try_map<F, S>(mut self, f: F) -> Self
    where
        F: Fn(Symbol) -> S,
        S: Into<Symbol>,
    {
        self.inner = self.inner.try_map(f);

        self
    }
}

#[cfg(feature = "color")]
fn map_symbol<F, S>(symbol: Option<Symbol>, f: F) -> Option<Symbol>
where
    F: Fn(Symbol) -> S,
    S: Into<Symbol>,
{
    symbol.map(|s| (f)(s).into())
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
    /// Any corners and intersections which were set will be overridden.
    pub fn top<S>(self, c: S) -> CustomStyle<On, B, L, R, IH, IV, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;

        style.frame.top.main = Some(c.clone());

        if !style.frame.left.is_empty() {
            style.frame.corner_top_left = Some(c.clone());
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_top_right = Some(c.clone());
        }

        if style.has_vertical() {
            style.frame.top.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn bottom<S>(self, c: S) -> CustomStyle<T, On, L, R, IH, IV, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.frame.bottom.main = Some(c.clone());

        if !style.frame.left.is_empty() {
            style.frame.corner_bottom_left = Some(c.clone());
        }

        if !style.frame.right.is_empty() {
            style.frame.corner_bottom_right = Some(c.clone());
        }

        if style.has_vertical() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn left<S>(self, c: S) -> CustomStyle<T, B, On, R, IH, IV, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.frame.left.main = Some(c.clone());

        if !style.frame.top.is_empty() {
            style.frame.corner_top_left = Some(c.clone());
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_left = Some(c.clone());
        }

        if !style.horizontal.is_empty() {
            style.frame.left.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn right<S>(self, c: S) -> CustomStyle<T, B, L, On, IH, IV, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.frame.right.main = Some(c.clone());

        if !style.frame.top.is_empty() {
            style.frame.corner_top_right = Some(c.clone());
        }

        if !style.frame.bottom.is_empty() {
            style.frame.corner_bottom_right = Some(c.clone());
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
    /// Any corners and intersections which were set will be overridden.
    pub fn horizontal<S>(self, c: S) -> CustomStyle<T, B, L, R, On, IV, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.horizontal.main = Some(c.clone());

        if style.horizontal.intersection.is_some() {
            style.horizontal.intersection = Some(c.clone());
        }

        if style.vertical.is_some() {
            style.horizontal.intersection = Some(c.clone());
        }

        if !style.frame.left.is_empty() {
            style.frame.left.intersection = Some(c.clone());
        }

        if !style.frame.right.is_empty() {
            style.frame.right.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn vertical<S>(self, c: S) -> CustomStyle<T, B, L, R, IH, On, H>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.vertical = Some(c.clone());

        if !style.horizontal.is_empty() {
            style.horizontal.intersection = Some(c.clone());
        }

        if !style.header.is_empty() {
            style.header.intersection = Some(c.clone());
        }

        if !style.frame.top.is_empty() {
            style.frame.top.intersection = Some(c.clone());
        }

        if !style.frame.bottom.is_empty() {
            style.frame.bottom.intersection = Some(c);
        }

        CustomStyle::new(style)
    }

    /// Sets a 1st horizontal split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn header<S>(self, c: S) -> CustomStyle<T, B, L, R, IH, IV, On>
    where
        S: Into<Symbol>,
    {
        let c = c.into();
        let mut style = self.inner;
        style.header.main = Some(c.clone());

        if style.vertical.is_some() {
            style.header.intersection = Some(c);
        }

        CustomStyle::new(style)
    }
}

impl<B, R, IH, IV, H> CustomStyle<On, B, On, R, IH, IV, H> {
    /// Sets a top left corner.
    pub fn top_left_corner<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.corner_top_left = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<B, L, IH, IV, H> CustomStyle<On, B, L, On, IH, IV, H> {
    /// Sets a top right corner.
    pub fn top_right_corner<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.corner_top_right = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, L, IH, IV, H> CustomStyle<T, On, L, On, IH, IV, H> {
    /// Sets a bottom right corner.
    pub fn bottom_right_corner<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.corner_bottom_right = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, R, IH, IV, H> CustomStyle<T, On, On, R, IH, IV, H> {
    /// Sets a bottom left corner.
    pub fn bottom_left_corner<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.corner_bottom_left = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, B, R, IV, H> CustomStyle<T, B, On, R, On, IV, H> {
    /// Sets a left intersection char.
    pub fn left_intersection<I>(self, c: I) -> Self
    where
        I: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.left.intersection = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, B, L, IV, H> CustomStyle<T, B, L, On, On, IV, H> {
    /// Sets a right intersection char.
    pub fn right_intersection<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.right.intersection = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<B, L, R, IH, H> CustomStyle<On, B, L, R, IH, On, H> {
    /// Sets a top intersection char.
    pub fn top_intersection<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.top.intersection = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, L, R, IH, H> CustomStyle<T, On, L, R, IH, On, H> {
    /// Sets a bottom intersection char.
    pub fn bottom_intersection<S>(self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        let mut style = self.inner;
        style.frame.bottom.intersection = Some(c.into());

        CustomStyle::new(style)
    }
}

impl<T, B, L, R, H> CustomStyle<T, B, L, R, On, On, H> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub fn inner_intersection<S>(mut self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        self.inner.horizontal.intersection = Some(c.into());
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH> CustomStyle<T, B, L, R, IH, On, On> {
    /// Sets an intersection char of a 1st horizontal split line.
    pub fn header_intersection<S>(mut self, c: S) -> Self
    where
        S: Into<Symbol>,
    {
        self.inner.header.intersection = Some(c.into());
        CustomStyle::new(self.inner)
    }
}

impl<B, L, R, IH, IV, H> CustomStyle<On, B, L, R, IH, IV, H> {
    /// Removes top border.
    pub fn top_off(mut self) -> CustomStyle<(), B, L, R, IH, IV, H> {
        self.inner.frame.top = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_top_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, L, R, IH, IV, H> CustomStyle<T, On, L, R, IH, IV, H> {
    /// Removes bottom border.
    pub fn bottom_off(mut self) -> CustomStyle<T, (), L, R, IH, IV, H> {
        self.inner.frame.bottom = Line::empty();
        self.inner.frame.corner_bottom_left = None;
        self.inner.frame.corner_bottom_right = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, R, IH, IV, H> CustomStyle<T, B, On, R, IH, IV, H> {
    /// Removes left border.
    pub fn left_off(mut self) -> CustomStyle<T, B, (), R, IH, IV, H> {
        self.inner.frame.left = Line::empty();
        self.inner.frame.corner_top_left = None;
        self.inner.frame.corner_bottom_left = None;

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, IH, IV, H> CustomStyle<T, B, L, On, IH, IV, H> {
    /// Removes right border.
    pub fn right_off(mut self) -> CustomStyle<T, B, L, (), IH, IV, H> {
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
    pub fn horizontal_off(mut self) -> CustomStyle<T, B, L, R, (), IV, H> {
        self.inner.horizontal = Line::empty();

        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, H> CustomStyle<T, B, L, R, IH, On, H> {
    /// Removes vertical split lines.
    pub fn vertical_off(mut self) -> CustomStyle<T, B, L, R, IH, (), H> {
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
    pub fn header_off(mut self) -> CustomStyle<T, B, L, R, IH, IV, ()> {
        self.inner.header = Line::empty();
        CustomStyle::new(self.inner)
    }
}

impl<T, B, L, R, IH, IV, H> TableOption for CustomStyle<T, B, L, R, IH, IV, H> {
    fn change(&mut self, grid: &mut Grid) {
        self.inner.change(grid);
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
///      +-------------+"
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
    /// Creates a [BorderText] instance.
    ///
    /// Lines are numbered from 0 to the count_rows included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::Line(line),
        }
    }

    /// Creates a [BorderText] instance for a top line.
    pub fn first<S: Into<Cow<'a, str>>>(text: S) -> Self {
        Self {
            text: text.into(),
            row: SplitLineIndex::First,
        }
    }

    /// Creates a [BorderText] instance for a bottom line.
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
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        grid.set_border(entity, self.clone());
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

/// A correctnes function of style for [crate::Table] which has [crate::Span]s.
///
/// See [Style::correct_spans].
#[derive(Debug)]
pub struct StyleCorrectSpan;

impl TableOption for StyleCorrectSpan {
    fn change(&mut self, grid: &mut Grid) {
        correct_span_styles(grid);
    }
}

fn correct_span_styles(grid: &mut Grid) {
    let borders = grid.get_borders();
    let inner_intersection = borders.intersection.clone();
    let bottom_intersection = borders.bottom_intersection.clone();
    let top_intersection = borders.top_intersection.clone();

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

            let mut border = grid.get_border(row, col);

            let has_top_border = border.left_top_corner.is_some() && border.top.is_some();
            if has_top_border {
                if has_up && is_first {
                    border.left_top_corner = inner_intersection.clone();
                } else if has_up {
                    border.left_top_corner = bottom_intersection.clone();
                } else if is_first {
                    border.left_top_corner = top_intersection.clone();
                } else {
                    border.left_top_corner = border.top.clone();
                }
            }

            let has_bottom_border = border.left_bottom_corner.is_some() && border.bottom.is_some();
            if has_bottom_border {
                if has_down && is_first {
                    border.left_bottom_corner = inner_intersection.clone();
                } else if has_down {
                    border.left_bottom_corner = top_intersection.clone();
                } else if is_first {
                    border.left_bottom_corner = bottom_intersection.clone();
                } else {
                    border.left_bottom_corner = border.bottom.clone();
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
        let border = grid.get_border(pos.0, pos.1);
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
