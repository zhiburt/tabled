//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

use std::collections::HashMap;

use crate::{
    grid::{color::AnsiColor, config, config::Borders, config::ColoredConfig, records::Records},
    settings::{Color, TableOption},
};

use super::{Border, HorizontalLine, Line, Style, VerticalLine};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Default, Debug, Clone)]
pub struct RawStyle {
    borders: Borders<char>,
    colors: Borders<AnsiColor<'static>>,
    horizontals: HashMap<usize, Line>,
    verticals: HashMap<usize, Line>,
}

impl RawStyle {
    /// Set a top border character.
    pub fn set_top(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top = s;
        self
    }

    /// Set a top border color.
    pub fn set_color_top(&mut self, color: Color) -> &mut Self {
        self.colors.top = Some(color.into());
        self
    }

    /// Set a bottom border character.
    pub fn set_bottom(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom = s;
        self
    }

    /// Set a bottom border color.
    pub fn set_color_bottom(&mut self, color: Color) -> &mut Self {
        self.colors.bottom = Some(color.into());
        self
    }

    /// Set a left border character.
    pub fn set_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.left = s;
        self
    }

    /// Set a left border color.
    pub fn set_color_left(&mut self, color: Color) -> &mut Self {
        self.colors.left = Some(color.into());
        self
    }

    /// Set a right border character.
    pub fn set_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.right = s;
        self
    }

    /// Set a right border color.
    pub fn set_color_right(&mut self, color: Color) -> &mut Self {
        self.colors.right = Some(color.into());
        self
    }

    /// Set a top intersection character.
    pub fn set_intersection_top(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_intersection = s;
        self
    }

    /// Set a top intersection color.
    pub fn set_color_intersection_top(&mut self, color: Color) -> &mut Self {
        self.colors.top_intersection = Some(color.into());
        self
    }

    /// Set a bottom intersection character.
    pub fn set_intersection_bottom(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_intersection = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_bottom(&mut self, color: Color) -> &mut Self {
        self.colors.bottom_intersection = Some(color.into());
        self
    }

    /// Set a left split character.
    pub fn set_intersection_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.left_intersection = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_left(&mut self, color: Color) -> &mut Self {
        self.colors.left_intersection = Some(color.into());
        self
    }

    /// Set a right split character.
    pub fn set_intersection_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.right_intersection = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection_right(&mut self, color: Color) -> &mut Self {
        self.colors.right_intersection = Some(color.into());
        self
    }

    /// Set an internal character.
    pub fn set_intersection(&mut self, s: Option<char>) -> &mut Self {
        self.borders.intersection = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_intersection(&mut self, color: Color) -> &mut Self {
        self.colors.intersection = Some(color.into());
        self
    }

    /// Set a vertical character.
    pub fn set_vertical(&mut self, s: Option<char>) -> &mut Self {
        self.borders.vertical = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_vertical(&mut self, color: Color) -> &mut Self {
        self.colors.vertical = Some(color.into());
        self
    }

    /// Set a horizontal character.
    pub fn set_horizontal(&mut self, s: Option<char>) -> &mut Self {
        self.borders.horizontal = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_horizontal(&mut self, color: Color) -> &mut Self {
        self.colors.horizontal = Some(color.into());
        self
    }

    /// Set a character for a top left corner.
    pub fn set_corner_top_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_left = s;
        self
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_top_left(&mut self, color: Color) -> &mut Self {
        self.colors.top_left = Some(color.into());
        self
    }

    /// Set a character for a top right corner.
    pub fn set_corner_top_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.top_right = s;
        self
    }

    /// Set a bottom intersection color.
    pub fn set_color_corner_top_right(&mut self, color: Color) -> &mut Self {
        self.colors.top_right = Some(color.into());
        self
    }

    /// Set a character for a bottom left corner.
    pub fn set_corner_bottom_left(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_left = s;
        self
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_bottom_left(&mut self, color: Color) -> &mut Self {
        self.colors.bottom_left = Some(color.into());
        self
    }

    /// Set a character for a bottom right corner.
    pub fn set_corner_bottom_right(&mut self, s: Option<char>) -> &mut Self {
        self.borders.bottom_right = s;
        self
    }
    /// Set a bottom intersection color.
    pub fn set_color_corner_bottom_right(&mut self, color: Color) -> &mut Self {
        self.colors.bottom_right = Some(color.into());
        self
    }

    /// Set horizontal border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, Line, RawStyle}};
    ///
    /// let mut style = RawStyle::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, Style::extended().get_horizontal());
    /// style.set_horizontals(lines);
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", i)))
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

    /// Insert a horizontal line to a specific row location.
    pub fn insert_horizontal(&mut self, row: usize, line: Line) -> &mut Self {
        let _ = self.horizontals.insert(row, line);
        self
    }

    /// Insert a horizontal line to a specific row location.
    pub fn get_horizontal(&self, row: usize) -> Option<Line> {
        self.horizontals.get(&row).cloned()
    }

    /// Set vertical border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, Line, RawStyle}};
    ///
    /// let mut style = RawStyle::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, Style::extended().get_horizontal());
    /// style.set_verticals(lines);
    ///
    /// let table = Table::new((0..3).map(|i| ("Hello", i)))
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

    /// Insert a vertical line into specific column location.
    pub fn insert_vertical(&mut self, column: usize, line: Line) -> &mut Self {
        let _ = self.verticals.insert(column, line);
        self
    }

    /// Get a left char.
    pub fn get_left(&self) -> Option<char> {
        self.borders.left
    }

    /// Get a left intersection char.
    pub fn get_left_intersection(&self) -> Option<char> {
        self.borders.left_intersection
    }

    /// Get a right char.
    pub fn get_right(&self) -> Option<char> {
        self.borders.right
    }

    /// Get a right intersection char.
    pub fn get_right_intersection(&self) -> Option<char> {
        self.borders.right_intersection
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
        Border::from(crate::grid::config::Border {
            top: self.borders.top,
            bottom: self.borders.bottom,
            left: self.borders.left,
            right: self.borders.right,
            left_top_corner: self.borders.top_left,
            right_top_corner: self.borders.top_right,
            left_bottom_corner: self.borders.bottom_left,
            right_bottom_corner: self.borders.bottom_right,
        })
    }

    /// Returns an general borders configuration of the style.
    pub fn get_borders(&self) -> Borders<char> {
        self.borders
    }
}

impl From<Borders<char>> for RawStyle {
    fn from(borders: Borders<char>) -> Self {
        Self {
            borders,
            horizontals: HashMap::new(),
            verticals: HashMap::new(),
            colors: Borders::default(),
        }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for RawStyle
where
    R: Records,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dimension: &mut D) {
        (&self).change(records, cfg, dimension)
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for &RawStyle {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg.clear_theme();

        cfg.set_borders(self.borders);

        for (&row, line) in &self.horizontals {
            cfg.insert_horizontal_line(row, config::HorizontalLine::from(*line));
        }

        for (&col, line) in &self.verticals {
            cfg.insert_vertical_line(col, config::VerticalLine::from(*line));
        }

        if !self.colors.is_empty() {
            cfg.set_borders_color(self.colors.clone());
        }
    }
}

impl<T, B, L, R, H, V, HLines, VLines> From<Style<T, B, L, R, H, V, HLines, VLines>> for RawStyle
where
    HLines: IntoIterator<Item = HorizontalLine> + Clone,
    VLines: IntoIterator<Item = VerticalLine> + Clone,
{
    fn from(style: Style<T, B, L, R, H, V, HLines, VLines>) -> Self {
        let horizontals = style
            .get_horizontals()
            .clone()
            .into_iter()
            .map(|hr| (hr.index, hr.line))
            .collect();

        let verticals = style
            .get_verticals()
            .clone()
            .into_iter()
            .map(|hr| (hr.index, hr.line))
            .collect();

        Self {
            borders: *style.get_borders(),
            horizontals,
            verticals,
            colors: Borders::default(),
        }
    }
}
