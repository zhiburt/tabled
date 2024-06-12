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
    grid::{
        config::{
            AlignmentHorizontal, AlignmentVertical, Border, Borders, ColoredConfig, CompactConfig,
            CompactMultilineConfig, HorizontalLine, VerticalLine,
        },
        records::{ExactRecords, PeekableRecords, Records, RecordsMut, Resizable},
    },
    settings::{style::Style, themes::Colorization, Alignment, Color, Rotate, TableOption},
};

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    border: TableBorders,
    lines: BorderLines,
    layout: Layout,
    colorization: Option<Colorization>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TableBorders {
    chars: Borders<char>,
    colors: Borders<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BorderLines {
    horizontal1: Option<HorizontalLine<char>>,
    horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
    verticals: Option<HashMap<usize, VerticalLine<char>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Layout {
    orientation: HeadPosition,
    footer: bool,
    reverse_rows: bool,
    reverse_column: bool,
    move_header_on_borders: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HeadPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Theme {
    /// Build a theme out of a style builder.
    pub const fn from_style<T, B, L, R, H, V, const HS: usize, const VS: usize>(
        style: Style<T, B, L, R, H, V, HS, VS>,
    ) -> Self {
        let chars = style.get_borders();
        let horizontals = style.get_horizontals();
        let horizontal1 = hlines_find(horizontals, 1);

        Self::_new(
            TableBorders::new(chars, Borders::empty()),
            BorderLines::new(horizontal1, None, None),
            Layout::new(HeadPosition::Top, false, false, false, false),
            None,
        )
    }
}

impl Theme {
    /// Creates a new empty style.
    ///
    /// It's quite an analog of [`Style::empty`]
    pub const fn new() -> Self {
        Self::_new(
            TableBorders::new(Borders::empty(), Borders::empty()),
            BorderLines::new(None, None, None),
            Layout::new(HeadPosition::Top, false, false, false, false),
            None,
        )
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
            self.border.chars.$arg = Some(c);
        }
    };
}

macro_rules! func_remove_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border character", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self) {
            self.border.chars.$arg = None;
        }
    };
}

macro_rules! func_get_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Get a border character", " ", "", $desc, "", " ", ".")]
        pub const fn $name(&self) -> Option<char> {
            self.border.chars.$arg
        }
    };
}

macro_rules! func_set_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self, color: Color) {
            self.border.colors.$arg = Some(color);
        }
    };
}

macro_rules! func_remove_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&mut self) {
            self.border.colors.$arg = None;
        }
    };
}

macro_rules! func_get_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Get a border color", " ", "", $desc, "", " ", ".")]
        pub fn $name(&self) -> Option<&Color> {
            self.border.colors.$arg.as_ref()
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
        self.border.chars.top = frame.top;
        self.border.chars.bottom = frame.bottom;
        self.border.chars.left = frame.left;
        self.border.chars.right = frame.right;
        self.border.chars.top_left = frame.left_top_corner;
        self.border.chars.top_right = frame.right_top_corner;
        self.border.chars.bottom_left = frame.left_bottom_corner;
        self.border.chars.bottom_right = frame.right_bottom_corner;
    }

    /// Returns an outer border of the style.
    pub fn set_border_color_frame(&mut self, frame: Border<Color>) {
        self.border.colors.top = frame.top;
        self.border.colors.bottom = frame.bottom;
        self.border.colors.left = frame.left;
        self.border.colors.right = frame.right;
        self.border.colors.top_left = frame.left_top_corner;
        self.border.colors.top_right = frame.right_top_corner;
        self.border.colors.bottom_left = frame.left_bottom_corner;
        self.border.colors.bottom_right = frame.right_bottom_corner;
    }

    /// Set borders structure.
    pub fn set_border(&mut self, borders: Borders<char>) {
        self.border.chars = borders;
    }

    /// Set borders structure.
    pub fn set_border_color(&mut self, borders: Borders<Color>) {
        self.border.colors = borders;
    }

    /// Set an outer border.
    pub const fn get_border_frame(&self) -> Border<char> {
        Border {
            top: self.border.chars.top,
            bottom: self.border.chars.bottom,
            left: self.border.chars.left,
            right: self.border.chars.right,
            left_top_corner: self.border.chars.top_left,
            right_top_corner: self.border.chars.top_right,
            left_bottom_corner: self.border.chars.bottom_left,
            right_bottom_corner: self.border.chars.bottom_right,
        }
    }

    /// Set an outer border.
    pub const fn get_border_color_frame(&self) -> Border<&Color> {
        Border {
            top: self.border.colors.top.as_ref(),
            bottom: self.border.colors.bottom.as_ref(),
            left: self.border.colors.left.as_ref(),
            right: self.border.colors.right.as_ref(),
            left_top_corner: self.border.colors.top_left.as_ref(),
            right_top_corner: self.border.colors.top_right.as_ref(),
            left_bottom_corner: self.border.colors.bottom_left.as_ref(),
            right_bottom_corner: self.border.colors.bottom_right.as_ref(),
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
        self.lines.horizontals = Some(lines);
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
        self.lines.verticals = Some(lines);
    }

    /// Insert a vertical line into specific column location.
    pub fn insert_line_vertical(&mut self, line: usize, vertical: VerticalLine<char>) {
        match &mut self.lines.verticals {
            Some(verticals) => {
                let _ = verticals.insert(line, vertical);
            }
            None => self.lines.verticals = Some(HashMap::from_iter([(line, vertical)])),
        }
    }

    /// Insert a horizontal line to a specific row location.
    pub fn insert_line_horizontal(&mut self, line: usize, horizontal: HorizontalLine<char>) {
        match &mut self.lines.horizontals {
            Some(horizontals) => {
                let _ = horizontals.insert(line, horizontal);
            }
            None => self.lines.horizontals = Some(HashMap::from_iter([(line, horizontal)])),
        }
    }

    /// Get a vertical line at the row if any set.
    pub fn get_line_vertical(&self, column: usize) -> Option<VerticalLine<char>> {
        self.lines
            .verticals
            .as_ref()
            .and_then(|lines| lines.get(&column).cloned())
    }

    /// Get a horizontal line at the row if any set.
    pub fn get_line_horizontal(&self, row: usize) -> Option<HorizontalLine<char>> {
        self.lines
            .horizontals
            .as_ref()
            .and_then(|list| list.get(&row).cloned())
    }
}

impl Theme {
    /// Reverse rows.
    pub fn reverse_rows(&mut self, reverse: bool) {
        self.layout.reverse_rows = reverse;
    }

    /// Reverse columns.
    pub fn reverse_columns(&mut self, reverse: bool) {
        self.layout.reverse_column = reverse;
    }

    /// Set a footer.
    ///
    /// Copy columns names to an apposite side of a table.
    pub fn set_footer(&mut self, footer: bool) {
        self.layout.footer = footer;
    }

    /// Set column alignment
    pub fn align_columns(&mut self, position: Alignment) {
        self.layout.orientation = convert_orientation(position);
    }
}

impl Theme {
    const fn _new(
        border: TableBorders,
        lines: BorderLines,
        layout: Layout,
        colorization: Option<Colorization>,
    ) -> Self {
        Self {
            border,
            lines,
            layout,
            colorization,
        }
    }
}

impl From<Borders<char>> for Theme {
    fn from(borders: Borders<char>) -> Self {
        Self::_new(
            TableBorders::new(borders, Borders::empty()),
            BorderLines::new(None, None, None),
            Layout::new(HeadPosition::Top, false, false, false, false),
            None,
        )
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for Theme
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg_clear_borders(cfg);
        cfg_set_custom_lines(cfg, self.lines);
        cfg_set_borders(cfg, self.border);

        move_head_if(records, self.layout.orientation);

        if self.layout.reverse_column {
            reverse_head(records, self.layout.orientation);
        }

        if self.layout.reverse_rows {
            reverse_data(records, self.layout.orientation);
        }

        if self.layout.footer {
            copy_head(records, self.layout.orientation);
        }
    }
}

impl<R, D> TableOption<R, CompactConfig, D> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_borders(self.border.chars);
    }
}

impl<R, D> TableOption<R, CompactMultilineConfig, D> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_borders(self.border.chars);
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

        Self::_new(
            TableBorders::new(borders, colors),
            BorderLines::new(None, Some(horizontals), Some(verticals)),
            Layout::new(HeadPosition::Top, false, false, false, false),
            None,
        )
    }
}

impl TableBorders {
    const fn new(chars: Borders<char>, colors: Borders<Color>) -> Self {
        Self { chars, colors }
    }
}

impl BorderLines {
    const fn new(
        horizontal1: Option<HorizontalLine<char>>,
        horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
        verticals: Option<HashMap<usize, VerticalLine<char>>>,
    ) -> Self {
        Self {
            horizontal1,
            horizontals,
            verticals,
        }
    }
}

impl Layout {
    const fn new(
        orientation: HeadPosition,
        footer: bool,
        reverse_rows: bool,
        reverse_column: bool,
        move_header_on_borders: bool,
    ) -> Self {
        Self {
            orientation,
            footer,
            reverse_rows,
            reverse_column,
            move_header_on_borders,
        }
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

fn cfg_set_borders(cfg: &mut ColoredConfig, border: TableBorders) {
    cfg.set_borders(border.chars);

    if !border.colors.is_empty() {
        cfg.set_borders_color(border.colors.convert_into());
    }
}

fn cfg_set_custom_lines(cfg: &mut ColoredConfig, lines: BorderLines) {
    if let Some(line) = lines.horizontal1 {
        cfg.insert_horizontal_line(1, line);
    }

    if let Some(lines) = lines.horizontals {
        for (row, line) in lines {
            cfg.insert_horizontal_line(row, line);
        }
    }

    if let Some(lines) = lines.verticals {
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

fn reverse_data<R>(records: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    let count_rows = records.count_rows();
    let count_columns = records.count_columns();
    if count_columns < 2 || count_rows < 2 {
        return;
    }

    match orientation {
        HeadPosition::Top => reverse_rows(records, 1, count_rows),
        HeadPosition::Bottom => reverse_rows(records, 0, count_rows - 1),
        HeadPosition::Left => reverse_columns(records, 1, count_columns),
        HeadPosition::Right => reverse_columns(records, 0, count_columns - 1),
    }
}

fn reverse_head<R>(data: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    match orientation {
        HeadPosition::Top | HeadPosition::Bottom => reverse_columns(data, 0, data.count_columns()),
        HeadPosition::Left | HeadPosition::Right => reverse_rows(data, 0, data.count_rows()),
    }
}

fn reverse_rows<R>(data: &mut R, from: usize, to: usize)
where
    R: Resizable,
{
    let end = to / 2;
    let mut to = to - 1;
    for row in from..end {
        data.swap_row(row, to);
        to -= 1;
    }
}

fn reverse_columns<R>(data: &mut R, from: usize, to: usize)
where
    R: Resizable,
{
    let end = to / 2;
    let mut to = to - 1;
    for row in from..end {
        data.swap_column(row, to);
        to -= 1;
    }
}

fn copy_head<R>(records: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    let head = collect_head_by(records, orientation);
    match orientation {
        HeadPosition::Top => cp_row(records, head, records.count_rows()),
        HeadPosition::Bottom => cp_row(records, head, 0),
        HeadPosition::Left => cp_column(records, head, records.count_columns()),
        HeadPosition::Right => cp_column(records, head, 0),
    }
}

fn collect_head_by<R>(records: &mut R, orientation: HeadPosition) -> Vec<String>
where
    R: Records + PeekableRecords + ExactRecords,
{
    match orientation {
        HeadPosition::Top => collect_head(records, 0),
        HeadPosition::Bottom => collect_head(records, records.count_rows() - 1),
        HeadPosition::Left => collect_head_vertical(records, 0),
        HeadPosition::Right => collect_head_vertical(records, records.count_columns() - 1),
    }
}

fn cp_row<R>(records: &mut R, row: Vec<String>, pos: usize)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    records.insert_row(pos);

    for (col, text) in row.into_iter().enumerate() {
        records.set((pos, col), text);
    }
}

fn cp_column<R>(records: &mut R, column: Vec<String>, pos: usize)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    records.insert_column(pos);

    for (row, text) in column.into_iter().enumerate() {
        records.set((row, pos), text);
    }
}

fn move_head_if<R>(records: &mut R, orientation: HeadPosition)
where
    R: Records + Resizable + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    match orientation {
        HeadPosition::Top => {}
        HeadPosition::Bottom => {
            let head = collect_head(records, 0);
            push_row(records, head);
            records.remove_row(0);
        }
        HeadPosition::Left => {
            Rotate::Left.change(records, &mut (), &mut ());
            Rotate::Bottom.change(records, &mut (), &mut ());
        }
        HeadPosition::Right => {
            Rotate::Right.change(records, &mut (), &mut ());
        }
    }
}

fn collect_head<R>(records: &mut R, row: usize) -> Vec<String>
where
    R: Records + PeekableRecords,
{
    (0..records.count_columns())
        .map(|column| records.get_text((row, column)))
        .map(ToString::to_string)
        .collect()
}

fn collect_head_vertical<R>(records: &mut R, column: usize) -> Vec<String>
where
    R: Records + PeekableRecords + ExactRecords,
{
    (0..records.count_rows())
        .map(|row| records.get_text((row, column)))
        .map(ToString::to_string)
        .collect()
}

fn push_row<R>(records: &mut R, row: Vec<String>)
where
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    records.push_row();

    let last_row = records.count_rows() - 1;

    for (col, text) in row.into_iter().enumerate() {
        records.set((last_row, col), text);
    }
}

fn convert_orientation(position: Alignment) -> HeadPosition {
    let h = Option::from(position);
    let v = Option::from(position);

    match (h, v) {
        (None, Some(AlignmentVertical::Top)) => HeadPosition::Top,
        (None, Some(AlignmentVertical::Bottom)) => HeadPosition::Bottom,
        (Some(AlignmentHorizontal::Left), None) => HeadPosition::Left,
        (Some(AlignmentHorizontal::Right), None) => HeadPosition::Right,
        (None, Some(AlignmentVertical::Center)) => HeadPosition::Top,
        (Some(AlignmentHorizontal::Center), None) => HeadPosition::Top,
        (None, None) | (Some(_), Some(_)) => HeadPosition::Top,
    }
}
