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
pub struct Style<T, B, L, R, H, V, HL = HLineArray<0>, VL = VLineArray<0>> {
    borders: Borders<char>,
    horizontals: HorizontalLineIter<HL>,
    verticals: VerticalLineIter<VL>,
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

impl<T, B, L, R, H, V, const HN: usize, const VN: usize>
    Style<T, B, L, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
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
    pub const fn horizontals<Lines>(
        self,
        lines: Lines,
    ) -> Style<T, B, L, R, H, V, Lines, VLineArray<VN>>
    where
        Lines: IntoIterator<Item = HorizontalLine>,
    {
        Style::update(self.borders, HorizontalLineIter::new(lines), self.verticals)
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
    pub const fn verticals<Lines>(
        self,
        lines: Lines,
    ) -> Style<T, B, L, R, H, V, HLineArray<HN>, Lines>
    where
        Lines: IntoIterator<Item = VerticalLine>,
    {
        Style::update(self.borders, self.horizontals, VerticalLineIter::new(lines))
    }

    /// Removes all horizontal lines set by [`Style::horizontals`]
    pub const fn remove_horizontals(
        self,
    ) -> Style<T, B, L, R, H, V, HLineArray<0>, VLineArray<VN>> {
        Style::update(self.borders, HorizontalLineIter::new([]), self.verticals)
    }

    /// Removes all verticals lines set by [`Style::verticals`]
    pub const fn remove_verticals(self) -> Style<T, B, L, R, H, V, HLineArray<HN>, VLineArray<0>> {
        Style::update(self.borders, self.horizontals, VerticalLineIter::new([]))
    }

    /// Sets a top border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn top(
        mut self,
        c: char,
    ) -> Style<On, B, L, R, H, V, HLineArray<HN>, VLineArray<VN>> {
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

        self.verticals.top = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a bottom border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn bottom(
        mut self,
        c: char,
    ) -> Style<T, On, L, R, H, V, HLineArray<HN>, VLineArray<VN>> {
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

        self.verticals.bottom = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a left border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn left(
        mut self,
        c: char,
    ) -> Style<T, B, On, R, H, V, HLineArray<HN>, VLineArray<VN>> {
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

        self.horizontals.left = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a right border.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn right(
        mut self,
        c: char,
    ) -> Style<T, B, L, On, H, V, HLineArray<HN>, VLineArray<VN>> {
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

        self.horizontals.right = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a horizontal split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn horizontal(
        mut self,
        c: char,
    ) -> Style<T, B, L, R, On, V, HLineArray<HN>, VLineArray<VN>> {
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

        self.verticals.inter = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }

    /// Sets a vertical split line.
    ///
    /// Any corners and intersections which were set will be overridden.
    pub const fn vertical(
        mut self,
        c: char,
    ) -> Style<T, B, L, R, H, On, HLineArray<HN>, VLineArray<VN>> {
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

        self.horizontals.inter = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
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

    /// Return borders of a table.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.borders
    }

    /// Return custom horizontals which were set.
    pub const fn get_horizontals(&self) -> &HLines {
        &self.horizontals.iter
    }

    /// Return custom verticals which were set.
    pub const fn get_verticals(&self) -> &VLines {
        &self.verticals.iter
    }
}

impl<B, R, H, V, const HN: usize, const VN: usize>
    Style<On, B, On, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a top left corner.
    pub const fn corner_top_left(mut self, c: char) -> Self {
        self.borders.top_left = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, H, V, const HN: usize, const VN: usize>
    Style<On, B, L, On, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a top right corner.
    pub const fn corner_top_right(mut self, c: char) -> Self {
        self.borders.top_right = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, H, V, const HN: usize, const VN: usize>
    Style<T, On, L, On, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a bottom right corner.
    pub const fn corner_bottom_right(mut self, c: char) -> Self {
        self.borders.bottom_right = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, R, H, V, const HN: usize, const VN: usize>
    Style<T, On, On, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a bottom left corner.
    pub const fn corner_bottom_left(mut self, c: char) -> Self {
        self.borders.bottom_left = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, R, V, const HN: usize, const VN: usize>
    Style<T, B, On, R, On, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a left intersection char.
    pub const fn intersection_left(mut self, c: char) -> Self {
        self.borders.left_intersection = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, V, const HN: usize, const VN: usize>
    Style<T, B, L, On, On, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a right intersection char.
    pub const fn intersection_right(mut self, c: char) -> Self {
        self.borders.right_intersection = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, R, H, const HN: usize, const VN: usize>
    Style<On, B, L, R, H, On, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a top intersection char.
    pub const fn intersection_top(mut self, c: char) -> Self {
        self.borders.top_intersection = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, R, H, const HN: usize, const VN: usize>
    Style<T, On, L, R, H, On, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets a bottom intersection char.
    pub const fn intersection_bottom(mut self, c: char) -> Self {
        self.borders.bottom_intersection = Some(c);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, const HN: usize, const VN: usize>
    Style<T, B, L, R, On, On, HLineArray<HN>, VLineArray<VN>>
{
    /// Sets an inner intersection char.
    /// A char between horizontal and vertical split lines.
    pub const fn intersection(mut self, c: char) -> Self {
        self.borders.intersection = Some(c);

        self.horizontals.inter = Some(CharFlag::Set(c));
        self.verticals.inter = Some(CharFlag::Set(c));

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<B, L, R, H, V, const HN: usize, const VN: usize>
    Style<On, B, L, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes top border.
    pub const fn remove_top(mut self) -> Style<(), B, L, R, H, V, HLineArray<HN>, VLineArray<VN>> {
        self.borders.top = None;
        self.borders.top_intersection = None;
        self.borders.top_left = None;
        self.borders.top_right = None;

        self.verticals.top = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, L, R, H, V, const HN: usize, const VN: usize>
    Style<T, On, L, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes bottom border.
    pub const fn remove_bottom(
        mut self,
    ) -> Style<T, (), L, R, H, V, HLineArray<HN>, VLineArray<VN>> {
        self.borders.bottom = None;
        self.borders.bottom_intersection = None;
        self.borders.bottom_left = None;
        self.borders.bottom_right = None;

        self.verticals.bottom = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, R, H, V, const HN: usize, const VN: usize>
    Style<T, B, On, R, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes left border.
    pub const fn remove_left(mut self) -> Style<T, B, (), R, H, V, HLineArray<HN>, VLineArray<VN>> {
        self.borders.left = None;
        self.borders.left_intersection = None;
        self.borders.top_left = None;
        self.borders.bottom_left = None;

        self.horizontals.left = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, H, V, const HN: usize, const VN: usize>
    Style<T, B, L, On, H, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes right border.
    pub const fn remove_right(
        mut self,
    ) -> Style<T, B, L, (), H, V, HLineArray<HN>, VLineArray<VN>> {
        self.borders.right = None;
        self.borders.right_intersection = None;
        self.borders.top_right = None;
        self.borders.bottom_right = None;

        self.horizontals.right = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, V, const HN: usize, const VN: usize>
    Style<T, B, L, R, On, V, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes horizontal split lines.
    ///
    /// Not including custom split lines.
    pub const fn remove_horizontal(
        mut self,
    ) -> Style<T, B, L, R, (), V, HLineArray<HN>, VLineArray<VN>> {
        self.borders.horizontal = None;
        self.borders.left_intersection = None;
        self.borders.right_intersection = None;
        self.borders.intersection = None;

        self.verticals.inter = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, H, const HN: usize, const VN: usize>
    Style<T, B, L, R, H, On, HLineArray<HN>, VLineArray<VN>>
{
    /// Removes vertical split lines.
    pub const fn remove_vertical(
        mut self,
    ) -> Style<T, B, L, R, H, (), HLineArray<HN>, VLineArray<VN>> {
        self.borders.vertical = None;
        self.borders.top_intersection = None;
        self.borders.bottom_intersection = None;
        self.borders.intersection = None;

        self.horizontals.inter = Some(CharFlag::Unset);

        Style::update(self.borders, self.horizontals, self.verticals)
    }
}

impl<T, B, L, R, H, V, HLines, VLines> Style<T, B, L, R, H, V, HLines, VLines> {
    const fn new(borders: Borders<char>, horizontals: HLines, verticals: VLines) -> Self {
        Self {
            borders,
            horizontals: HorizontalLineIter::new(horizontals),
            verticals: VerticalLineIter::new(verticals),
            _top: PhantomData,
            _bottom: PhantomData,
            _left: PhantomData,
            _right: PhantomData,
            _horizontal: PhantomData,
            _vertical: PhantomData,
        }
    }

    const fn update(
        borders: Borders<char>,
        horizontals: HorizontalLineIter<HLines>,
        verticals: VerticalLineIter<VLines>,
    ) -> Style<T, B, L, R, H, V, HLines, VLines> {
        Style {
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

#[cfg(feature = "std")]
impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, ColoredConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine>,
    VLines: IntoIterator<Item = VerticalLine>,
{
    fn change(self, records: &mut I, cfg: &mut ColoredConfig, dimension: &mut D) {
        cfg.clear_theme();
        cfg.set_borders(self.borders);

        for hl in self.horizontals.iter() {
            hl.change(records, cfg, dimension);
        }

        for vl in self.verticals.iter() {
            vl.change(records, cfg, dimension);
        }
    }
}

impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, CompactConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine>,
{
    fn change(self, records: &mut I, cfg: &mut CompactConfig, dimension: &mut D) {
        *cfg = cfg.set_borders(self.borders);

        let first_line = self.horizontals.iter().next();
        if let Some(line) = first_line {
            line.change(records, cfg, dimension);
        }
    }
}

impl<T, B, L, R, H, V, HLines, VLines, I, D> TableOption<I, D, CompactMultilineConfig>
    for Style<T, B, L, R, H, V, HLines, VLines>
where
    HLines: IntoIterator<Item = HorizontalLine>,
{
    fn change(self, records: &mut I, cfg: &mut CompactMultilineConfig, dimension: &mut D) {
        self.change(records, cfg.as_mut(), dimension)
    }
}

/// An iterator which limits [`Line`] influence on iterations over lines for in [`Style`].
#[derive(Debug, Clone)]
pub struct HorizontalLineIter<I> {
    iter: I,
    inter: Option<CharFlag>,
    left: Option<CharFlag>,
    right: Option<CharFlag>,
}

impl<I> HorizontalLineIter<I> {
    const fn new(iter: I) -> Self {
        Self {
            iter,
            inter: None,
            left: None,
            right: None,
        }
    }

    fn iter(self) -> HorizontalLineIter<I::IntoIter>
    where
        I: IntoIterator,
    {
        HorizontalLineIter {
            iter: self.iter.into_iter(),
            inter: self.inter,
            left: self.left,
            right: self.right,
        }
    }
}

impl<I> Iterator for HorizontalLineIter<I>
where
    I: Iterator<Item = HorizontalLine>,
{
    type Item = HorizontalLine;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.iter.next()?;

        line_set_flag_char(&mut line.line.intersection, self.inter);
        line_set_flag_char(&mut line.line.connector1, self.left);
        line_set_flag_char(&mut line.line.connector2, self.right);

        Some(line)
    }
}

/// An iterator which limits [`Line`] influence on iterations over lines for in [`Style`].
#[derive(Debug, Clone)]
pub struct VerticalLineIter<I> {
    iter: I,
    inter: Option<CharFlag>,
    top: Option<CharFlag>,
    bottom: Option<CharFlag>,
}

impl<I> VerticalLineIter<I> {
    const fn new(iter: I) -> Self {
        Self {
            iter,
            inter: None,
            top: None,
            bottom: None,
        }
    }

    #[cfg(feature = "std")]
    fn iter(self) -> VerticalLineIter<I::IntoIter>
    where
        I: IntoIterator,
    {
        VerticalLineIter {
            iter: self.iter.into_iter(),
            inter: self.inter,
            top: self.top,
            bottom: self.bottom,
        }
    }
}

impl<I> Iterator for VerticalLineIter<I>
where
    I: Iterator<Item = VerticalLine>,
{
    type Item = VerticalLine;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.iter.next()?;

        line_set_flag_char(&mut line.line.intersection, self.inter);
        line_set_flag_char(&mut line.line.connector1, self.top);
        line_set_flag_char(&mut line.line.connector2, self.bottom);

        Some(line)
    }
}

#[derive(Debug, Clone, Copy)]
enum CharFlag {
    Set(char),
    Unset,
}

fn line_set_flag_char(char: &mut Option<char>, flag: Option<CharFlag>) {
    match flag {
        Some(CharFlag::Set(c)) => {
            *char = Some(c);
        }
        Some(CharFlag::Unset) => {
            *char = None;
        }
        None => {}
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
