//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

use std::collections::HashMap;

use papergrid::{records::Records, Borders};

use crate::{
    style::{HorizontalLine, Line, VerticalLine},
    Border, Style, Table, TableOption,
};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Default, Debug, Clone)]
pub struct RawStyle {
    borders: Borders<char>,
    horizontals: HashMap<usize, Line>,
    verticals: HashMap<usize, Line>,
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
        self.borders.vertical = s;
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

    /// Set horizontal border lines.
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
    /// style.set_horizontals(lines);
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
    ///         "  Hello   0    \n",
    ///         "  Hello   1    \n",
    ///         "  Hello   2    \n",
    ///         " ======= ===== ",
    ///     ),
    /// )
    /// ```
    pub fn set_horizontals(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.horizontals = lines;
        self
    }

    /// Set vertical border lines.
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
    /// style.set_verticals(lines);
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
    pub fn set_verticals(&mut self, lines: HashMap<usize, Line>) -> &mut Self {
        self.verticals = lines;
        self
    }

    /// Get a left char.
    pub fn get_left(&self) -> Option<char> {
        self.borders.vertical_left
    }

    /// Get a left intersection char.
    pub fn get_left_intersection(&self) -> Option<char> {
        self.borders.horizontal_left
    }

    /// Get a right char.
    pub fn get_right(&self) -> Option<char> {
        self.borders.vertical_right
    }

    /// Get a right intersection char.
    pub fn get_right_intersection(&self) -> Option<char> {
        self.borders.horizontal_right
    }

    /// Get a top char.
    pub fn get_top(&self) -> Option<char> {
        self.borders.top
    }

    /// Get a top left char.
    pub fn get_top_left(&self) -> Option<char> {
        self.borders.top_left
    }

    /// Get a top right char.
    pub fn get_top_right(&self) -> Option<char> {
        self.borders.top_right
    }

    /// Get a top intersection char.
    pub fn get_top_intersection(&self) -> Option<char> {
        self.borders.top_intersection
    }

    /// Get a bottom intersection char.
    pub fn get_bottom(&self) -> Option<char> {
        self.borders.bottom
    }

    /// Get a bottom intersection char.
    pub fn get_bottom_left(&self) -> Option<char> {
        self.borders.bottom_left
    }

    /// Get a bottom intersection char.
    pub fn get_bottom_right(&self) -> Option<char> {
        self.borders.bottom_right
    }

    /// Get a bottom intersection char.
    pub fn get_bottom_intersection(&self) -> Option<char> {
        self.borders.bottom_intersection
    }

    /// Returns an outer border of the style.
    pub fn get_frame(&self) -> Border {
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

    /// Returns a [`RawStyle`] version which can set colors.
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    #[cfg(feature = "color")]
    pub fn colored(self) -> crate::style::RawStyleColored {
        crate::style::RawStyleColored::from(self)
    }
}

impl From<Borders<char>> for RawStyle {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            horizontals: HashMap::new(),
            verticals: HashMap::new(),
        }
    }
}

impl<R> TableOption<R> for RawStyle
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        (&*self).change(table)
    }
}

impl<R> TableOption<R> for &RawStyle
where
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let (count_rows, count_cols) = table.shape();

        let cfg = table.get_config_mut();
        cfg.clear_theme();
        cfg.set_borders(self.borders.clone());

        if count_rows > 1 {
            for (&row, line) in &self.horizontals {
                if line.is_empty() {
                    cfg.remove_horizontal_line(row);
                } else {
                    cfg.set_horizontal_line(row, papergrid::HorizontalLine::from(*line));
                }
            }
        }

        if count_cols > 1 {
            for (&col, line) in &self.verticals {
                if line.is_empty() {
                    cfg.remove_vertical_line(col);
                } else {
                    cfg.set_vertical_line(col, papergrid::VerticalLine::from(*line));
                }
            }
        }

        table.destroy_width_cache();
        table.destroy_height_cache();
    }
}

impl<T, B, L, R, H, V, HLines, VLines> From<Style<T, B, L, R, H, V, HLines, VLines>> for RawStyle
where
    HLines: IntoIterator<Item = HorizontalLine>,
    VLines: IntoIterator<Item = VerticalLine>,
{
    fn from(style: Style<T, B, L, R, H, V, HLines, VLines>) -> Self {
        let horizontals = style
            .horizontals
            .into_iter()
            .flat_map(|hr| {
                let index = hr.index;
                hr.line.map(|line| (index, line))
            })
            .collect();

        let verticals = style
            .verticals
            .into_iter()
            .flat_map(|hr| {
                let index = hr.index;
                hr.line.map(|line| (index, line))
            })
            .collect();

        Self {
            borders: style.borders,
            horizontals,
            verticals,
        }
    }
}
