//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

use core::iter::FromIterator;
use std::collections::HashMap;

use papergrid::config::compact::CompactConfig;

use crate::{
    grid::{
        color::AnsiColor,
        config::{
            Border, Borders, ColoredConfig, CompactMultilineConfig, HorizontalLine, VerticalLine,
        },
        records::Records,
    },
    settings::{Color, TableOption},
};

use super::StyleBuilder;

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    borders: Borders<char>,
    colors: Borders<AnsiColor<'static>>,
    horizontal1: Option<HorizontalLine<char>>,
    horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
    verticals: Option<HashMap<usize, VerticalLine<char>>>,
}

impl Style {
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
    /// let mut style = Style::empty();
    /// style.set_top(Some('*'));
    /// style.set_bottom(Some('*'));
    /// style.set_vertical(Some('#'));
    /// style.set_intersection_top(Some('*'));
    /// ```
    pub const fn empty() -> Style {
        Self::with(StyleBuilder::empty())
    }

    /// This style is analog of `empty` but with a vertical space(' ') line.
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const fn blank() -> Style {
        Self::with(StyleBuilder::blank())
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
    pub const fn ascii() -> Style {
        Self::with(StyleBuilder::ascii())
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
    pub const fn psql() -> Style {
        Self::with(StyleBuilder::psql())
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
    pub const fn markdown() -> Style {
        Self::with(StyleBuilder::markdown())
    }

    /// This style is analog of [`StyleBuilder::ascii`] which uses UTF-8 charset.
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
    pub const fn modern() -> Style {
        Self::with(StyleBuilder::modern())
    }

    /// This style looks like a [`StyleBuilder::modern`] but without horozizontal lines except a header.
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
    pub const fn sharp() -> Style {
        Self::with(StyleBuilder::sharp())
    }

    /// This style looks like a [`StyleBuilder::sharp`] but with rounded corners.
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
    pub const fn rounded() -> Style {
        Self::with(StyleBuilder::rounded())
    }

    /// This style looks like a [`StyleBuilder::rounded`] but with horizontals lines.
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
    pub const fn modern_rounded() -> Style {
        Self::with(StyleBuilder::modern_rounded())
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
    pub const fn extended() -> Style {
        Self::with(StyleBuilder::extended())
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
    pub const fn dots() -> Style {
        Self::with(StyleBuilder::dots())
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
    pub const fn re_structured_text() -> Style {
        Self::with(StyleBuilder::re_structured_text())
    }

    /// This is a theme analog of [`StyleBuilder::rounded`], but in using ascii charset and
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
    pub const fn ascii_rounded() -> Style {
        Self::with(StyleBuilder::ascii_rounded())
    }
}

impl Style {
    /// Creates a new empty style.
    ///
    /// It's quite an analog of [`Style::empty`]
    pub const fn new() -> Self {
        Self {
            borders: Borders::empty(),
            colors: Borders::empty(),
            horizontal1: None,
            horizontals: None,
            verticals: None,
        }
    }

    const fn with<A, B, C, D, E, J, const HS: usize, const VS: usize>(
        builder: StyleBuilder<A, B, C, D, E, J, HS, VS>,
    ) -> Self {
        let borders = builder.get_borders();

        let horizontals = builder.get_horizontals();
        let mut horizontal1 = None;

        let mut i = 0;
        while i < horizontals.len() {
            let (line, hline) = horizontals[i];
            if line == 1 {
                horizontal1 = Some(hline);
            }

            i += 1;
        }

        Self {
            borders,
            horizontal1,
            colors: Borders::empty(),
            horizontals: None,
            verticals: None,
        }
    }
}

impl Style {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<char>) {
        self.borders.top = s
    }

    /// Set a top border color.
    pub fn set_color_top(&mut self, color: Color) {
        self.colors.top = Some(color.into())
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<char>) {
        self.borders.bottom = s
    }

    /// Set a bottom border color.
    pub fn set_color_bottom(&mut self, color: Color) {
        self.colors.bottom = Some(color.into())
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<char>) {
        self.borders.left = s
    }

    /// Set a left border color.
    pub fn set_color_left(&mut self, color: Color) {
        self.colors.left = Some(color.into())
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<char>) {
        self.borders.right = s
    }

    /// Set a right border color.
    pub fn set_color_right(&mut self, color: Color) {
        self.colors.right = Some(color.into())
    }

    /// Set a top intersection character.
    pub fn set_intersection_top(&mut self, s: Option<char>) {
        self.borders.top_intersection = s
    }

    /// Set a top intersection color.
    pub fn set_color_intersection_top(&mut self, color: Color) {
        self.colors.top_intersection = Some(color.into())
    }

    /// Set a bottom intersection character.
    pub fn set_intersection_bottom(&mut self, s: Option<char>) {
        self.borders.bottom_intersection = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_bottom(&mut self, color: Color) {
        self.colors.bottom_intersection = Some(color.into())
    }

    /// Set a left split character.
    pub fn set_intersection_left(&mut self, s: Option<char>) {
        self.borders.left_intersection = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_left(&mut self, color: Color) {
        self.colors.left_intersection = Some(color.into())
    }

    /// Set a right split character.
    pub fn set_intersection_right(&mut self, s: Option<char>) {
        self.borders.right_intersection = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_right(&mut self, color: Color) {
        self.colors.right_intersection = Some(color.into())
    }

    /// Set an internal character.
    pub fn set_intersection(&mut self, s: Option<char>) {
        self.borders.intersection = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection(&mut self, color: Color) {
        self.colors.intersection = Some(color.into())
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<char>) {
        self.borders.vertical = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_vertical(&mut self, color: Color) {
        self.colors.vertical = Some(color.into())
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<char>) {
        self.borders.horizontal = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_horizontal(&mut self, color: Color) {
        self.colors.horizontal = Some(color.into())
    }

    /// Set a character for a top left corner.
    pub fn set_corner_top_left(&mut self, s: Option<char>) {
        self.borders.top_left = s
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_top_left(&mut self, color: Color) {
        self.colors.top_left = Some(color.into())
    }

    /// Set a character for a top right corner.
    pub fn set_corner_top_right(&mut self, s: Option<char>) {
        self.borders.top_right = s
    }

    /// Set a bottom intersection color.
    pub fn set_color_corner_top_right(&mut self, color: Color) {
        self.colors.top_right = Some(color.into())
    }

    /// Set a character for a bottom left corner.
    pub fn set_corner_bottom_left(&mut self, s: Option<char>) {
        self.borders.bottom_left = s
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_bottom_left(&mut self, color: Color) {
        self.colors.bottom_left = Some(color.into())
    }

    /// Set a character for a bottom right corner.
    pub fn set_corner_bottom_right(&mut self, s: Option<char>) {
        self.borders.bottom_right = s
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_bottom_right(&mut self, color: Color) {
        self.colors.bottom_right = Some(color.into())
    }

    /// Returns an outer border of the style.
    pub fn set_frame(&mut self, frame: Border<char>) {
        self.borders.top = frame.top;
        self.borders.bottom = frame.bottom;
        self.borders.left = frame.left;
        self.borders.right = frame.right;
        self.borders.top_left = frame.left_top_corner;
        self.borders.top_right = frame.right_top_corner;
        self.borders.bottom_left = frame.left_bottom_corner;
        self.borders.bottom_right = frame.right_bottom_corner;
    }

    /// Set borders structure.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.borders = borders;
    }

    /// Set horizontal border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, StyleBuilder, HorizontalLine}};
    ///
    /// let mut style = Style::re_structured_text();
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(StyleBuilder::extended()).into());
    ///
    /// style.set_lines_horizontal(lines);
    ///
    /// let data = (0..3).map(|i| ("Hello", i));
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         " ======= ===== \n",
    ///         "  &str    i32  \n",
    ///         "╠═══════╬═════╣\n",
    ///         "  Hello   0    \n",
    ///         "  Hello   1    \n",
    ///         "  Hello   2    \n",
    ///         " ======= ===== ",
    ///     ),
    /// )
    /// ```
    pub fn set_lines_horizontal(&mut self, lines: HashMap<usize, HorizontalLine<char>>) {
        self.horizontals = Some(lines);
    }

    /// Set vertical border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, StyleBuilder, HorizontalLine}};
    ///
    ///
    /// let mut style = Style::re_structured_text();
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(StyleBuilder::extended()).into());
    ///
    /// style.set_lines_vertical(lines);
    ///
    /// let data = (0..3).map(|i| ("Hello", i));
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "=======╠=====\n",
    ///         " &str  ═ i32 \n",
    ///         "======= =====\n",
    ///         " Hello ═ 0   \n",
    ///         " Hello ═ 1   \n",
    ///         " Hello ═ 2   \n",
    ///         "=======╣=====",
    ///     ),
    /// )
    /// ```
    pub fn set_lines_vertical(&mut self, lines: HashMap<usize, VerticalLine<char>>) {
        self.verticals = Some(lines);
    }

    /// Insert a vertical line into specific column location.
    pub fn insert_line_vertical(&mut self, line: usize, vertical: VerticalLine<char>) {
        match &mut self.verticals {
            Some(verticals) => {
                let _ = verticals.insert(line, vertical);
            }
            None => self.verticals = Some(HashMap::from_iter([(line, vertical)])),
        }
    }

    /// Insert a horizontal line to a specific row location.
    pub fn insert_line_horizontal(&mut self, line: usize, horizontal: HorizontalLine<char>) {
        match &mut self.horizontals {
            Some(horizontals) => {
                let _ = horizontals.insert(line, horizontal);
            }
            None => self.horizontals = Some(HashMap::from_iter([(line, horizontal)])),
        }
    }

    /// Get a vertical line at the row if any set.
    pub fn get_line_vertical(&self, column: usize) -> Option<VerticalLine<char>> {
        self.verticals
            .as_ref()
            .and_then(|lines| lines.get(&column).cloned())
    }

    /// Get a horizontal line at the row if any set.
    pub fn get_line_horizontal(&self, row: usize) -> Option<HorizontalLine<char>> {
        self.horizontals
            .as_ref()
            .and_then(|list| list.get(&row).cloned())
    }

    /// Get a left char.
    pub const fn get_left(&self) -> Option<char> {
        self.borders.left
    }

    /// Get a left intersection char.
    pub const fn get_left_intersection(&self) -> Option<char> {
        self.borders.left_intersection
    }

    /// Get a right char.
    pub const fn get_right(&self) -> Option<char> {
        self.borders.right
    }

    /// Get a right intersection char.
    pub const fn get_right_intersection(&self) -> Option<char> {
        self.borders.right_intersection
    }

    /// Get a top char.
    pub const fn get_top(&self) -> Option<char> {
        self.borders.top
    }

    /// Get a top left char.
    pub const fn get_top_left(&self) -> Option<char> {
        self.borders.top_left
    }

    /// Get a top right char.
    pub const fn get_top_right(&self) -> Option<char> {
        self.borders.top_right
    }

    /// Get a top intersection char.
    pub const fn get_top_intersection(&self) -> Option<char> {
        self.borders.top_intersection
    }

    /// Get a bottom intersection char.
    pub const fn get_bottom(&self) -> Option<char> {
        self.borders.bottom
    }

    /// Get a bottom intersection char.
    pub const fn get_bottom_left(&self) -> Option<char> {
        self.borders.bottom_left
    }

    /// Get a bottom intersection char.
    pub const fn get_bottom_right(&self) -> Option<char> {
        self.borders.bottom_right
    }

    /// Get a bottom intersection char.
    pub const fn get_bottom_intersection(&self) -> Option<char> {
        self.borders.bottom_intersection
    }

    /// Set an outer border.
    pub const fn get_frame(&self) -> Border<char> {
        Border {
            top: self.borders.top,
            bottom: self.borders.bottom,
            left: self.borders.left,
            right: self.borders.right,
            left_top_corner: self.borders.top_left,
            right_top_corner: self.borders.top_right,
            left_bottom_corner: self.borders.bottom_left,
            right_bottom_corner: self.borders.bottom_right,
        }
    }

    /// Returns an general borders configuration of the style.
    pub const fn get_borders(&self) -> Borders<char> {
        self.borders
    }
}

impl From<Borders<char>> for Style {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            horizontals: None,
            verticals: None,
            horizontal1: None,
            colors: Borders::default(),
        }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Style
where
    R: Records,
{
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg.clear_theme();
        cfg.set_borders(self.borders);

        if let Some(line) = self.horizontal1 {
            cfg.insert_horizontal_line(1, line);
        }

        if let Some(lines) = self.horizontals {
            for (row, line) in lines {
                cfg.insert_horizontal_line(row, line);
            }
        }

        if let Some(lines) = self.verticals {
            for (col, line) in lines {
                cfg.insert_vertical_line(col, line);
            }
        }

        if !self.colors.is_empty() {
            cfg.set_borders_color(self.colors.clone());
        }
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for Style {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_borders(self.borders);
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for Style {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_borders(self.borders);
    }
}

impl<T, B, L, R, H, V, const HSIZE: usize, const VSIZE: usize>
    From<StyleBuilder<T, B, L, R, H, V, HSIZE, VSIZE>> for Style
where
    T: Copy,
    B: Copy,
    L: Copy,
    R: Copy,
    H: Copy,
    V: Copy,
{
    fn from(style: StyleBuilder<T, B, L, R, H, V, HSIZE, VSIZE>) -> Self {
        style.build()
    }
}

impl From<ColoredConfig> for Style {
    fn from(cfg: ColoredConfig) -> Self {
        let borders = *cfg.get_borders();
        let colors = cfg.get_color_borders().clone();
        let horizontals = cfg.get_horizontal_lines().into_iter().collect();
        let verticals = cfg.get_vertical_lines().into_iter().collect();

        Self {
            borders,
            colors,
            horizontal1: None,
            horizontals: Some(horizontals),
            verticals: Some(verticals),
        }
    }
}
