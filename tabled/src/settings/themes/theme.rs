//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conveniently.

// todo: StyleFromTable()
//       table.with(&mut StyleFromTable);
//       vs
//       Theme::from(table.get_config());
//
// not sure what the best interface is
// IMHO 2

use std::collections::HashMap;

use crate::{
    grid::config::{
        Border, Borders, ColoredConfig, CompactConfig, CompactMultilineConfig, HorizontalLine,
        VerticalLine,
    },
    settings::{style::Style, Color, TableOption},
};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    chars: Borders<char>,
    colors: Borders<Color>,
    lines_horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
    lines_verticals: Option<HashMap<usize, VerticalLine<char>>>,
    lines_horizontal1: Option<HorizontalLine<char>>,
}

impl Theme {
    /// Creates a new empty style.
    ///
    /// It's an analog of [`Style::empty`]
    pub const fn new() -> Self {
        Self::gen(Borders::empty(), Borders::empty(), None, None, None)
    }

    /// Build a theme out of a style builder.
    pub const fn from_style<T, B, L, R, H, V, const HS: usize, const VS: usize>(
        style: Style<T, B, L, R, H, V, HS, VS>,
    ) -> Self {
        let chars = style.get_borders();
        let hlines = style.get_horizontals();
        let hlines1 = hlines_find(hlines, 1);

        Self::gen(chars, Borders::empty(), None, None, hlines1)
    }

    /// Returns an outer border of the style.
    pub fn set_frame(&mut self, frame: Border<char>) {
        self.chars.top = frame.top;
        self.chars.bottom = frame.bottom;
        self.chars.left = frame.left;
        self.chars.right = frame.right;
        self.chars.top_left = frame.left_top_corner;
        self.chars.top_right = frame.right_top_corner;
        self.chars.bottom_left = frame.left_bottom_corner;
        self.chars.bottom_right = frame.right_bottom_corner;
    }

    /// Returns an outer border of the style.
    pub fn set_frame_colors(&mut self, frame: Border<Color>) {
        self.colors.top = frame.top;
        self.colors.bottom = frame.bottom;
        self.colors.left = frame.left;
        self.colors.right = frame.right;
        self.colors.top_left = frame.left_top_corner;
        self.colors.top_right = frame.right_top_corner;
        self.colors.bottom_left = frame.left_bottom_corner;
        self.colors.bottom_right = frame.right_bottom_corner;
    }

    /// Set borders structure.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.chars = borders;
    }

    /// Set borders structure.
    pub fn set_colors(&mut self, borders: Borders<Color>) {
        self.colors = borders;
    }

    /// Get borders structure.
    pub const fn get_borders(&self) -> &Borders<char> {
        &self.chars
    }

    /// Get borders color structure.
    pub const fn get_borders_colors(&self) -> &Borders<Color> {
        &self.colors
    }

    /// Get borders structure.
    pub fn get_borders_mut(&mut self) -> &mut Borders<char> {
        &mut self.chars
    }

    /// Get borders color structure.
    pub fn get_colors_mut(&mut self) -> &mut Borders<Color> {
        &mut self.colors
    }

    /// Remove borders.
    pub fn remove_borders(&mut self) {
        self.set_borders(Borders::empty());
    }

    /// Remove colors.
    pub fn remove_colors(&mut self) {
        self.set_colors(Borders::empty());
    }

    /// Remove horizontal lines.
    pub fn remove_horizontal_lines(&mut self) {
        self.set_horizontal_lines(HashMap::new());
        self.lines_horizontal1 = None;
        self.chars.horizontal = None;
        self.chars.left_intersection = None;
        self.chars.right_intersection = None;
        self.chars.intersection = None;
    }

    /// Remove vertical lines.
    pub fn remove_vertical_lines(&mut self) {
        self.set_vertical_lines(HashMap::new());
        self.chars.vertical = None;
        self.chars.top_intersection = None;
        self.chars.bottom_intersection = None;
        self.chars.intersection = None;
    }

    /// Set an outer border.
    pub const fn get_frame(&self) -> Border<char> {
        Border {
            top: self.chars.top,
            bottom: self.chars.bottom,
            left: self.chars.left,
            right: self.chars.right,
            left_top_corner: self.chars.top_left,
            right_top_corner: self.chars.top_right,
            left_bottom_corner: self.chars.bottom_left,
            right_bottom_corner: self.chars.bottom_right,
        }
    }

    /// Set an outer border.
    pub const fn get_frame_colors(&self) -> Border<&Color> {
        Border {
            top: self.colors.top.as_ref(),
            bottom: self.colors.bottom.as_ref(),
            left: self.colors.left.as_ref(),
            right: self.colors.right.as_ref(),
            left_top_corner: self.colors.top_left.as_ref(),
            right_top_corner: self.colors.top_right.as_ref(),
            left_bottom_corner: self.colors.bottom_left.as_ref(),
            right_bottom_corner: self.colors.bottom_right.as_ref(),
        }
    }

    /// Set horizontal border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, HorizontalLine}, settings::themes::Theme};
    ///
    /// let mut style = Theme::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(Style::extended()).into());
    ///
    /// style.set_horizontal_lines(lines);
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
    pub fn set_horizontal_lines(&mut self, lines: HashMap<usize, HorizontalLine<char>>) {
        self.lines_horizontals = Some(lines);
    }

    /// Set vertical border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{
    ///     Table,
    ///     settings::style::{Style, HorizontalLine},
    ///     settings::themes::Theme,
    /// };
    ///
    ///
    /// let mut style = Theme::from_style(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(Style::extended()).into());
    ///
    /// style.set_vertical_lines(lines);
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
    pub fn set_vertical_lines(&mut self, lines: HashMap<usize, VerticalLine<char>>) {
        self.lines_verticals = Some(lines);
    }

    /// Insert a vertical line into specific column location.
    pub fn insert_vertical_line<L>(&mut self, line: usize, vertical: L)
    where
        L: Into<VerticalLine<char>>,
    {
        let vertical = vertical.into();

        let verticals = match &mut self.lines_verticals {
            Some(verticals) => verticals,
            None => {
                self.lines_verticals = Some(HashMap::with_capacity(1));
                self.lines_verticals.as_mut().expect("checked")
            }
        };

        let _ = verticals.insert(line, vertical);
    }

    /// Insert a horizontal line to a specific row location.
    pub fn insert_horizontal_line<L>(&mut self, line: usize, horizontal: L)
    where
        L: Into<HorizontalLine<char>>,
    {
        let horizontal = horizontal.into();

        let horizontals = match &mut self.lines_horizontals {
            Some(horizontals) => horizontals,
            None => {
                self.lines_horizontals = Some(HashMap::with_capacity(1));
                self.lines_horizontals.as_mut().expect("checked")
            }
        };

        let _ = horizontals.insert(line, horizontal);
    }

    /// Get a vertical line at the row if any set.
    pub fn get_vertical_line(&self, column: usize) -> Option<&VerticalLine<char>> {
        self.lines_verticals.as_ref().and_then(|m| m.get(&column))
    }

    /// Get a horizontal line at the row if any set.
    pub fn get_horizontal_line(&self, row: usize) -> Option<&HorizontalLine<char>> {
        let line = self.lines_horizontals.as_ref().and_then(|m| m.get(&row));
        if line.is_some() {
            return line;
        }

        if row == 1 && self.lines_horizontal1.is_some() {
            return self.lines_horizontal1.as_ref();
        }

        None
    }

    /// Verifies if borders has left line set on the frame.
    pub const fn borders_has_left(&self) -> bool {
        self.chars.has_left()
    }

    /// Verifies if borders has right line set on the frame.
    pub const fn borders_has_right(&self) -> bool {
        self.chars.has_right()
    }

    /// Verifies if borders has top line set on the frame.
    pub const fn borders_has_top(&self) -> bool {
        self.chars.has_top()
    }

    /// Verifies if borders has bottom line set on the frame.
    pub const fn borders_has_bottom(&self) -> bool {
        self.chars.has_bottom()
    }

    /// Verifies if borders has horizontal lines set.
    pub const fn borders_has_horizontal(&self) -> bool {
        self.chars.has_horizontal()
    }

    /// Verifies if borders has vertical lines set.
    pub const fn borders_has_vertical(&self) -> bool {
        self.chars.has_vertical()
    }

    const fn gen(
        chars: Borders<char>,
        colors: Borders<Color>,
        horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
        verticals: Option<HashMap<usize, VerticalLine<char>>>,
        horizontal1: Option<HorizontalLine<char>>,
    ) -> Self {
        Self {
            chars,
            colors,
            lines_horizontals: horizontals,
            lines_verticals: verticals,
            lines_horizontal1: horizontal1,
        }
    }
}

impl From<Borders<char>> for Theme {
    fn from(borders: Borders<char>) -> Self {
        Self::gen(
            borders,
            Borders::empty(),
            Default::default(),
            Default::default(),
            None,
        )
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for Theme {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg_clear_borders(cfg);
        cfg_set_custom_lines(
            cfg,
            self.lines_horizontals,
            self.lines_verticals,
            self.lines_horizontal1,
        );
        cfg_set_borders(cfg, self.chars, self.colors);
    }
}

impl<R, D> TableOption<R, CompactConfig, D> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_borders(self.chars);
    }
}

impl<R, D> TableOption<R, CompactMultilineConfig, D> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_borders(self.chars);
    }
}

impl<T, B, L, R, H, V, const HSIZE: usize, const VSIZE: usize>
    From<Style<T, B, L, R, H, V, HSIZE, VSIZE>> for Theme
{
    fn from(style: Style<T, B, L, R, H, V, HSIZE, VSIZE>) -> Self {
        Self::from_style(style)
    }
}

impl From<ColoredConfig> for Theme {
    fn from(cfg: ColoredConfig) -> Self {
        let borders = *cfg.get_borders();
        let colors = cfg.get_color_borders().clone().convert_into();
        let horizontals = cfg.get_horizontal_lines().into_iter().collect();
        let verticals = cfg.get_vertical_lines().into_iter().collect();

        Self::gen(borders, colors, Some(horizontals), Some(verticals), None)
    }
}

macro_rules! func_set_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border character", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self, c: char) {
            self.chars.$arg = Some(c);
        }
    };
}

macro_rules! func_remove_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border character", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self) {
            self.chars.$arg = None;
        }
    };
}

macro_rules! func_get_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Get a border character", " ", "", $desc, "", " ", ".")]
        pub const fn $name(&self) -> Option<char> {
            self.chars.$arg
        }
    };
}

macro_rules! func_set_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self, color: Color) {
            self.colors.$arg = Some(color);
        }
    };
}

macro_rules! func_remove_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self) {
            self.colors.$arg = None;
        }
    };
}

macro_rules! func_get_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Get a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&self) -> Option<&Color> {
            self.colors.$arg.as_ref()
        }
    };
}

#[rustfmt::skip]
impl Theme {
    func_set_chars!(set_borders_top,                      top,                        "top");
    func_set_chars!(set_borders_bottom,                   bottom,                     "bottom");
    func_set_chars!(set_borders_left,                     left,                       "left");
    func_set_chars!(set_borders_right,                    right,                      "right");
    func_set_chars!(set_borders_corner_top_left,          top_left,                   "top left corner");
    func_set_chars!(set_borders_corner_top_right,         top_right,                  "top right corner");
    func_set_chars!(set_borders_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_chars!(set_borders_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_chars!(set_borders_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_chars!(set_borders_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_chars!(set_borders_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_chars!(set_borders_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_chars!(set_borders_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_chars!(set_borders_horizontal,               horizontal,                 "horizontal");
    func_set_chars!(set_borders_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_chars!(get_borders_top,                      top,                        "top");
    func_get_chars!(get_borders_bottom,                   bottom,                     "bottom");
    func_get_chars!(get_borders_left,                     left,                       "left");
    func_get_chars!(get_borders_right,                    right,                      "right");
    func_get_chars!(get_borders_corner_top_left,          top_left,                   "top left corner");
    func_get_chars!(get_borders_corner_top_right,         top_right,                  "top right corner");
    func_get_chars!(get_borders_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_chars!(get_borders_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_chars!(get_borders_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_chars!(get_borders_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_chars!(get_borders_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_chars!(get_borders_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_chars!(get_borders_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_chars!(get_borders_horizontal,               horizontal,                 "horizontal");
    func_get_chars!(get_borders_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_chars!(remove_borders_top,                      top,                        "top");
    func_remove_chars!(remove_borders_bottom,                   bottom,                     "bottom");
    func_remove_chars!(remove_borders_left,                     left,                       "left");
    func_remove_chars!(remove_borders_right,                    right,                      "right");
    func_remove_chars!(remove_borders_corner_top_left,          top_left,                   "top left corner");
    func_remove_chars!(remove_borders_corner_top_right,         top_right,                  "top right corner");
    func_remove_chars!(remove_borders_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_chars!(remove_borders_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_chars!(remove_borders_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_chars!(remove_borders_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_chars!(remove_borders_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_chars!(remove_borders_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_chars!(remove_borders_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_chars!(remove_borders_horizontal,               horizontal,                 "horizontal");
    func_remove_chars!(remove_borders_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_set_colors!(set_colors_top,                      top,                        "top");
    func_set_colors!(set_colors_bottom,                   bottom,                     "bottom");
    func_set_colors!(set_colors_left,                     left,                       "left");
    func_set_colors!(set_colors_right,                    right,                      "right");
    func_set_colors!(set_colors_corner_top_left,          top_left,                   "top left corner");
    func_set_colors!(set_colors_corner_top_right,         top_right,                  "top right corner");
    func_set_colors!(set_colors_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_colors!(set_colors_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_colors!(set_colors_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_colors!(set_colors_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_colors!(set_colors_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_colors!(set_colors_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_colors!(set_colors_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_colors!(set_colors_horizontal,               horizontal,                 "horizontal");
    func_set_colors!(set_colors_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_colors!(remove_colors_top,                      top,                        "top");
    func_remove_colors!(remove_colors_bottom,                   bottom,                     "bottom");
    func_remove_colors!(remove_colors_left,                     left,                       "left");
    func_remove_colors!(remove_colors_right,                    right,                      "right");
    func_remove_colors!(remove_colors_corner_top_left,          top_left,                   "top left corner");
    func_remove_colors!(remove_colors_corner_top_right,         top_right,                  "top right corner");
    func_remove_colors!(remove_colors_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_colors!(remove_colors_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_colors!(remove_colors_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_colors!(remove_colors_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_colors!(remove_colors_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_colors!(remove_colors_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_colors!(remove_colors_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_colors!(remove_colors_horizontal,               horizontal,                 "horizontal");
    func_remove_colors!(remove_colors_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_colors!(get_colors_top,                      top,                        "top");
    func_get_colors!(get_colors_bottom,                   bottom,                     "bottom");
    func_get_colors!(get_colors_left,                     left,                       "left");
    func_get_colors!(get_colors_right,                    right,                      "right");
    func_get_colors!(get_colors_corner_top_left,          top_left,                   "top left corner");
    func_get_colors!(get_colors_corner_top_right,         top_right,                  "top right corner");
    func_get_colors!(get_colors_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_colors!(get_colors_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_colors!(get_colors_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_colors!(get_colors_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_colors!(get_colors_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_colors!(get_colors_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_colors!(get_colors_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_colors!(get_colors_horizontal,               horizontal,                 "horizontal");
    func_get_colors!(get_colors_vertical,                 vertical,                   "vertical");
}

fn cfg_clear_borders(cfg: &mut ColoredConfig) {
    cfg.remove_borders();
    cfg.remove_borders_colors();
    cfg.remove_vertical_chars();
    cfg.remove_horizontal_chars();
    cfg.remove_color_line_horizontal();
    cfg.remove_color_line_vertical();
}

fn cfg_set_borders(cfg: &mut ColoredConfig, borders: Borders<char>, colors: Borders<Color>) {
    cfg.set_borders(borders);

    if !colors.is_empty() {
        cfg.set_borders_color(colors.convert_into());
    }
}

fn cfg_set_custom_lines(
    cfg: &mut ColoredConfig,
    horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
    verticals: Option<HashMap<usize, VerticalLine<char>>>,
    horizontal1: Option<HorizontalLine<char>>,
) {
    if let Some(line) = horizontal1 {
        cfg.insert_horizontal_line(1, line);
    }

    if let Some(horizontals) = horizontals {
        for (row, line) in horizontals {
            cfg.insert_horizontal_line(row, line);
        }
    }

    if let Some(verticals) = verticals {
        for (col, line) in verticals {
            cfg.insert_vertical_line(col, line);
        }
    }
}

const fn hlines_find<const N: usize>(
    lines: [(usize, HorizontalLine<char>); N],
    search: usize,
) -> Option<HorizontalLine<char>> {
    let mut line = None;

    let mut i = 0;
    while i < lines.len() {
        let (num, hline) = lines[i];
        if num == search {
            line = Some(hline);
        }

        i += 1;
    }

    line
}
