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
use std::iter::FromIterator;

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
    const fn gen(
        chars: Borders<char>,
        colors: Borders<Color>,
        lines_horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
        lines_verticals: Option<HashMap<usize, VerticalLine<char>>>,
        lines_horizontal1: Option<HorizontalLine<char>>,
    ) -> Self {
        Self {
            chars,
            colors,
            lines_horizontals,
            lines_verticals,
            lines_horizontal1,
        }
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
}

impl Theme {
    /// Creates a new empty style.
    ///
    /// It's an analog of [`Style::empty`]
    pub const fn new() -> Self {
        Self::gen(Borders::empty(), Borders::empty(), None, None, None)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
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
    func_set_chars!(set_border_top,                      top,                        "top");
    func_set_chars!(set_border_bottom,                   bottom,                     "bottom");
    func_set_chars!(set_border_left,                     left,                       "left");
    func_set_chars!(set_border_right,                    right,                      "right");
    func_set_chars!(set_border_corner_top_left,          top_left,                   "top left corner");
    func_set_chars!(set_border_corner_top_right,         top_right,                  "top right corner");
    func_set_chars!(set_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_chars!(set_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_chars!(set_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_chars!(set_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_chars!(set_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_chars!(set_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_chars!(set_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_chars!(set_border_horizontal,               horizontal,                 "horizontal");
    func_set_chars!(set_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_chars!(get_border_top,                      top,                        "top");
    func_get_chars!(get_border_bottom,                   bottom,                     "bottom");
    func_get_chars!(get_border_left,                     left,                       "left");
    func_get_chars!(get_border_right,                    right,                      "right");
    func_get_chars!(get_border_corner_top_left,          top_left,                   "top left corner");
    func_get_chars!(get_border_corner_top_right,         top_right,                  "top right corner");
    func_get_chars!(get_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_chars!(get_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_chars!(get_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_chars!(get_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_chars!(get_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_chars!(get_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_chars!(get_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_chars!(get_border_horizontal,               horizontal,                 "horizontal");
    func_get_chars!(get_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_chars!(remove_border_top,                      top,                        "top");
    func_remove_chars!(remove_border_bottom,                   bottom,                     "bottom");
    func_remove_chars!(remove_border_left,                     left,                       "left");
    func_remove_chars!(remove_border_right,                    right,                      "right");
    func_remove_chars!(remove_border_corner_top_left,          top_left,                   "top left corner");
    func_remove_chars!(remove_border_corner_top_right,         top_right,                  "top right corner");
    func_remove_chars!(remove_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_chars!(remove_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_chars!(remove_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_chars!(remove_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_chars!(remove_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_chars!(remove_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_chars!(remove_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_chars!(remove_border_horizontal,               horizontal,                 "horizontal");
    func_remove_chars!(remove_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_set_colors!(set_border_color_top,                      top,                        "top");
    func_set_colors!(set_border_color_bottom,                   bottom,                     "bottom");
    func_set_colors!(set_border_color_left,                     left,                       "left");
    func_set_colors!(set_border_color_right,                    right,                      "right");
    func_set_colors!(set_border_color_corner_top_left,          top_left,                   "top left corner");
    func_set_colors!(set_border_color_corner_top_right,         top_right,                  "top right corner");
    func_set_colors!(set_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_colors!(set_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_colors!(set_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_colors!(set_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_colors!(set_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_colors!(set_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_colors!(set_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_colors!(set_border_color_horizontal,               horizontal,                 "horizontal");
    func_set_colors!(set_border_color_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_colors!(remove_border_color_top,                      top,                        "top");
    func_remove_colors!(remove_border_color_bottom,                   bottom,                     "bottom");
    func_remove_colors!(remove_border_color_left,                     left,                       "left");
    func_remove_colors!(remove_border_color_right,                    right,                      "right");
    func_remove_colors!(remove_border_color_corner_top_left,          top_left,                   "top left corner");
    func_remove_colors!(remove_border_color_corner_top_right,         top_right,                  "top right corner");
    func_remove_colors!(remove_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_colors!(remove_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_colors!(remove_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_colors!(remove_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_colors!(remove_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_colors!(remove_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_colors!(remove_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_colors!(remove_border_color_horizontal,               horizontal,                 "horizontal");
    func_remove_colors!(remove_border_color_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_colors!(get_border_color_top,                      top,                        "top");
    func_get_colors!(get_border_color_bottom,                   bottom,                     "bottom");
    func_get_colors!(get_border_color_left,                     left,                       "left");
    func_get_colors!(get_border_color_right,                    right,                      "right");
    func_get_colors!(get_border_color_corner_top_left,          top_left,                   "top left corner");
    func_get_colors!(get_border_color_corner_top_right,         top_right,                  "top right corner");
    func_get_colors!(get_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_colors!(get_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_colors!(get_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_colors!(get_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_colors!(get_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_colors!(get_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_colors!(get_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_colors!(get_border_color_horizontal,               horizontal,                 "horizontal");
    func_get_colors!(get_border_color_vertical,                 vertical,                   "vertical");
}

impl Theme {
    /// Returns an outer border of the style.
    pub fn set_border_frame(&mut self, frame: Border<char>) {
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
    pub fn set_border_color_frame(&mut self, frame: Border<Color>) {
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
    pub fn set_borders_colors(&mut self, borders: Borders<Color>) {
        self.colors = borders;
    }

    /// Set borders structure.
    pub const fn get_borders(&self) -> Borders<char> {
        self.chars
    }

    /// Set borders structure.
    pub fn get_borders_colors(&self) -> Borders<Color> {
        self.colors.clone()
    }

    /// Set an outer border.
    pub const fn get_border_frame(&self) -> Border<char> {
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
    pub const fn get_border_color_frame(&self) -> Border<&Color> {
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
}

impl Theme {
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
        self.lines_verticals = Some(lines);
    }

    /// Insert a vertical line into specific column location.
    pub fn insert_line_vertical(&mut self, line: usize, vertical: VerticalLine<char>) {
        match &mut self.lines_verticals {
            Some(verticals) => {
                let _ = verticals.insert(line, vertical);
            }
            None => self.lines_verticals = Some(HashMap::from_iter([(line, vertical)])),
        }
    }

    /// Insert a horizontal line to a specific row location.
    pub fn insert_line_horizontal(&mut self, line: usize, horizontal: HorizontalLine<char>) {
        match &mut self.lines_horizontals {
            Some(horizontals) => {
                let _ = horizontals.insert(line, horizontal);
            }
            None => self.lines_horizontals = Some(HashMap::from_iter([(line, horizontal)])),
        }
    }

    /// Get a vertical line at the row if any set.
    pub fn get_line_vertical(&self, column: usize) -> Option<VerticalLine<char>> {
        self.lines_verticals
            .as_ref()
            .and_then(|lines| lines.get(&column).cloned())
    }

    /// Get a horizontal line at the row if any set.
    pub fn get_line_horizontal(&self, row: usize) -> Option<HorizontalLine<char>> {
        self.lines_horizontals
            .as_ref()
            .and_then(|list| list.get(&row).cloned())
    }
}

impl From<Borders<char>> for Theme {
    fn from(borders: Borders<char>) -> Self {
        Self::gen(borders, Borders::empty(), None, None, None)
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

    if let Some(lines) = horizontals {
        for (row, line) in lines {
            cfg.insert_horizontal_line(row, line);
        }
    }

    if let Some(lines) = verticals {
        for (col, line) in lines {
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
