//! This module contains a compile time style builder [`Style`].

use core::marker::PhantomData;

use crate::{
    grid::config::{Borders, CompactConfig, CompactMultilineConfig},
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::grid::config::ColoredConfig;

use super::{HorizontalLine, Line, VerticalLine};

/// Style is represents a theme of a [`Table`].
///
/// ```text
/// corner top left         top intersection                    corner top right
///                .             |                             .
///                 .            V                            .
///                  ╭───┬───┬───┬───┬───┬───┬────┬────┬────╮
///                  │ i │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │
///                  ├───┼───┼───┼───┼───┼───┼────┼────┼────┤ <- this horizontal line is custom 'horizontals'
///                  │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0  │ 0  │ 0  │    other lines horizontal lines are not set they called 'horizontal'
///                  │ 1 │ 0 │ 1 │ 2 │ 3 │ 4 │ 5  │ 6  │ 7  │
///                  │ 2 │ 0 │ 2 │ 4 │ 6 │ 8 │ 10 │ 12 │ 14 │
///                  ╰───┴───┴───┴───┴───┴───┴────┴────┴────╯
///                .         ^                    ^           .
///               .          |                    |            .
/// corner bottom left       |         bottom intersection     corner bottom right
///                          |
///                          |
///             all this vertical lines are called 'vertical'
/// ```
///
///
/// ```text
///                     ┌───┬───┬───┬───┬───┐
///                     │ 0 │ 1 │ 2 │ 3 │ 4 │
/// intersection left ->├───X───X───X───X───┤ <- all this horizontal lines are called 'horizontal'
///                     │ 1 │ 2 │ 3 │ 4 │ 5 │
///                     ├───X───X───X───X───┤ <- intersection right
///                     │ 2 │ 3 │ 4 │ 5 │ 6 │
///                     └───┴───┴───┴───┴───┘
///
/// All 'X' positions are called 'intersection'.
/// It's a place where 'vertical' and 'horizontal' lines intersect.
/// ```
///
/// It tries to limit an controlling a valid state of it.
/// For example, it won't allow to call method [`Style::corner_top_left`] unless [`Style::left`] and [`Style::top`] is set.
///
/// You can turn [`Style`] into [`RawStyle`] to have more control using [`Into`] implementation.
///
/// # Example
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// use tabled::{Table, settings::Style};
///
/// let style = Style::ascii()
///                 .bottom('*')
///                 .intersection(' ');
///
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(style).to_string();
///
/// println!("{}", table);
/// ```
///
/// [`Table`]: crate::Table
/// [`RawStyle`]: crate::settings::style::RawStyle
/// [`Style::corner_top_left`]: Style::corner_top_left
/// [`Style::left`]: Style.left
/// [`Style::top`]: Style.function.top
#[derive(Debug, Clone)]
pub struct Style<T, B, L, R, H, V, HLines = HLineArray<0>, VLines = VLineArray<0>> {
    borders: Borders<char>,
    horizontals: HLines,
    verticals: VLines,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
    _horizontal: PhantomData<H>,
    _vertical: PhantomData<V>,
}

type HLineArray<const N: usize> = [HorizontalLine; N];

type VLineArray<const N: usize> = [VerticalLine; N];

/// A marker struct which is used in [`Style`].
#[derive(Debug, Clone)]
pub struct On;

impl Style<(), (), (), (), (), (), (), ()> {
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
    /// # use tabled::settings::Style;
    /// let style = Style::empty()
    ///     .top('*')
    ///     .bottom('*')
    ///     .vertical('#')
    ///     .intersection_top('*');
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
    pub const fn psql() -> Style<(), (), (), (), (), On, HLineArray<1>> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                None,
                None,
                Some('|'),
            ),
            [HorizontalLine::new(1, Line::empty())
                .main(Some('-'))
                .intersection(Some('+'))],
            [],
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
    pub const fn markdown() -> Style<(), (), On, On, (), On, HLineArray<1>> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                Some('|'),
                Some('|'),
                Some('|'),
            ),
            [HorizontalLine::new(1, Line::full('-', '|', '|', '|'))],
            [],
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
            [],
        )
    }

    /// This style looks like a [`Style::modern`] but without horozizontal lines except a header.
    ///
    /// Beware: It uses UTF-8 characters.
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const fn sharp() -> Style<On, On, On, On, (), On, HLineArray<1>> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '┌', '┐'),
                Line::full('─', '┴', '└', '┘'),
                Line::empty(),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [HorizontalLine::new(1, Line::full('─', '┼', '├', '┤'))],
            [],
        )
    }

    /// This style looks like a [`Style::sharp`] but with rounded corners.
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
    pub const fn rounded() -> Style<On, On, On, On, (), On, HLineArray<1>> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '╭', '╮'),
                Line::full('─', '┴', '╰', '╯'),
                Line::empty(),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [HorizontalLine::new(1, Line::full('─', '┼', '├', '┤'))],
            [],
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
    pub const fn re_structured_text() -> Style<On, On, (), (), (), On, HLineArray<1>> {
        Style::new(
            create_borders(
                Line::new(Some('='), Some(' '), None, None),
                Line::new(Some('='), Some(' '), None, None),
                Line::empty(),
                None,
                None,
                Some(' '),
            ),
            [HorizontalLine::new(
                1,
                Line::new(Some('='), Some(' '), None, None),
            )],
            [],
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
            [],
        )
    }
}

impl<T, B, L, R, H, V, HLines, VLines> Style<T, B, L, R, H, V, HLines, VLines> {
    /// Frame function returns a frame as a border.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::{Style, Highlight, object::Rows}};
    ///
    /// let data = [["10:52:19", "Hello"], ["10:52:20", "World"]];
    /// let table = Table::new(data)
    ///     .with(Highlight::new(Rows::first(), Style::modern().get_frame()))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
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
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub const fn get_frame(&self) -> super::Border {
        let mut border = super::Border::filled(' ');

        if let Some(c) = self.borders.top {
            border = border.top(c);
        }

        if let Some(c) = self.borders.bottom {
            border = border.bottom(c);
        }

        if let Some(c) = self.borders.left {
            border = border.left(c);
        }

        if let Some(c) = self.borders.right {
            border = border.right(c);
        }

        if let Some(c) = self.borders.top_left {
            border = border.corner_top_left(c);
        }

        if let Some(c) = self.borders.bottom_left {
            border = border.corner_bottom_left(c);
        }

        if let Some(c) = self.borders.top_right {
            border = border.corner_top_right(c);
        }

        if let Some(c) = self.borders.bottom_right {
            border = border.corner_bottom_right(c);
        }

        border
    }

    /// Get a [`Style`]'s default horizontal line.
    ///
    /// It doesn't return an overloaded line via [`Style::horizontals`].
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "std", doc = "```")]
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    /// use tabled::{settings::style::{Style, HorizontalLine, Line}, Table};
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", "World", i)))
    ///    .with(Style::ascii().remove_horizontal().horizontals([HorizontalLine::new(1, Style::modern().get_horizontal())]))
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
            self.borders.left_intersection,
            self.borders.right_intersection,
        )
    }

    /// Get a [`Style`]'s default horizontal line.
    ///
    /// It doesn't return an overloaded line via [`Style::verticals`].
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "std", doc = "```")]
    #[cfg_attr(not(feature = "std"), doc = "```ignore")]
    /// use tabled::{settings::style::{Style, VerticalLine, Line}, Table};
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", "World", i)))
    ///    .with(Style::ascii().remove_horizontal().verticals([VerticalLine::new(1, Style::modern().get_vertical())]))
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+-------┬-------+-----+\n",
    ///         "| &str  │ &str  | i32 |\n",
    ///         "| Hello │ World | 0   |\n",
    ///         "| Hello │ World | 1   |\n",
    ///         "| Hello │ World | 2   |\n",
    ///         "+-------┴-------+-----+",
    ///     )
    /// )
    /// ```
    pub const fn get_vertical(&self) -> Line {
        Line::new(
            self.borders.vertical,
            self.borders.intersection,
            self.borders.top_intersection,
            self.borders.bottom_intersection,
        )
    }

    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn top(mut self, c: char) -> Style<On, B, L, R, H, V, HLines, VLines>
    where
        for<'a> &'a mut VLines: IntoIterator<Item = &'a mut VerticalLine>,
    {
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

        for vl in &mut self.verticals {
            vl.line.connector1 = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn bottom(mut self, c: char) -> Style<T, On, L, R, H, V, HLines, VLines>
    where
        for<'a> &'a mut VLines: IntoIterator<Item = &'a mut VerticalLine>,
    {
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

        for vl in &mut self.verticals {
            vl.line.connector2 = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn left(mut self, c: char) -> Style<T, B, On, R, H, V, HLines, VLines>
    where
        for<'a> &'a mut HLines: IntoIterator<Item = &'a mut HorizontalLine>,
    {
        self.borders.left = Some(c);

        if self.borders.has_top() {
            self.borders.top_left = Some(c);
        }

        if self.borders.has_bottom() {
            self.borders.bottom_left = Some(c);
        }

        if self.borders.has_horizontal() {
            self.borders.left_intersection = Some(c);
        }

        for hl in &mut self.horizontals {
            hl.line.connector1 = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn right(mut self, c: char) -> Style<T, B, L, On, H, V, HLines, VLines>
    where
        for<'a> &'a mut HLines: IntoIterator<Item = &'a mut HorizontalLine>,
    {
        self.borders.right = Some(c);

        if self.borders.has_top() {
            self.borders.top_right = Some(c);
        }

        if self.borders.has_bottom() {
            self.borders.bottom_right = Some(c);
        }

        if self.borders.has_horizontal() {
            self.borders.right_intersection = Some(c);
        }

        for hl in &mut self.horizontals {
            hl.line.connector2 = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a horizontal split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn horizontal(mut self, c: char) -> Style<T, B, L, R, On, V, HLines, VLines>
    where
        for<'a> &'a mut VLines: IntoIterator<Item = &'a mut VerticalLine>,
    {
        self.borders.horizontal = Some(c);

        if self.borders.has_vertical() {
            self.borders.intersection = Some(c);
        }

        if self.borders.has_left() {
            self.borders.left_intersection = Some(c);
        }

        if self.borders.has_right() {
            self.borders.right_intersection = Some(c);
        }

        for vl in &mut self.verticals {
            vl.line.intersection = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub fn vertical(mut self, c: char) -> Style<T, B, L, R, H, On, HLines, VLines>
    where
        for<'a> &'a mut HLines: IntoIterator<Item = &'a mut HorizontalLine>,
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

        for hl in &mut self.horizontals {
            hl.line.intersection = Some(c);
        }

        Style::new(self.borders, self.horizontals, self.verticals)
    }

    /// Set border horizontal lines.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{settings::style::{Style, HorizontalLine, Line}, Table};
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", i)))
    ///    .with(Style::rounded().horizontals((1..4).map(|i| HorizontalLine::new(i, Line::filled('#')))))
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
    pub fn horizontals<NewLines>(self, lines: NewLines) -> Style<T, B, L, R, H, V, NewLines, VLines>
    where
        NewLines: IntoIterator<Item = HorizontalLine> + Clone,
    {
        Style::new(self.borders, lines, self.verticals)
    }

    /// Set border vertical lines.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{Table, settings::style::{Style, VerticalLine, Line}};
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", i)))
    ///    .with(Style::rounded().verticals((0..3).map(|i| VerticalLine::new(i, Line::filled('#')))))
    ///    .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "#───────#─────#\n",
    ///         "# &str  # i32 #\n",
    ///         "├───────┼─────┤\n",
    ///         "# Hello # 0   #\n",
    ///         "# Hello # 1   #\n",
    ///         "# Hello # 2   #\n",
    ///         "#───────#─────#",
    ///     )
    /// )
    /// ```
    pub fn verticals<NewLines>(self, lines: NewLines) -> Style<T, B, L, R, H, V, HLines, NewLines>
    where
        NewLines: IntoIterator<Item = VerticalLine> + Clone,
    {
        Style::new(self.borders, self.horizontals, lines)
    }

    /// Removes all horizontal lines set by [`Style::horizontals`]
    pub fn remove_horizontals(self) -> Style<T, B, L, R, H, V, HLineArray<0>, VLines> {
        Style::new(self.borders, [], self.verticals)
    }

    /// Removes all verticals lines set by [`Style::verticals`]
    pub fn remove_verticals(self) -> Style<T, B, L, R, H, V, HLines, VLineArray<0>> {
        Style::new(self.borders, self.horizontals, [])
    }
}

impl<B, R, H, V, HLines, VLines> Style<On, B, On, R, H, V, HLines, VLines> {
    /// Sets a top left corner.
    pub fn corner_top_left(mut self, c: char) -> Self {
        self.borders.top_left = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, H, V, HLines, VLines> Style<On, B, L, On, H, V, HLines, VLines> {
    /// Sets a top right corner.
    pub fn corner_top_right(mut self, c: char) -> Self {
        self.borders.top_right = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, H, V, HLines, VLines> Style<T, On, L, On, H, V, HLines, VLines> {
    /// Sets a bottom right corner.
    pub fn corner_bottom_right(mut self, c: char) -> Self {
        self.borders.bottom_right = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, R, H, V, HLines, VLines> Style<T, On, On, R, H, V, HLines, VLines> {
    /// Sets a bottom left corner.
    pub fn corner_bottom_left(mut self, c: char) -> Self {
        self.borders.bottom_left = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, R, V, HLines, VLines> Style<T, B, On, R, On, V, HLines, VLines> {
    /// Sets a left intersection char.
    pub fn intersection_left(mut self, c: char) -> Self {
        self.borders.left_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, V, HLines, VLines> Style<T, B, L, On, On, V, HLines, VLines> {
    /// Sets a right intersection char.
    pub fn intersection_right(mut self, c: char) -> Self {
        self.borders.right_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, R, H, HLines, VLines> Style<On, B, L, R, H, On, HLines, VLines> {
    /// Sets a top intersection char.
    pub fn intersection_top(mut self, c: char) -> Self {
        self.borders.top_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, R, H, HLines, VLines> Style<T, On, L, R, H, On, HLines, VLines> {
    /// Sets a bottom intersection char.
    pub fn intersection_bottom(mut self, c: char) -> Self {
        self.borders.bottom_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, HLines, VLines> Style<T, B, L, R, On, On, HLines, VLines> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub fn intersection(mut self, c: char) -> Self {
        self.borders.intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, R, H, V, HLines, VLines> Style<On, B, L, R, H, V, HLines, VLines> {
    /// Removes top border.
    pub fn remove_top(
        mut self,
    ) -> Style<(), B, L, R, H, V, HLines, VerticalLineIter<VLines::IntoIter>>
    where
        VLines: IntoIterator<Item = VerticalLine> + Clone,
    {
        self.borders.top = None;
        self.borders.top_intersection = None;
        self.borders.top_left = None;
        self.borders.top_right = None;

        let iter = VerticalLineIter::new(self.verticals.into_iter(), false, true, false);
        Style::new(self.borders, self.horizontals, iter)
    }
}

impl<T, L, R, H, V, HLines, VLines> Style<T, On, L, R, H, V, HLines, VLines> {
    /// Removes bottom border.
    pub fn remove_bottom(
        mut self,
    ) -> Style<T, (), L, R, H, V, HLines, VerticalLineIter<VLines::IntoIter>>
    where
        VLines: IntoIterator<Item = VerticalLine> + Clone,
    {
        self.borders.bottom = None;
        self.borders.bottom_intersection = None;
        self.borders.bottom_left = None;
        self.borders.bottom_right = None;

        let iter = VerticalLineIter::new(self.verticals.into_iter(), false, false, true);
        Style::new(self.borders, self.horizontals, iter)
    }
}

impl<T, B, R, H, V, HLines, VLines> Style<T, B, On, R, H, V, HLines, VLines> {
    /// Removes left border.
    pub fn remove_left(
        mut self,
    ) -> Style<T, B, (), R, H, V, HorizontalLineIter<HLines::IntoIter>, VLines>
    where
        HLines: IntoIterator<Item = HorizontalLine> + Clone,
    {
        self.borders.left = None;
        self.borders.left_intersection = None;
        self.borders.top_left = None;
        self.borders.bottom_left = None;

        let iter = HorizontalLineIter::new(self.horizontals.into_iter(), false, true, false);
        Style::new(self.borders, iter, self.verticals)
    }
}

impl<T, B, L, H, V, HLines, VLines> Style<T, B, L, On, H, V, HLines, VLines> {
    /// Removes right border.
    pub fn remove_right(
        mut self,
    ) -> Style<T, B, L, (), H, V, HorizontalLineIter<HLines::IntoIter>, VLines>
    where
        HLines: IntoIterator<Item = HorizontalLine> + Clone,
    {
        self.borders.right = None;
        self.borders.right_intersection = None;
        self.borders.top_right = None;
        self.borders.bottom_right = None;

        let iter = HorizontalLineIter::new(self.horizontals.into_iter(), false, false, true);
        Style::new(self.borders, iter, self.verticals)
    }
}

impl<T, B, L, R, V, HLines, VLines> Style<T, B, L, R, On, V, HLines, VLines> {
    /// Removes horizontal split lines.
    ///
    /// Not including custom split lines.
    pub fn remove_horizontal(
        mut self,
    ) -> Style<T, B, L, R, (), V, HLines, VerticalLineIter<VLines::IntoIter>>
    where
        VLines: IntoIterator<Item = VerticalLine> + Clone,
    {
        self.borders.horizontal = None;
        self.borders.left_intersection = None;
        self.borders.right_intersection = None;
        self.borders.intersection = None;

        let iter = VerticalLineIter::new(self.verticals.into_iter(), true, false, false);
        Style::new(self.borders, self.horizontals, iter)
    }
}

impl<T, B, L, R, H, HLines, VLines> Style<T, B, L, R, H, On, HLines, VLines> {
    /// Removes vertical split lines.
    pub fn remove_vertical(
        mut self,
    ) -> Style<T, B, L, R, H, (), HorizontalLineIter<HLines::IntoIter>, VLines>
    where
        HLines: IntoIterator<Item = HorizontalLine> + Clone,
    {
        self.borders.vertical = None;
        self.borders.top_intersection = None;
        self.borders.bottom_intersection = None;
        self.borders.intersection = None;

        let iter = HorizontalLineIter::new(self.horizontals.into_iter(), true, false, false);
        Style::new(self.borders, iter, self.verticals)
    }
}

impl<T, B, L, R, H, V, HLines, VLines> Style<T, B, L, R, H, V, HLines, VLines> {
    const fn new(borders: Borders<char>, horizontals: HLines, verticals: VLines) -> Self {
        Self {
            borders,
            horizontals,
            verticals,
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _horizontal: PhantomData,
            _vertical: PhantomData,
        }
    }

    /// Return borders of a table.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Return custom horizontals which were set.
    pub const fn get_horizontals(&self) -> &HLines {
        &self.horizontals
    }

    /// Return custom verticals which were set.
    pub const fn get_verticals(&self) -> &VLines {
        &self.verticals
    }
}

#[cfg(feature = "std")]
impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, ColoredConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine> + Clone,
    VLines: IntoIterator<Item = VerticalLine> + Clone,
{
    fn change(self, records: &mut I, cfg: &mut ColoredConfig, dimension: &mut D) {
        cfg.clear_theme();

        cfg.set_borders(self.borders);

        for hl in self.horizontals {
            hl.change(records, cfg, dimension);
        }

        for vl in self.verticals {
            vl.change(records, cfg, dimension);
        }
    }
}

impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, CompactConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine> + Clone,
{
    fn change(self, records: &mut I, cfg: &mut CompactConfig, dimension: &mut D) {
        *cfg = cfg.set_borders(self.borders);

        let first_line = self.horizontals.into_iter().next();
        if let Some(line) = first_line {
            line.change(records, cfg, dimension);
        }
    }
}

impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, CompactMultilineConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine> + Clone,
{
    fn change(self, records: &mut I, cfg: &mut CompactMultilineConfig, dimension: &mut D) {
        self.change(records, cfg.as_mut(), dimension)
    }
}

/// An iterator which limits [`Line`] influence on iterations over lines for in [`Style`].
#[derive(Debug, Clone)]
pub struct HorizontalLineIter<I> {
    iter: I,
    intersection: bool,
    left: bool,
    right: bool,
}

impl<I> HorizontalLineIter<I> {
    fn new(iter: I, intersection: bool, left: bool, right: bool) -> Self {
        Self {
            iter,
            intersection,
            left,
            right,
        }
    }
}

impl<I> Iterator for HorizontalLineIter<I>
where
    I: Iterator<Item = HorizontalLine>,
{
    type Item = HorizontalLine;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hl = self.iter.next()?;

        if self.intersection {
            hl.line.intersection = None;
        }

        if self.left {
            hl.line.connector1 = None;
        }

        if self.right {
            hl.line.connector2 = None;
        }

        Some(hl)
    }
}

/// An iterator which limits [`Line`] influence on iterations over lines for in [`Style`].
#[derive(Debug, Clone)]
pub struct VerticalLineIter<I> {
    iter: I,
    intersection: bool,
    top: bool,
    bottom: bool,
}

impl<I> VerticalLineIter<I> {
    fn new(iter: I, intersection: bool, top: bool, bottom: bool) -> Self {
        Self {
            iter,
            intersection,
            top,
            bottom,
        }
    }
}

impl<I> Iterator for VerticalLineIter<I>
where
    I: Iterator<Item = VerticalLine>,
{
    type Item = VerticalLine;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hl = self.iter.next()?;

        if self.intersection {
            hl.line.intersection = None;
        }

        if self.top {
            hl.line.connector1 = None;
        }

        if self.bottom {
            hl.line.connector2 = None;
        }

        Some(hl)
    }
}

const fn create_borders(
    top: Line,
    bottom: Line,
    horizontal: Line,
    left: Option<char>,
    right: Option<char>,
    vertical: Option<char>,
) -> Borders<char> {
    Borders {
        top: top.main,
        bottom: bottom.main,
        top_left: top.connector1,
        top_right: top.connector2,
        bottom_left: bottom.connector1,
        bottom_right: bottom.connector2,
        top_intersection: top.intersection,
        bottom_intersection: bottom.intersection,
        left_intersection: horizontal.connector1,
        right_intersection: horizontal.connector2,
        horizontal: horizontal.main,
        intersection: horizontal.intersection,
        left,
        right,
        vertical,
    }
}
