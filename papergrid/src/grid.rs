use std::{
    borrow::Cow,
    cmp,
    collections::HashMap,
    fmt::{self, Display, Write},
};

use crate::{
    borders::BordersConfig,
    entity_map::EntityMap,
    estimation::Estimate,
    records::Records,
    util::{cut_str, string_trim, string_width},
    width::{CfgWidthFunction, WidthFunc},
    Border, Borders, Entity, Line,
};

#[cfg(feature = "color")]
use crate::{AnsiColor, Color};

const DEFAULT_BORDER_HORIZONTAL_CHAR: char = ' ';
const DEFAULT_BORDER_HORIZONTAL_SYMBOL: char = ' ';
const DEFAULT_BORDER_VERTICAL_SYMBOL: char = ' ';
const DEFAULT_SPACE_CHAR: char = ' ';

// todo: Grid is just a collection of methods with no actuall state
//       Grid::new takes size, config and records.
//       and produces WidthEstimator, HeightEstimator which are used only on fmt::Display

/// Grid provides a set of methods for building a text-based table
#[derive(Debug, Clone)]
pub struct Grid<'a, R, W, H> {
    config: &'a GridConfig,
    records: R,
    width: W,
    height: H,
}

impl<'a, R, W, H> Grid<'a, R, W, H> {
    /// The new method creates a grid instance with default styles.
    ///
    /// The size of the grid can not be changed after the instance is created.
    ///
    /// # Example
    ///
    /// ```
    /// use papergrid::{Grid, Entity, Border};
    ///
    /// let mut grid = Grid::new(vec![vec![String::from("Hello World"); 2]; 2], 2, 2);
    ///
    /// grid.set_border(
    ///     Entity::Global,
    ///     Border {
    ///         right: Some(' '),
    ///         ..Default::default()
    ///     }
    /// );
    ///
    /// assert_eq!(
    ///     grid.to_string(),
    ///     "Hello World Hello World \n\
    ///      Hello World Hello World "
    /// );
    /// ```
    ///
    /// Not empty initialization but empty content
    ///
    /// ```rust
    /// use papergrid::Grid;
    ///
    /// let mut grid = Grid::new(vec![vec![String::from(""); 2]; 2], 2, 2);
    /// assert_eq!(grid.to_string(), "\n");
    /// ```
    ///
    /// Empty
    ///
    /// ```rust
    /// use papergrid::Grid;
    ///
    /// let mut grid = Grid::new(vec![], 0, 0);
    /// assert_eq!(grid.to_string(), "");
    /// ```
    pub fn new(records: R, config: &'a GridConfig, width: W, height: H) -> Self {
        Grid {
            config,
            records,
            height,
            width,
        }
    }
}

impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
{
    /// This function returns an amount of rows on the grid
    fn count_rows(&self) -> usize {
        self.records.count_rows()
    }

    /// This function returns an amount of columns on the grid
    fn count_columns(&self) -> usize {
        self.records.count_columns()
    }

    pub fn get_vertical(&self, pos: Position) -> Option<&char> {
        self.config.get_vertical(pos, self.count_columns())
    }

    fn get_horizontal(&self, pos: Position) -> Option<&char> {
        self.config.get_horizontal(pos, self.count_rows())
    }

    fn get_intersection(&self, pos: Position) -> Option<&char> {
        self.config
            .get_intersection(pos, (self.count_rows(), self.count_columns()))
    }

    fn has_horizontal(&self, row: usize) -> bool {
        self.config.has_horizontal(row, self.count_rows())
    }
}

impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
    W: Estimate<R>,
{
    /// Returns a total width of table, including split lines.
    pub fn total_width(&self) -> usize {
        if self.count_rows() == 0 || self.count_columns() == 0 {
            return 0;
        }

        total_width(self)
    }
}

#[cfg(feature = "color")]
impl<R, W, H> Grid<'_, R, W, H>
where
    R: Records,
{
    fn get_intersection_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config
            .border_colors
            .get_intersection(pos, self.count_rows(), self.count_columns())
    }

    fn get_horizontal_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config
            .border_colors
            .get_horizontal(pos, self.count_rows())
    }

    fn get_vertical_color(&self, pos: Position) -> Option<&AnsiColor> {
        self.config
            .border_colors
            .get_vertical(pos, self.count_columns())
    }
}

#[derive(Debug, Clone)]
pub struct GridConfig {
    tab_width: usize,
    margin: Margin,
    padding: EntityMap<Padding>,
    alignment_h: EntityMap<AlignmentHorizontal>,
    alignment_v: EntityMap<AlignmentVertical>,
    formatting: EntityMap<Formatting>,
    spans: HashMap<Position, usize>,
    borders: BordersConfig<char>,
    borders_missing_char: char,
    override_split_lines: HashMap<usize, String>,
    #[cfg(feature = "color")]
    margin_color: MarginColor,
    #[cfg(feature = "color")]
    padding_color: EntityMap<PaddingColor>,
    #[cfg(feature = "color")]
    border_colors: BordersConfig<AnsiColor>,
}

pub type Position = (usize, usize);

impl GridConfig {
    /// Get a span value of the cell, if any is set.
    pub fn get_column_span(&self, (row, col): Position) -> Option<usize> {
        self.spans.get(&(row, col)).copied()
    }

    /// Verifies if there's any spans set.
    pub fn has_column_spans(&self) -> bool {
        !self.spans.is_empty()
    }

    /// Get a span value of the cell, if any is set.
    pub fn iter_column_spans(&self) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.spans.iter().map(|(&pos, &span)| (pos, span))
    }

    /// Set a [`Margin`] value.
    pub fn set_margin(&mut self, margin: Margin) {
        self.margin = margin;
    }

    /// Returns a [`Margin`] value currently set.
    pub fn get_margin(&self) -> &Margin {
        &self.margin
    }

    /// Set colors for a [`Margin`] value.
    #[cfg(feature = "color")]
    pub fn set_margin_color(&mut self, color: MarginColor) {
        self.margin_color = color;
    }

    /// Clears all theme changes.
    /// And sets it to default.
    pub fn clear_theme(&mut self) {
        self.borders = BordersConfig::default();
        self.override_split_lines.clear();
    }

    /// Set the [`Borders`] value as currect one.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.borders.set_borders(borders);
    }

    /// Set tab width in spaces.
    pub fn set_tab_width(&mut self, width: usize) {
        self.tab_width = width;
    }

    /// Get tab width value in spaces.
    pub fn get_tab_width(&self) -> usize {
        self.tab_width
    }

    /// Returns a current [`Borders`] structure.
    pub fn get_borders(&self) -> &Borders<char> {
        self.borders.get_borders()
    }

    /// Set the border line by row index.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn set_split_line(&mut self, row: usize, line: Line<char>) {
        self.borders.insert_line(row, line);
    }

    /// Sets off the border line by row index if any were set
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn remove_split_line(&mut self, row: usize) {
        self.borders.remove_line(row);
    }

    /// Gets a overriden line.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_split_line(&self, row: usize) -> Option<&Line<char>> {
        self.borders.get_line(row)
    }

    /// Override the split line with a custom text.
    ///
    /// If borders are not set the string won't be rendered.
    pub fn override_split_line(&mut self, row: usize, line: impl Into<String>) {
        self.override_split_lines.insert(row, line.into());
    }

    /// Set a column span to a given cells.
    pub fn set_span(&mut self, pos: Position, span: usize) {
        self.set_cell_span(pos, span)
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Padding) {
        self.padding.set(entity, padding);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> &Padding {
        self.padding.lookup(entity)
    }

    #[cfg(feature = "color")]
    /// Set a padding to a given cells.
    pub fn set_padding_color(&mut self, entity: Entity, color: PaddingColor) {
        self.padding_color.set(entity, color);
    }

    /// Set a formatting to a given cells.
    pub fn set_formatting(&mut self, entity: Entity, formatting: Formatting) {
        self.formatting.set(entity, formatting);
    }

    /// Get a formatting settings for a given [Entity].
    pub fn get_formatting(&self, entity: Entity) -> &Formatting {
        self.formatting.lookup(entity)
    }

    /// Set a vertical alignment to a given cells.
    pub fn set_alignment_vertical(&mut self, entity: Entity, alignment: AlignmentVertical) {
        self.alignment_v.set(entity, alignment);
    }

    /// Get a vertical alignment for a given [Entity].
    pub fn get_alignment_vertical(&self, entity: Entity) -> &AlignmentVertical {
        self.alignment_v.lookup(entity)
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_horizontal(&mut self, entity: Entity, alignment: AlignmentHorizontal) {
        self.alignment_h.set(entity, alignment);
    }

    /// Get a horizontal alignment for a given [Entity].
    pub fn get_alignment_horizontal(&self, entity: Entity) -> &AlignmentHorizontal {
        self.alignment_h.lookup(entity)
    }

    fn set_cell_span(&mut self, (row, mut col): Position, mut span: usize) {
        // It's a default span so we can do nothing.
        if span == 1 || (col == 0 && span == 0) {
            return;
        }

        if span == 0 && col > 0 {
            match closest_visible(self, (row, col - 1)) {
                Some(c) => {
                    span += 1 + col - c;
                    col = c;
                }
                None => return,
            }
        }

        self.spans.insert((row, col), span);
    }

    /// The function returns whether the cells will be rendered or it will be hidden by a cell with a span.
    pub fn is_cell_visible(&self, pos: Position) -> bool {
        let is_cell_overriden = self.is_cell_overriden(pos);
        !is_cell_overriden
    }

    fn is_cell_overriden(&self, pos: Position) -> bool {
        self.spans
            .iter()
            .any(|(&(row, col), span)| pos.1 > col && pos.1 < col + span && row == pos.0)
    }

    pub fn has_vertical(&self, col: usize, count_columns: usize) -> bool {
        self.borders.has_vertical(col, count_columns)
    }

    pub fn has_horizontal(&self, row: usize, count_rows: usize) -> bool {
        self.borders.has_horizontal(row, count_rows)
    }

    /// Set border set a border value to all cells in [`Entity`].
    pub fn set_border(&mut self, pos: Position, border: Border) {
        self.borders.insert_border(pos, border);
    }

    /// Sets off all borders possible on the [`Entity`].
    ///
    /// It doesn't changes globaly set borders through [`Grid::set_borders`].
    pub fn remove_border(&mut self, pos: Position, count_columns: usize) {
        self.borders.remove_border(pos, count_columns)
    }

    pub fn set_borders_missing(&mut self, c: char) {
        self.borders_missing_char = c;
    }

    pub fn count_vertical(&self, count_columns: usize) -> usize {
        (0..count_columns + 1)
            .filter(|&col| self.has_vertical(col, count_columns))
            .count()
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<char> {
        let mut border = self.borders.get_border(pos, shape.0, shape.1).copied();

        // make sure that there's no user defined lines
        // in which case we use spaces.

        let mut top_set = border.top.is_some();
        let mut bottom_set = border.bottom.is_some();
        let mut left_set = border.left.is_some();
        let mut right_set = border.right.is_some();

        if border.top.is_none() && self.has_horizontal(pos.0, shape.0) {
            border.top = Some(DEFAULT_BORDER_HORIZONTAL_SYMBOL);
            top_set = true;
        }

        if border.bottom.is_none() && self.has_horizontal(pos.0 + 1, shape.0) {
            border.bottom = Some(DEFAULT_BORDER_HORIZONTAL_SYMBOL);
            bottom_set = true;
        }

        if border.left.is_none() && self.has_vertical(pos.1, shape.1) {
            border.left = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
            left_set = true;
        }

        if border.right.is_none() && self.has_vertical(pos.1 + 1, shape.1) {
            border.right = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
            right_set = true;
        }

        if border.left_top_corner.is_none() && top_set && left_set {
            border.left_top_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.left_bottom_corner.is_none() && bottom_set && left_set {
            border.left_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.right_top_corner.is_none() && top_set && right_set {
            border.right_top_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        if border.right_bottom_corner.is_none() && bottom_set && right_set {
            border.right_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_SYMBOL);
        }

        border
    }

    fn get_vertical(&self, pos: Position, count_columns: usize) -> Option<&char> {
        let c = self.borders.get_vertical(pos, count_columns);
        if c.is_some() {
            return c;
        }

        if self.has_vertical(pos.1, count_columns) {
            return Some(&self.borders_missing_char);
        }

        None
    }

    fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<&char> {
        let c = self.borders.get_horizontal(pos, count_rows);
        if c.is_some() {
            return c;
        }

        if self.has_horizontal(pos.0, count_rows) {
            return Some(&self.borders_missing_char);
        }

        None
    }

    fn get_intersection(&self, pos: Position, shape: (usize, usize)) -> Option<&char> {
        let c = self.borders.get_intersection(pos, shape.0, shape.1);
        if c.is_some() {
            return c;
        }

        if self.has_horizontal(pos.0, shape.0) && self.has_vertical(pos.1, shape.1) {
            return Some(&self.borders_missing_char);
        }

        None
    }
}

#[cfg(feature = "color")]
impl GridConfig {
    pub fn set_border_color_global(&mut self, clr: AnsiColor) {
        self.border_colors = BordersConfig::default();
        self.border_colors.set_global(clr);
    }

    pub fn set_borders_color(&mut self, clrs: Borders<AnsiColor>) {
        self.border_colors.set_borders(clrs);
    }

    pub fn set_border_color(&mut self, pos: Position, border: Border<AnsiColor>) {
        self.border_colors.insert_border(pos, border)
    }

    pub fn get_color_borders(&self) -> &Borders<AnsiColor> {
        self.border_colors.get_borders()
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            tab_width: 4,
            margin: Margin::default(),
            padding: EntityMap::default(),
            formatting: EntityMap::default(),
            alignment_h: EntityMap::new(AlignmentHorizontal::Left),
            alignment_v: EntityMap::new(AlignmentVertical::Top),
            borders: BordersConfig::default(),
            borders_missing_char: ' ',
            spans: HashMap::default(),
            override_split_lines: HashMap::default(),
            #[cfg(feature = "color")]
            margin_color: MarginColor::default(),
            #[cfg(feature = "color")]
            padding_color: EntityMap::default(),
            #[cfg(feature = "color")]
            border_colors: BordersConfig::default(),
        }
    }
}

impl<'a, R, W, H> fmt::Display for Grid<'a, R, W, H>
where
    R: Records,
    W: Estimate<R>,
    H: Estimate<R>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count_rows() == 0 || self.count_columns() == 0 {
            return Ok(());
        }

        print_grid(self, f)
    }
}

/// Style represent a style of a cell on a grid.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Style {
    padding: Padding,
    alignment_horizontal: AlignmentHorizontal,
    alignment_vertical: AlignmentVertical,
    formatting: Formatting,
}

/// Formatting represent a logic of formatting of a cell.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Formatting {
    pub horizontal_trim: bool,
    pub vertical_trim: bool,
    pub allow_lines_alignement: bool,
}

impl Formatting {
    pub fn new(horizontal_trim: bool, vertical_trim: bool, allow_lines_alignement: bool) -> Self {
        Self {
            horizontal_trim,
            vertical_trim,
            allow_lines_alignement,
        }
    }
}

/// Margin represent a 4 indents of table as a whole.
pub type Margin = Sides<Indent>;

/// Padding represent a 4 indents of cell.
pub type Padding = Sides<Indent>;

#[cfg(feature = "color")]
/// Margin represent a 4 indents of table as a whole.
pub type MarginColor = Sides<AnsiColor>;

#[cfg(feature = "color")]
/// PaddingColor represent a 4 indents of a cell.
pub type PaddingColor = Sides<AnsiColor>;

/// Indent represent a filled space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indent {
    pub fill: char,
    pub size: usize,
}

impl Indent {
    /// Creates a new Indent structure.
    pub fn new(size: usize, fill: char) -> Self {
        Self { fill, size }
    }

    /// Creates a new Indent startucture with space (`' '`) as a fill character.
    pub fn spaced(size: usize) -> Self {
        Self { size, fill: ' ' }
    }
}

impl Default for Indent {
    fn default() -> Self {
        Self { size: 0, fill: ' ' }
    }
}

/// [`AlignmentHorizontal`] represents an horizontal alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentHorizontal {
    Center,
    Left,
    Right,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
}

impl<T> Sides<T> {
    pub fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

fn calculate_indent(
    alignment: AlignmentHorizontal,
    text_width: usize,
    available: usize,
) -> (usize, usize) {
    let diff = available - text_width;
    match alignment {
        AlignmentHorizontal::Left => (0, diff),
        AlignmentHorizontal::Right => (diff, 0),
        AlignmentHorizontal::Center => {
            let left = diff / 2;
            let rest = diff - left;
            (left, rest)
        }
    }
}

fn print_text(f: &mut fmt::Formatter<'_>, text: &str, tab_width: usize) -> fmt::Result {
    // So to not use replace_tab we are printing by char;
    // Hopefully it's more affective as it reduceses a number of allocations.
    for c in text.chars() {
        match c {
            '\r' => (),
            '\t' => repeat_char(f, ' ', tab_width)?,
            c => f.write_char(c)?,
        }
    }

    Ok(())
}

/// [`AlignmentVertical`] represents an vertical alignment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentVertical {
    Center,
    Top,
    Bottom,
}

fn indent_from_top(alignment: AlignmentVertical, available: usize, real: usize) -> usize {
    match alignment {
        AlignmentVertical::Top => 0,
        AlignmentVertical::Bottom => available - real,
        AlignmentVertical::Center => (available - real) / 2,
    }
}

fn print_cell_line<R, W, H, F>(
    f: &mut fmt::Formatter<'_>,
    grid: &Grid<'_, R, W, H>,
    pos: Position,
    line: usize,
    height: usize,
    width_ctrl: F,
) -> fmt::Result
where
    R: Records,
    W: Estimate<R>,
    F: WidthFunc,
{
    let width = grid_cell_width(grid, pos);
    let mut cell_height = grid.records.count_lines(pos);
    let formatting = *grid.config.get_formatting(pos.into());
    if formatting.vertical_trim {
        cell_height -= count_empty_lines_at_start(&grid.records, pos)
            + count_empty_lines_at_end(&grid.records, pos);
    }

    #[cfg(feature = "color")]
    let padding_color = grid.config.padding_color.lookup(pos.into());

    let padding = grid.config.get_padding(pos.into());
    let alignment = grid.config.get_alignment_vertical(pos.into());
    let indent = top_indent(*padding, *alignment, cell_height, height);
    if indent > line {
        return print_indent(
            f,
            padding.top.fill,
            width,
            #[cfg(feature = "color")]
            &padding_color.top,
        );
    }

    let mut index = line - indent;
    let cell_has_this_line = cell_height > index;
    if !cell_has_this_line {
        // happens when other cells have bigger height
        return print_indent(
            f,
            padding.bottom.fill,
            width,
            #[cfg(feature = "color")]
            &padding_color.bottom,
        );
    }

    if formatting.vertical_trim {
        let empty_lines = count_empty_lines_at_start(&grid.records, pos);
        index += empty_lines;

        if index > grid.records.count_lines(pos) {
            return print_indent(
                f,
                padding.top.fill,
                width,
                #[cfg(feature = "color")]
                &padding_color.top,
            );
        }
    }

    print_indent(
        f,
        padding.left.fill,
        padding.left.size,
        #[cfg(feature = "color")]
        &padding_color.left,
    )?;

    let width = width - padding.left.size - padding.right.size;
    let alignment = *grid.config.get_alignment_horizontal(pos.into());
    print_line_aligned(
        f,
        &grid.records,
        pos,
        index,
        alignment,
        formatting,
        width,
        grid.config.tab_width,
        &width_ctrl,
    )?;

    print_indent(
        f,
        padding.right.fill,
        padding.right.size,
        #[cfg(feature = "color")]
        &padding_color.right,
    )?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn print_line_aligned<R, W>(
    f: &mut fmt::Formatter<'_>,
    records: &R,
    pos: Position,
    index: usize,
    alignment: AlignmentHorizontal,
    formatting: Formatting,
    available_width: usize,
    tab_width: usize,
    width_ctrl: &W,
) -> Result<(), fmt::Error>
where
    R: Records,
    W: WidthFunc,
{
    let line = records.get_line(pos, index);
    let (line, line_width) = if formatting.horizontal_trim && !line.is_empty() {
        let line = string_trim(line);
        let width = width_ctrl.width(&line);
        (line, width)
    } else {
        let line = Cow::Borrowed(line);
        let width = records.get_line_width(pos, index, width_ctrl);
        (line, width)
    };

    if formatting.allow_lines_alignement {
        let (left, right) = calculate_indent(alignment, line_width, available_width);
        return print_text_formated(f, records, pos, &line, tab_width, left, right);
    }

    let cell_width = if formatting.horizontal_trim {
        (0..records.count_lines(pos))
            .map(|i| records.get_line(pos, i))
            .map(|line| width_ctrl.width(line.trim()))
            .max()
            .unwrap_or(0)
    } else {
        records.get_width(pos, width_ctrl)
    };

    let (left, right) = calculate_indent(alignment, cell_width, available_width);
    print_text_formated(f, records, pos, &line, tab_width, left, right)?;

    // do we need line_width here?
    let rest_width = cell_width - line_width;
    repeat_char(f, DEFAULT_SPACE_CHAR, rest_width)?;

    Ok(())
}

fn print_text_formated<R>(
    f: &mut fmt::Formatter<'_>,
    records: &R,
    pos: Position,
    text: &str,
    tab_width: usize,
    left: usize,
    right: usize,
) -> fmt::Result
where
    R: Records,
{
    repeat_char(f, DEFAULT_SPACE_CHAR, left)?;

    #[cfg(feature = "color")]
    records.fmt_text_prefix(f, pos)?;

    print_text(f, text, tab_width)?;

    #[cfg(feature = "color")]
    records.fmt_text_suffix(f, pos)?;

    repeat_char(f, DEFAULT_SPACE_CHAR, right)?;

    Ok(())
}

fn top_indent(
    padding: Padding,
    alignment: AlignmentVertical,
    cell_height: usize,
    height: usize,
) -> usize {
    let height = height - padding.top.size;
    let indent = indent_from_top(alignment, height, cell_height);

    indent + padding.top.size
}

fn count_empty_lines_at_end<R>(records: R, pos: Position) -> usize
where
    R: Records,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .rev()
        .take_while(|l| l.trim().is_empty())
        .count()
}

fn count_empty_lines_at_start<R>(records: R, pos: Position) -> usize
where
    R: Records,
{
    (0..records.count_lines(pos))
        .map(|i| records.get_line(pos, i))
        .take_while(|s| s.trim().is_empty())
        .count()
}

fn repeat_char(f: &mut fmt::Formatter<'_>, c: char, n: usize) -> fmt::Result {
    for _ in 0..n {
        c.fmt(f)?;
    }

    Ok(())
}

fn closest_visible(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos) {
            return Some(pos.1);
        }

        if pos.1 == 0 {
            return None;
        }

        pos.1 -= 1;
    }
}

// only valid to call for stabilized widths.
fn total_width<R, W, H>(grid: &Grid<'_, R, W, H>) -> usize
where
    W: Estimate<R>,
    R: Records,
{
    let content_width = grid.width.total();
    let count_borders = grid.config.count_vertical(grid.count_columns());

    content_width + count_borders + grid.config.margin.left.size + grid.config.margin.right.size
}

fn print_grid<R, W, H>(grid: &Grid<'_, R, W, H>, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    W: Estimate<R>,
    H: Estimate<R>,
    R: Records,
{
    let width_ctrl = CfgWidthFunction::from_cfg(grid.config);
    let total_width = grid.total_width();

    if grid.config.margin.top.size > 0 {
        print_margin_top(grid, total_width, f)?;
        f.write_char('\n')?;
    }

    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.count_rows() {
        if grid.has_horizontal(row) {
            print_margin_left(grid, f)?;
            print_split_line(grid, total_width, row, f)?;
            print_margin_right(grid, f)?;
            f.write_char('\n')?;
        }

        let height = grid.height.get(row).unwrap();

        let is_last_row = row + 1 == grid.count_rows();

        for i in 0..height {
            print_margin_left(grid, f)?;

            for col in 0..grid.count_columns() {
                if grid.config.is_cell_visible((row, col)) {
                    print_vertical_char(grid, (row, col), f)?;
                    print_cell_line(f, grid, (row, col), i, height, &width_ctrl)?;
                }

                let is_last_column = col + 1 == grid.count_columns();
                if is_last_column {
                    print_vertical_char(grid, (row, col + 1), f)?;
                }
            }

            print_margin_right(grid, f)?;

            let is_last_line = i + 1 == height;
            if !(is_last_line && is_last_row) {
                f.write_char('\n')?;
            }
        }
    }

    if grid.has_horizontal(grid.count_rows()) {
        f.write_char('\n')?;
        print_margin_left(grid, f)?;
        print_split_line(grid, total_width, grid.count_rows(), f)?;
        print_margin_right(grid, f)?;
    }

    if grid.config.margin.bottom.size > 0 {
        f.write_char('\n')?;
        print_margin_bottom(grid, total_width, f)?;
    }

    Ok(())
}

fn print_vertical_char<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    pos: Position,
    f: &mut fmt::Formatter<'_>,
) -> Result<(), fmt::Error>
where
    R: Records,
{
    let left = grid.get_vertical(pos);
    if let Some(c) = left {
        #[cfg(feature = "color")]
        write_colored(f, c, grid.get_vertical_color(pos))?;

        #[cfg(not(feature = "color"))]
        c.fmt(f)?;
    }

    Ok(())
}

fn print_margin_top<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    width: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent_lines(
        f,
        &grid.config.margin.top,
        width,
        #[cfg(feature = "color")]
        &grid.config.margin_color.top,
    )
}

fn print_margin_bottom<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    width: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent_lines(
        f,
        &grid.config.margin.bottom,
        width,
        #[cfg(feature = "color")]
        &grid.config.margin_color.bottom,
    )
}

fn print_margin_left<R, W, H>(grid: &Grid<'_, R, W, H>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    print_indent(
        f,
        grid.config.margin.left.fill,
        grid.config.margin.left.size,
        #[cfg(feature = "color")]
        &grid.config.margin_color.left,
    )
}

fn print_margin_right<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    print_indent(
        f,
        grid.config.margin.right.fill,
        grid.config.margin.right.size,
        #[cfg(feature = "color")]
        &grid.config.margin_color.right,
    )
}

fn print_indent_lines(
    f: &mut fmt::Formatter<'_>,
    indent: &Indent,
    width: usize,
    #[cfg(feature = "color")] color: &AnsiColor,
) -> fmt::Result {
    for i in 0..indent.size {
        print_indent(
            f,
            indent.fill,
            width,
            #[cfg(feature = "color")]
            color,
        )?;

        if i + 1 != indent.size {
            f.write_char('\n')?;
        }
    }

    Ok(())
}

fn print_indent(
    f: &mut fmt::Formatter<'_>,
    c: char,
    n: usize,
    #[cfg(feature = "color")] color: &AnsiColor,
) -> fmt::Result {
    #[cfg(feature = "color")]
    color.fmt_prefix(f)?;
    repeat_char(f, c, n)?;
    #[cfg(feature = "color")]
    color.fmt_suffix(f)?;

    Ok(())
}

fn grid_cell_width<R, W, H>(grid: &Grid<'_, R, W, H>, pos: Position) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    let span = grid.config.get_column_span(pos);
    match span {
        Some(span) => range_width(grid, pos.1, pos.1 + span),
        None => grid.width.get(pos.1).unwrap(),
    }
}

fn range_width<R, W, H>(grid: &Grid<'_, R, W, H>, start: usize, end: usize) -> usize
where
    R: Records,
    W: Estimate<R>,
{
    let count_borders =
        count_borders_in_range(grid.config, start, end, grid.records.count_columns());
    let range_width = (start..end)
        .map(|col| grid.width.get(col).unwrap())
        .sum::<usize>();
    count_borders + range_width
}

fn count_borders_in_range(
    cfg: &GridConfig,
    start: usize,
    end: usize,
    count_columns: usize,
) -> usize {
    (start..end)
        .skip(1)
        .filter(|&i| cfg.has_vertical(i, count_columns))
        .count()
}

fn print_split_line<R, W, H>(
    grid: &Grid<'_, R, W, H>,
    total_width: usize,
    row: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    W: Estimate<R>,
    R: Records,
{
    let mut char_skip = 0;
    let override_text = grid.config.override_split_lines.get(&row);
    if let Some(text) = override_text {
        if !text.is_empty() {
            let text = cut_str(text, total_width);
            let line = text.lines().next().unwrap();
            char_skip = string_width(line);
            f.write_str(line)?;
        }
    }

    #[cfg(feature = "color")]
    let mut used_color = None;

    for col in 0..grid.count_columns() {
        if col == 0 {
            let left = grid.get_intersection((row, col));
            if let Some(c) = left {
                if char_skip == 0 {
                    #[cfg(feature = "color")]
                    {
                        if let Some(clr) = grid.get_intersection_color((row, col)) {
                            clr.fmt_prefix(f)?;
                            used_color = Some(clr);
                        }
                    }

                    c.fmt(f)?;
                } else {
                    char_skip -= 1;
                }
            }
        }

        let mut width = grid.width.get(col).unwrap();
        if char_skip > 0 {
            let sub = cmp::min(width, char_skip);
            width -= sub;
            char_skip -= sub;
        }

        let main = grid.get_horizontal((row, col));
        match main {
            Some(c) => {
                #[cfg(feature = "color")]
                {
                    prepare_coloring(f, grid.get_horizontal_color((row, col)), &mut used_color)?;
                }

                repeat_char(f, *c, width)?;
            }
            None => repeat_char(f, DEFAULT_BORDER_HORIZONTAL_CHAR, width)?,
        }

        let right = grid.get_intersection((row, col + 1));
        if let Some(c) = right {
            if char_skip == 0 {
                #[cfg(feature = "color")]
                {
                    prepare_coloring(
                        f,
                        grid.get_intersection_color((row, col + 1)),
                        &mut used_color,
                    )?;
                }

                c.fmt(f)?;
            } else {
                char_skip -= 1;
            }
        }
    }

    #[cfg(feature = "color")]
    if let Some(clr) = used_color.take() {
        clr.fmt_suffix(f)?;
    }

    Ok(())
}

#[cfg(feature = "color")]
fn prepare_coloring<'a>(
    f: &mut fmt::Formatter<'_>,
    clr: Option<&'a AnsiColor>,
    used_color: &mut Option<&'a AnsiColor>,
) -> fmt::Result {
    match clr {
        Some(clr) => match used_color.as_mut() {
            Some(used_clr) => {
                if **used_clr != *clr {
                    used_clr.fmt_suffix(f)?;
                    clr.fmt_prefix(f)?;
                    *used_clr = clr;
                }
            }
            None => {
                clr.fmt_prefix(f)?;
                *used_color = Some(clr);
            }
        },
        None => match used_color.take() {
            Some(clr) => clr.fmt_suffix(f)?,
            None => (),
        },
    }

    Ok(())
}

#[cfg(feature = "color")]
fn write_colored(
    f: &mut fmt::Formatter<'_>,
    c: impl fmt::Display,
    clr: Option<&AnsiColor>,
) -> fmt::Result {
    if let Some(clr) = &clr {
        clr.fmt_prefix(f)?;
        c.fmt(f)?;
        clr.fmt_suffix(f)?;
    } else {
        c.fmt(f)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn horizontal_aligment_test() {
    //     use std::fmt;

    //     struct F<'a>(&'a str, AlignmentHorizontal, usize);

    //     impl fmt::Display for F<'_> {
    //         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //             let width = string_width(self.0);
    //             print_text_formated(f, &EmptyRecords::default(), (0, 0), self.0, 4, self.1, self.2, 0)
    //             Ok(())
    //         }
    //     }

    //     assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
    //     assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
    //     assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
    //     assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
    //     assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");

    //     #[cfg(feature = "color")]
    //     {
    //         use owo_colors::OwoColorize;
    //         let text = "Colored Text".red().to_string();
    //         assert_eq!(
    //             F(&text, AlignmentHorizontal::Center, 15).to_string(),
    //             format!(" {}  ", text)
    //         );
    //     }
    // }

    #[test]
    fn vertical_aligment_test() {
        use AlignmentVertical::*;

        assert_eq!(indent_from_top(Bottom, 1, 1), 0);
        assert_eq!(indent_from_top(Top, 1, 1), 0);
        assert_eq!(indent_from_top(Center, 1, 1), 0);
        assert_eq!(indent_from_top(Bottom, 3, 1), 2);
        assert_eq!(indent_from_top(Top, 3, 1), 0);
        assert_eq!(indent_from_top(Center, 3, 1), 1);
        assert_eq!(indent_from_top(Center, 4, 1), 1);
    }
}
