//! This module contains a compile time style builder [`Style`].

use core::marker::PhantomData;

use crate::{
    grid::config::{Borders, CompactConfig, CompactMultilineConfig},
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::{
    grid::config::{Border as GridBorder, ColoredConfig},
    settings::Border,
};

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
/// let data = vec!["Hello", "2021"];
/// let style = Style::ascii().bottom('*').intersection(' ');
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style<T, B, L, R, H, V, const HSIZE: usize = 0, const VSIZE: usize = 0> {
    borders: Borders<char>,
    horizontals: HArray<L, R, V, HSIZE>,
    verticals: VArray<T, B, H, VSIZE>,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
    _horizontal: PhantomData<H>,
    _vertical: PhantomData<V>,
}

type HArray<L, R, I, const N: usize> = [(usize, HorizontalLine<L, R, I>); N];
type VArray<T, B, I, const N: usize> = [(usize, VerticalLine<T, B, I>); N];

/// A marker struct which is used in [`Style`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default, Hash)]
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
    pub const fn psql() -> Style<(), (), (), (), (), On, 1, 0> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                None,
                None,
                Some('|'),
            ),
            [(1, HorizontalLine::new('-').intersection('+'))],
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
    pub const fn markdown() -> Style<(), (), On, On, (), On, 1, 0> {
        Style::new(
            create_borders(
                Line::empty(),
                Line::empty(),
                Line::empty(),
                Some('|'),
                Some('|'),
                Some('|'),
            ),
            [(1, HorizontalLine::full('-', '|', '|', '|'))],
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
    pub const fn sharp() -> Style<On, On, On, On, (), On, 1, 0> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '┌', '┐'),
                Line::full('─', '┴', '└', '┘'),
                Line::empty(),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [(1, HorizontalLine::full('─', '├', '┤', '┼'))],
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
    pub const fn rounded() -> Style<On, On, On, On, (), On, 1, 0> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '╭', '╮'),
                Line::full('─', '┴', '╰', '╯'),
                Line::empty(),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [(1, HorizontalLine::full('─', '├', '┤', '┼'))],
            [],
        )
    }

    /// This style looks like a [`Style::rounded`] but with horizontals lines.
    ///
    /// Beware: It uses UTF-8 characters.
    ///
    /// ```text
    ///     ╭────┬──────────────┬───────────────────────────╮
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     ╰────┴──────────────┴───────────────────────────╯
    /// ```
    pub const fn modern_rounded() -> Style<On, On, On, On, On, On, 0, 0> {
        Style::new(
            create_borders(
                Line::full('─', '┬', '╭', '╮'),
                Line::full('─', '┴', '╰', '╯'),
                Line::full('─', '┼', '├', '┤'),
                Some('│'),
                Some('│'),
                Some('│'),
            ),
            [],
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
    pub const fn re_structured_text() -> Style<On, On, (), (), (), On, 1, 0> {
        Style::new(
            create_borders(
                Line::new(Some('='), Some(' '), None, None),
                Line::new(Some('='), Some(' '), None, None),
                Line::empty(),
                None,
                None,
                Some(' '),
            ),
            [(1, HorizontalLine::new('=').intersection(' '))],
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

impl<T, B, L, R, H, V, const HSIZE: usize, const VSIZE: usize>
    Style<T, B, L, R, H, V, HSIZE, VSIZE>
{
    const fn new(
        borders: Borders<char>,
        horizontals: HArray<L, R, V, HSIZE>,
        verticals: VArray<T, B, H, VSIZE>,
    ) -> Self {
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
}

impl<T, B, L, R, H, V, const HN: usize, const VN: usize> Style<T, B, L, R, H, V, HN, VN> {
    /// Set border horizontal lines.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{settings::style::{Style, HorizontalLine, Line}, Table};
    ///
    /// let data = (0..3).map(|i| ("Hello", i));
    ///
    /// let style = Style::rounded().horizontals([
    ///     (1, HorizontalLine::filled('#')),
    ///     (2, HorizontalLine::filled('&')),
    ///     (3, HorizontalLine::filled('@')),
    /// ]);
    ///
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "╭───────┬─────╮\n",
    ///         "│ &str  │ i32 │\n",
    ///         "###############\n",
    ///         "│ Hello │ 0   │\n",
    ///         "&&&&&&&&&&&&&&&\n",
    ///         "│ Hello │ 1   │\n",
    ///         "@@@@@@@@@@@@@@@\n",
    ///         "│ Hello │ 2   │\n",
    ///         "╰───────┴─────╯",
    ///     )
    /// )
    /// ```
    pub const fn horizontals<const SIZE: usize>(
        self,
        list: HArray<L, R, V, SIZE>,
    ) -> Style<T, B, L, R, H, V, SIZE, VN> {
        Style::new(self.borders, list, self.verticals)
    }

    /// Set border vertical lines.
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "derive", doc = "```")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// use tabled::{settings::style::{Style, VerticalLine, Line}, Table};
    ///
    /// let data = (0..3).map(|i| ("Hello", "World", i));
    ///
    /// let style = Style::rounded().verticals([
    ///     (1, VerticalLine::new('#').top(':').bottom('.')),
    ///     (2, VerticalLine::new('&').top(':').bottom('.')),
    /// ]);
    ///
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "╭───────:───────:─────╮\n",
    ///         "│ &str  # &str  & i32 │\n",
    ///         "├───────┼───────┼─────┤\n",
    ///         "│ Hello # World & 0   │\n",
    ///         "│ Hello # World & 1   │\n",
    ///         "│ Hello # World & 2   │\n",
    ///         "╰───────.───────.─────╯",
    ///     )
    /// )
    /// ```
    pub const fn verticals<const SIZE: usize>(
        self,
        list: VArray<T, B, H, SIZE>,
    ) -> Style<T, B, L, R, H, V, HN, SIZE> {
        Style::new(self.borders, self.horizontals, list)
    }

    /// Removes all horizontal lines set by [`Style::horizontals`]
    pub const fn remove_horizontals(self) -> Style<T, B, L, R, H, V, 0, VN> {
        Style::new(self.borders, [], self.verticals)
    }

    /// Removes all verticals lines set by [`Style::verticals`]
    pub const fn remove_verticals(self) -> Style<T, B, L, R, H, V, HN, 0> {
        Style::new(self.borders, self.horizontals, [])
    }

    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn top(mut self, c: char) -> Style<On, B, L, R, H, V, HN, VN>
    where
        T: Copy,
        B: Copy,
        H: Copy,
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

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_set(lines, Line::new(None, None, Some(c), None));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn bottom(mut self, c: char) -> Style<T, On, L, R, H, V, HN, VN>
    where
        T: Copy,
        B: Copy,
        H: Copy,
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

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_set(lines, Line::new(None, None, None, Some(c)));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn left(mut self, c: char) -> Style<T, B, On, R, H, V, HN, VN>
    where
        L: Copy,
        R: Copy,
        V: Copy,
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

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_set(lines, Line::new(None, None, Some(c), None));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn right(mut self, c: char) -> Style<T, B, L, On, H, V, HN, VN>
    where
        L: Copy,
        R: Copy,
        V: Copy,
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

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_set(lines, Line::new(None, None, None, Some(c)));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
    }

    /// Sets a horizontal split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn horizontal(mut self, c: char) -> Style<T, B, L, R, On, V, HN, VN>
    where
        T: Copy,
        B: Copy,
        H: Copy,
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

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_set(lines, Line::new(None, Some(c), None, None));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn vertical(mut self, c: char) -> Style<T, B, L, R, H, On, HN, VN>
    where
        L: Copy,
        R: Copy,
        V: Copy,
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

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_set(lines, Line::new(None, Some(c), None, None));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
    }

    /// Set a vertical line.
    /// An equvalent of calling vertical+top_intersection+bottom_intersection+intersion.
    ///
    /// Notice, that it will clear everything that is outdated, meaning
    /// If your style has a top border line and but the given vertical line has not got it then it will be removed.
    pub const fn vertical_line<Top, Bottom, Intersection>(
        mut self,
        line: VerticalLine<Top, Bottom, Intersection>,
    ) -> Style<Top, Bottom, L, R, Intersection, On, HN, VN>
    where
        L: Copy,
        R: Copy,
        Top: Copy,
        Bottom: Copy,
        Intersection: Copy,
    {
        let line = line.into_inner();

        self.borders.vertical = line.main;
        self.borders.intersection = line.intersection;
        self.borders.top_intersection = line.connector1;
        self.borders.bottom_intersection = line.connector2;

        if line.intersection.is_none() {
            self.borders.horizontal = None;
            self.borders.left_intersection = None;
            self.borders.right_intersection = None;
            self.borders.intersection = None;
        } else {
            if self.borders.has_left() && self.borders.left_intersection.is_none() {
                self.borders.left_intersection = Some(' ');
            }

            if self.borders.has_right() && self.borders.right_intersection.is_none() {
                self.borders.right_intersection = Some(' ');
            }

            if self.borders.horizontal.is_none() {
                self.borders.horizontal = Some(' ');
            }
        }

        if line.connector1.is_none() {
            self.borders.top = None;
            self.borders.top_left = None;
            self.borders.top_right = None;
            self.borders.top_intersection = None;
        } else if self.borders.has_top() && self.borders.top_intersection.is_none() {
            self.borders.top_intersection = Some(' ');
        }

        if line.connector2.is_none() {
            self.borders.bottom = None;
            self.borders.bottom_left = None;
            self.borders.bottom_right = None;
            self.borders.bottom_intersection = None;
        } else if self.borders.has_bottom() && self.borders.bottom_intersection.is_none() {
            self.borders.bottom_intersection = Some(' ');
        }

        let horizontals = harray_convert(self.horizontals);
        let horizontals = linearr_set(horizontals, Line::new(None, line.intersection, None, None));
        let horizontals = linearr_convert_to_harray(horizontals);

        let verticals = varray_convert(self.verticals);
        let verticals = linearr_set(
            verticals,
            Line::new(None, line.intersection, line.connector1, line.connector2),
        );
        let verticals = linearr_convert_to_varray(verticals);

        Style::new(self.borders, horizontals, verticals)
    }

    /// Set a horizontal line.
    /// An equvalent of calling horizontal+left_intersection+right_intersection+intersion.
    ///
    /// Notice, that it will clear everything that is outdated, meaning
    /// If your style has a left border line and but the given vertical line has not got it then it will be removed.
    pub const fn horizontal_line<Left, Right, Intersection>(
        mut self,
        line: HorizontalLine<Left, Right, Intersection>,
    ) -> Style<Left, Right, L, R, Intersection, On, HN, VN>
    where
        L: Copy,
        R: Copy,
        Left: Copy,
        Right: Copy,
        Intersection: Copy,
    {
        let line = line.into_inner();

        self.borders.horizontal = line.main;
        self.borders.intersection = line.intersection;
        self.borders.left_intersection = line.connector1;
        self.borders.right_intersection = line.connector2;

        if line.intersection.is_none() {
            self.borders.vertical = None;
            self.borders.top_intersection = None;
            self.borders.bottom_intersection = None;
            self.borders.intersection = None;
        } else {
            if self.borders.has_top() && self.borders.top_intersection.is_none() {
                self.borders.top_intersection = Some(' ');
            }

            if self.borders.has_bottom() && self.borders.bottom_intersection.is_none() {
                self.borders.bottom_intersection = Some(' ');
            }

            if self.borders.vertical.is_none() {
                self.borders.vertical = Some(' ');
            }
        }

        if line.connector1.is_none() {
            self.borders.left = None;
            self.borders.top_left = None;
            self.borders.bottom_left = None;
            self.borders.left_intersection = None;
        } else if self.borders.has_left() && self.borders.left_intersection.is_none() {
            self.borders.left_intersection = Some(' ');
        }

        if line.connector2.is_none() {
            self.borders.right = None;
            self.borders.top_right = None;
            self.borders.bottom_right = None;
            self.borders.right_intersection = None;
        } else if self.borders.has_right() && self.borders.right_intersection.is_none() {
            self.borders.right_intersection = Some(' ');
        }

        let horizontals = harray_convert(self.horizontals);
        let horizontals = linearr_set(
            horizontals,
            Line::new(None, line.intersection, line.connector1, line.connector2),
        );
        let horizontals = linearr_convert_to_harray(horizontals);

        let verticals = varray_convert(self.verticals);
        let verticals = linearr_set(verticals, Line::new(None, line.intersection, None, None));
        let verticals = linearr_convert_to_varray(verticals);

        Style::new(self.borders, horizontals, verticals)
    }

    /// Set a frame for a style.
    ///
    /// It makes assumptions that a full frame will be set, but it may not be.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::Style};
    ///
    /// let data = [["10:52:19", "Hello"], ["10:52:20", "World"]];
    /// let table = Table::new(data)
    ///     .with(Style::ascii().frame(Style::modern().get_frame()))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "┌──────────+───────┐\n",
    ///         "│ 0        | 1     │\n",
    ///         "+----------+-------+\n",
    ///         "│ 10:52:19 | Hello │\n",
    ///         "+----------+-------+\n",
    ///         "│ 10:52:20 | World │\n",
    ///         "└──────────+───────┘",
    ///     )
    /// );
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub const fn frame<Top, Bottom, Left, Right>(
        mut self,
        border: Border<Top, Bottom, Left, Right>,
    ) -> Style<Top, Bottom, Left, Right, H, V, HN, VN>
    where
        T: Copy,
        B: Copy,
        L: Copy,
        R: Copy,
        H: Copy,
        V: Copy,
        Left: Copy,
        Right: Copy,
        Top: Copy,
        Bottom: Copy,
    {
        let border = border.into_inner();
        let border = correct_border(border);

        let horizontals = harray_convert(self.horizontals);
        let horizontals = linearr_set(
            horizontals,
            Line::new(None, None, border.left, border.right),
        );
        let horizontals = linearr_convert_to_harray(horizontals);

        let verticals = varray_convert(self.verticals);
        let verticals = linearr_set(verticals, Line::new(None, None, border.top, border.bottom));
        let verticals = linearr_convert_to_varray(verticals);

        self.borders.top = border.top;
        self.borders.bottom = border.bottom;
        self.borders.left = border.left;
        self.borders.top_left = border.left_top_corner;
        self.borders.bottom_left = border.left_bottom_corner;
        self.borders.right = border.right;
        self.borders.top_right = border.right_top_corner;
        self.borders.bottom_right = border.right_bottom_corner;

        Style::new(self.borders, horizontals, verticals)
    }
}

impl<T, B, L, R, H, V, const HSIZE: usize, const VSIZE: usize>
    Style<T, B, L, R, H, V, HSIZE, VSIZE>
{
    /// Frame function returns a frame as a border.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::Style};
    ///
    /// let data = [["10:52:19", "Hello"], ["10:52:20", "World"]];
    /// let table = Table::new(data)
    ///     .with(Style::re_structured_text().frame(Style::modern().get_frame()))
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "┌────────── ───────┐\n",
    ///         "│ 0          1     │\n",
    ///         "│========== =======│\n",
    ///         "│ 10:52:19   Hello │\n",
    ///         "│ 10:52:20   World │\n",
    ///         "└────────── ───────┘",
    ///     )
    /// );
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub const fn get_frame(&self) -> Border<T, B, L, R> {
        crate::settings::Border::from_border(crate::grid::config::Border {
            top: self.borders.top,
            bottom: self.borders.bottom,
            left: self.borders.left,
            left_top_corner: self.borders.top_left,
            left_bottom_corner: self.borders.bottom_left,
            right: self.borders.right,
            right_top_corner: self.borders.top_right,
            right_bottom_corner: self.borders.bottom_right,
        })
    }

    /// Return borders of a table.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Return custom horizontals which were set.
    pub const fn get_horizontals(&self) -> HArray<L, R, V, HSIZE>
    where
        L: Copy,
        R: Copy,
        V: Copy,
    {
        self.horizontals
    }

    /// Return custom verticals which were set.
    pub const fn get_verticals(&self) -> VArray<T, B, H, VSIZE>
    where
        T: Copy,
        B: Copy,
        H: Copy,
    {
        self.verticals
    }
}

impl<B, R, H, V, const HN: usize, const VN: usize> Style<On, B, On, R, H, V, HN, VN> {
    /// Sets a top left corner.
    pub const fn corner_top_left(mut self, c: char) -> Self {
        self.borders.top_left = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, H, V, const HN: usize, const VN: usize> Style<On, B, L, On, H, V, HN, VN> {
    /// Sets a top right corner.
    pub const fn corner_top_right(mut self, c: char) -> Self {
        self.borders.top_right = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, H, V, const HN: usize, const VN: usize> Style<T, On, L, On, H, V, HN, VN> {
    /// Sets a bottom right corner.
    pub const fn corner_bottom_right(mut self, c: char) -> Self {
        self.borders.bottom_right = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, R, H, V, const HN: usize, const VN: usize> Style<T, On, On, R, H, V, HN, VN> {
    /// Sets a bottom left corner.
    pub const fn corner_bottom_left(mut self, c: char) -> Self {
        self.borders.bottom_left = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, R, V, const HN: usize, const VN: usize> Style<T, B, On, R, On, V, HN, VN> {
    /// Sets a left intersection char.
    pub const fn intersection_left(mut self, c: char) -> Self {
        self.borders.left_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, V, const HN: usize, const VN: usize> Style<T, B, L, On, On, V, HN, VN> {
    /// Sets a right intersection char.
    pub const fn intersection_right(mut self, c: char) -> Self {
        self.borders.right_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, R, H, const HN: usize, const VN: usize> Style<On, B, L, R, H, On, HN, VN> {
    /// Sets a top intersection char.
    pub const fn intersection_top(mut self, c: char) -> Self {
        self.borders.top_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, R, H, const HN: usize, const VN: usize> Style<T, On, L, R, H, On, HN, VN> {
    /// Sets a bottom intersection char.
    pub const fn intersection_bottom(mut self, c: char) -> Self {
        self.borders.bottom_intersection = Some(c);

        Style::new(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, const HN: usize, const VN: usize> Style<T, B, L, R, On, On, HN, VN> {
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub const fn intersection(mut self, c: char) -> Self
    where
        T: Copy,
        B: Copy,
        R: Copy,
        L: Copy,
    {
        self.borders.intersection = Some(c);

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_set(lines, Line::new(None, Some(c), None, None));
            linearr_convert_to_harray(lines)
        };

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_set(lines, Line::new(None, Some(c), None, None));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, horizontals, verticals)
    }
}

impl<B, L, R, H, V, const HN: usize, const VN: usize> Style<On, B, L, R, H, V, HN, VN> {
    /// Removes top border.
    pub const fn remove_top(mut self) -> Style<(), B, L, R, H, V, HN, VN>
    where
        B: Copy,
        H: Copy,
    {
        self.borders.top = None;
        self.borders.top_intersection = None;
        self.borders.top_left = None;
        self.borders.top_right = None;

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_unset(lines, Line::new(None, None, Some(' '), None));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
    }
}

impl<T, L, R, H, V, const HN: usize, const VN: usize> Style<T, On, L, R, H, V, HN, VN> {
    /// Removes bottom border.
    pub const fn remove_bottom(mut self) -> Style<T, (), L, R, H, V, HN, VN>
    where
        T: Copy,
        H: Copy,
    {
        self.borders.bottom = None;
        self.borders.bottom_intersection = None;
        self.borders.bottom_left = None;
        self.borders.bottom_right = None;

        let verticals = {
            let lines = varray_convert(self.verticals);
            let lines = linearr_unset(lines, Line::new(None, None, None, Some(' ')));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
    }
}

impl<T, B, R, H, V, const HN: usize, const VN: usize> Style<T, B, On, R, H, V, HN, VN> {
    /// Removes left border.
    pub const fn remove_left(mut self) -> Style<T, B, (), R, H, V, HN, VN>
    where
        R: Copy,
        V: Copy,
    {
        self.borders.left = None;
        self.borders.left_intersection = None;
        self.borders.top_left = None;
        self.borders.bottom_left = None;

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_unset(lines, Line::new(None, None, Some(' '), None));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
    }
}

impl<T, B, L, H, V, const HN: usize, const VN: usize> Style<T, B, L, On, H, V, HN, VN> {
    /// Removes right border.
    pub const fn remove_right(mut self) -> Style<T, B, L, (), H, V, HN, VN>
    where
        L: Copy,
        V: Copy,
    {
        self.borders.right = None;
        self.borders.right_intersection = None;
        self.borders.top_right = None;
        self.borders.bottom_right = None;

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            let lines = linearr_unset(lines, Line::new(None, None, None, Some(' ')));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
    }
}

impl<T, B, L, R, V, const HN: usize, const VN: usize> Style<T, B, L, R, On, V, HN, VN> {
    /// Removes horizontal split lines.
    ///
    /// Not including custom split lines.
    pub const fn remove_horizontal(mut self) -> Style<T, B, L, R, (), V, HN, VN>
    where
        T: Copy,
        B: Copy,
        V: Copy,
    {
        self.borders.horizontal = None;
        self.borders.left_intersection = None;
        self.borders.right_intersection = None;
        self.borders.intersection = None;

        let verticals = {
            let lines = varray_convert(self.verticals);
            // let lines = linearr_unset(lines, Line::new(None, Some(' '), None, None));
            linearr_convert_to_varray(lines)
        };

        Style::new(self.borders, self.horizontals, verticals)
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
    /// let data = (0..3).map(|i| ("Hello", "World", i));
    /// let table = Table::new(data)
    ///    .with(Style::ascii().remove_horizontal().horizontals([(1, Style::modern().get_horizontal_line())]))
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
    pub const fn get_horizontal_line(&self) -> HorizontalLine<L, R, V> {
        HorizontalLine::update(Line::new(
            self.borders.horizontal,
            self.borders.intersection,
            self.borders.left_intersection,
            self.borders.right_intersection,
        ))
    }
}

impl<T, B, L, R, H, const HN: usize, const VN: usize> Style<T, B, L, R, H, On, HN, VN> {
    /// Removes vertical split lines.
    pub const fn remove_vertical(mut self) -> Style<T, B, L, R, H, (), HN, VN>
    where
        R: Copy,
        L: Copy,
    {
        self.borders.vertical = None;
        self.borders.top_intersection = None;
        self.borders.bottom_intersection = None;
        self.borders.intersection = None;

        let horizontals = {
            let lines = harray_convert(self.horizontals);
            // let lines = linearr_unset(lines, Line::new(None, Some(' '), None, None));
            linearr_convert_to_harray(lines)
        };

        Style::new(self.borders, horizontals, self.verticals)
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
    ///    .with(Style::ascii().remove_horizontal().verticals([(1, Style::modern().get_vertical_line().remove_intersection())]))
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
    pub const fn get_vertical_line(&self) -> VerticalLine<T, B, H> {
        VerticalLine::update(Line::new(
            self.borders.vertical,
            self.borders.intersection,
            self.borders.top_intersection,
            self.borders.bottom_intersection,
        ))
    }
}

#[cfg(feature = "std")]
impl<T, B, L, R, H, V, Data, Dims, const HSIZE: usize, const VSIZE: usize>
    TableOption<Data, Dims, ColoredConfig> for Style<T, B, L, R, H, V, HSIZE, VSIZE>
{
    fn change(self, _: &mut Data, cfg: &mut ColoredConfig, _: &mut Dims) {
        cfg.clear_theme();
        cfg.set_borders(self.borders);

        for (i, line) in self.horizontals {
            let line = line.into_inner().into();
            cfg.insert_horizontal_line(i, line);
        }

        for (i, line) in self.verticals {
            let line = line.into_inner().into();
            cfg.insert_vertical_line(i, line);
        }
    }
}

impl<T, B, L, R, H, V, Data, Dims, const HSIZE: usize, const VSIZE: usize>
    TableOption<Data, Dims, CompactConfig> for Style<T, B, L, R, H, V, HSIZE, VSIZE>
{
    fn change(self, _: &mut Data, cfg: &mut CompactConfig, _: &mut Dims) {
        *cfg = cfg.set_borders(self.borders);

        if let Some((i, line)) = self.horizontals.get(0) {
            if *i == 1 {
                let line = line.into_inner().into();
                *cfg = cfg.set_first_horizontal_line(line);
            }
        }
    }
}

impl<T, B, L, R, H, V, Data, Dims, const HSIZE: usize, const VSIZE: usize>
    TableOption<Data, Dims, CompactMultilineConfig> for Style<T, B, L, R, H, V, HSIZE, VSIZE>
{
    fn change(self, records: &mut Data, cfg: &mut CompactMultilineConfig, dims: &mut Dims) {
        self.change(records, cfg.as_mut(), dims)
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

#[cfg(feature = "std")]
const fn correct_border(mut border: GridBorder<char>) -> GridBorder<char> {
    if border.has_top() && border.top.is_none() {
        border.top = Some(' ');
    }

    if border.has_bottom() && border.bottom.is_none() {
        border.bottom = Some(' ');
    }

    if border.has_left() && border.left.is_none() {
        border.left = Some(' ');
    }

    if border.has_right() && border.right.is_none() {
        border.right = Some(' ');
    }

    if border.has_top() && border.has_left() && border.left_top_corner.is_none() {
        border.left_top_corner = Some(' ');
    }

    if border.has_top() && border.has_right() && border.right_top_corner.is_none() {
        border.right_top_corner = Some(' ');
    }

    if border.has_bottom() && border.has_left() && border.left_top_corner.is_none() {
        border.left_bottom_corner = Some(' ');
    }

    if border.has_bottom() && border.has_right() && border.right_bottom_corner.is_none() {
        border.right_bottom_corner = Some(' ');
    }

    border
}

const fn varray_convert<T, B, I, const N: usize>(lines: VArray<T, B, I, N>) -> [(usize, Line); N] {
    let mut buf = [(0, Line::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, line) = &lines[i];
        let index = *index;
        let line = line.into_inner();

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

const fn harray_convert<L, R, I, const N: usize>(lines: HArray<L, R, I, N>) -> [(usize, Line); N] {
    let mut buf = [(0, Line::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, line) = &lines[i];
        let index = *index;
        let line = line.into_inner();

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

const fn linearr_convert_to_varray<T, B, I, const N: usize>(
    lines: [(usize, Line); N],
) -> VArray<T, B, I, N>
where
    T: Copy,
    B: Copy,
    I: Copy,
{
    let mut buf = [(0, VerticalLine::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, line) = lines[i];
        let line = VerticalLine::update(line);

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

const fn linearr_convert_to_harray<L, R, I, const N: usize>(
    lines: [(usize, Line); N],
) -> HArray<L, R, I, N>
where
    L: Copy,
    R: Copy,
    I: Copy,
{
    let mut buf = [(0, HorizontalLine::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, line) = lines[i];
        let line = HorizontalLine::update(line);

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

const fn linearr_set<const N: usize>(lines: [(usize, Line); N], set: Line) -> [(usize, Line); N] {
    let mut buf = [(0, Line::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, mut line) = lines[i];

        if set.connector1.is_some() {
            line.connector1 = set.connector1;
        }

        if set.connector2.is_some() {
            line.connector2 = set.connector2;
        }

        if set.intersection.is_some() {
            line.intersection = set.intersection;
        }

        if set.main.is_some() {
            line.main = set.main;
        }

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

const fn linearr_unset<const N: usize>(lines: [(usize, Line); N], set: Line) -> [(usize, Line); N] {
    let mut buf = [(0, Line::empty()); N];
    let mut i = 0;
    while i < N {
        let (index, mut line) = lines[i];

        if set.connector1.is_some() {
            line.connector1 = None;
        }

        if set.connector2.is_some() {
            line.connector2 = None;
        }

        if set.intersection.is_some() {
            line.intersection = None;
        }

        if set.main.is_some() {
            line.main = None;
        }

        buf[i].0 = index;
        buf[i].1 = line;

        i += 1;
    }

    buf
}

// todo: MACROS to reduce duplication of the same code for Varray and Harray
