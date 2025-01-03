//! A module which contains configuration options for a [`Grid`].
//!
//! [`Grid`]: crate::grid::iterable::Grid

mod borders_config;
mod entity_map;
mod offset;

use std::collections::HashMap;

use crate::ansi::{ANSIBuf, ANSIStr};
use crate::config::compact::CompactConfig;
use crate::config::{
    AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity, Indent, Position, Sides,
};
use borders_config::BordersConfig;

use super::{CellConfig, Formatting};

pub use self::{entity_map::EntityMap, offset::Offset};

/// HorizontalLine represents a horizontal border line.
type HorizontalLine = super::HorizontalLine<char>;

/// VerticalLine represents a vertical border line.
type VerticalLine = super::VerticalLine<char>;

// TODO: IMPROVE FORMAT::CFG add all possible args to it

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpannedConfig {
    margin: Sides<MarginIndent>,
    borders: BordersConfig<char>,
    borders_colors: BordersConfig<ANSIBuf>,
    borders_missing_char: char,
    cells: EntityMap<CellConfig>,
    spans: HashMap<Position, CellSpan>,
    chars_horizontal: HashMap<Position, HashMap<Offset, LineChar>>,
    chars_vertical: HashMap<Position, HashMap<Offset, LineChar>>,
    summary: ConfigSummary,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct ConfigSummary {
    padding_color: bool,
    border_color: bool,
    justification: bool,
    margin: bool,
    offset: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct LineChar {
    char: Option<char>,
    color: Option<ANSIBuf>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CellSpan {
    row: usize,
    col: usize,
}

impl CellSpan {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Default for SpannedConfig {
    fn default() -> Self {
        Self {
            cells: EntityMap::default(),
            margin: Sides::default(),
            spans: HashMap::default(),
            borders: BordersConfig::default(),
            borders_colors: BordersConfig::default(),
            borders_missing_char: ' ',
            chars_horizontal: HashMap::default(),
            chars_vertical: HashMap::default(),
            summary: ConfigSummary::default(),
        }
    }
}

impl SpannedConfig {
    /// Set a margin of a grid.
    pub fn set_margin(&mut self, margin: Sides<Indent>) {
        self.margin.left.indent = margin.left;
        self.margin.right.indent = margin.right;
        self.margin.top.indent = margin.top;
        self.margin.bottom.indent = margin.bottom;

        self.summary.margin = true;
    }

    /// Set a color of margin of a grid.
    pub fn set_margin_color(&mut self, margin: Sides<Option<ANSIBuf>>) {
        self.margin.left.color = margin.left;
        self.margin.right.color = margin.right;
        self.margin.top.color = margin.top;
        self.margin.bottom.color = margin.bottom;
    }

    /// Set an offset of margin of a grid.
    pub fn set_margin_offset(&mut self, margin: Sides<Offset>) {
        self.margin.left.offset = margin.left;
        self.margin.right.offset = margin.right;
        self.margin.top.offset = margin.top;
        self.margin.bottom.offset = margin.bottom;
    }

    /// Returns a margin value currently set.
    pub fn get_margin(&self) -> Sides<Indent> {
        Sides::new(
            self.margin.left.indent,
            self.margin.right.indent,
            self.margin.top.indent,
            self.margin.bottom.indent,
        )
    }

    /// Returns a margin color value currently set.
    pub fn get_margin_color(&self) -> Sides<Option<ANSIBuf>> {
        Sides::new(
            self.margin.left.color.clone(),
            self.margin.right.color.clone(),
            self.margin.top.color.clone(),
            self.margin.bottom.color.clone(),
        )
    }

    /// Returns a margin offset value currently set.
    pub fn get_margin_offset(&self) -> Sides<Offset> {
        Sides::new(
            self.margin.left.offset,
            self.margin.right.offset,
            self.margin.top.offset,
            self.margin.bottom.offset,
        )
    }

    /// Removes border changes.
    pub fn remove_borders(&mut self) {
        self.borders = BordersConfig::default();
    }

    /// Removes border changes.
    pub fn remove_borders_colors(&mut self) {
        self.borders_colors = BordersConfig::default();
    }

    /// Removes border changes.
    pub fn remove_color_line_horizontal(&mut self) {
        self.chars_horizontal.values_mut().for_each(|chars| {
            chars.values_mut().for_each(|c| {
                c.color = None;
            });
        });
    }

    /// Removes border changes.
    pub fn remove_color_line_vertical(&mut self) {
        // todo: maybe delete None elements
        self.chars_vertical.values_mut().for_each(|chars| {
            chars.values_mut().for_each(|c| {
                c.color = None;
            });
        });
    }

    /// Removes border changes.
    pub fn remove_horizontal_chars(&mut self) {
        self.chars_horizontal.values_mut().for_each(|chars| {
            chars.values_mut().for_each(|c| {
                c.char = None;
            });
        });
    }

    /// Removes border changes.
    pub fn remove_vertical_chars(&mut self) {
        self.chars_vertical.values_mut().for_each(|chars| {
            chars.values_mut().for_each(|c| {
                c.char = None;
            });
        });
    }

    /// Set the [`Borders`] value as correct one.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.borders.set_borders(borders);
    }

    /// Gets a global border value if set.
    pub fn get_border_default(&self) -> Option<&char> {
        self.borders.get_global()
    }

    /// Set the all [`Borders`] values to a char.
    pub fn set_border_default(&mut self, c: char) {
        self.borders.set_global(c);
    }

    /// Returns a current [`Borders`] structure.
    pub fn get_borders(&self) -> &Borders<char> {
        self.borders.get_borders()
    }

    /// Set the border line by row index.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn insert_horizontal_line(&mut self, line: usize, val: HorizontalLine) {
        self.borders.insert_horizontal_line(line, val);
    }

    /// Sets off the border line by row index if any were set
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn remove_horizontal_line(&mut self, line: usize, count_rows: usize) {
        self.borders.remove_horizontal_line(line, count_rows);
    }

    /// Gets a overridden vertical line.
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn get_vertical_line(&self, line: usize) -> Option<&VerticalLine> {
        self.borders.get_vertical_line(line)
    }

    /// Gets all overridden vertical lines.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_vertical_lines(&self) -> HashMap<usize, VerticalLine> {
        self.borders.get_vertical_lines()
    }

    /// Set the border line by column index.
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn insert_vertical_line(&mut self, line: usize, val: VerticalLine) {
        self.borders.insert_vertical_line(line, val);
    }

    /// Sets off the border line by column index if any were set
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn remove_vertical_line(&mut self, line: usize, count_columns: usize) {
        self.borders.remove_vertical_line(line, count_columns);
    }

    /// Gets a overridden line.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_horizontal_line(&self, line: usize) -> Option<&HorizontalLine> {
        self.borders.get_horizontal_line(line)
    }

    /// Gets all overridden lines.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_horizontal_lines(&self) -> HashMap<usize, HorizontalLine> {
        self.borders.get_horizontal_lines()
    }

    /// Override a character on a horizontal line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn set_horizontal_char(&mut self, pos: Position, c: char, offset: Offset) {
        let line_char = hm_offset_get(&mut self.chars_horizontal, pos, offset);
        line_char.char = Some(c);

        self.summary.offset = true;
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn lookup_horizontal_char(&self, pos: Position, offset: usize, end: usize) -> Option<char> {
        hm_offset_lookup(&self.chars_horizontal, pos, offset, end)?.char
    }

    /// Checks if there any char in a horizontal border being overridden.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn is_overridden_horizontal(&self, pos: Position) -> bool {
        self.chars_horizontal.contains_key(&pos)
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn remove_overridden_horizontal(&mut self, pos: Position) {
        self.chars_horizontal.remove(&pos);
    }

    /// Override a vertical split line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn set_vertical_char(&mut self, pos: Position, c: char, offset: Offset) {
        let line_char = hm_offset_get(&mut self.chars_vertical, pos, offset);
        line_char.char = Some(c);

        self.summary.offset = true;
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn lookup_vertical_char(&self, pos: Position, offset: usize, end: usize) -> Option<char> {
        hm_offset_lookup(&self.chars_vertical, pos, offset, end)?.char
    }

    /// Checks if there any char in a horizontal border being overridden.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn is_overridden_vertical(&self, pos: Position) -> bool {
        self.chars_vertical.contains_key(&pos)
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn remove_overridden_vertical(&mut self, pos: Position) {
        self.chars_vertical.remove(&pos);
    }

    /// Override a character color on a horizontal line.
    pub fn set_horizontal_color(&mut self, pos: Position, c: ANSIBuf, offset: Offset) {
        let line_char = hm_offset_get(&mut self.chars_horizontal, pos, offset);
        line_char.color = Some(c);
    }

    /// Get a overridden color in a horizontal border.
    pub fn lookup_horizontal_color(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<&ANSIBuf> {
        hm_offset_lookup(&self.chars_horizontal, pos, offset, end)?
            .color
            .as_ref()
    }

    /// Override a character color on a vertical line.
    pub fn set_vertical_color(&mut self, pos: Position, c: ANSIBuf, offset: Offset) {
        let line_char = hm_offset_get(&mut self.chars_vertical, pos, offset);
        line_char.color = Some(c);
    }

    /// Get a overridden color in a vertical border.
    pub fn lookup_vertical_color(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<&ANSIBuf> {
        hm_offset_lookup(&self.chars_vertical, pos, offset, end)?
            .color
            .as_ref()
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Sides<Indent>) {
        self.cells.modify(entity, move |c| c.padding = padding);
    }

    /// Get a padding for a given cell by [Position].
    pub fn get_padding(&self, pos: Position) -> Sides<Indent> {
        self.cells.get(pos).padding
    }

    /// Set a padding to a given cells.
    pub fn set_padding_color(&mut self, entity: Entity, padding: Sides<Option<ANSIBuf>>) {
        self.cells
            .modify(entity, move |c| c.padding_color = padding.clone());

        self.summary.padding_color = true;
    }

    /// Get a padding color for a given cell by [Position].
    pub fn get_padding_color(&self, pos: Position) -> Sides<Option<ANSIBuf>> {
        self.cells.get(pos).padding_color.clone()
    }

    /// Set a formatting to a given cells.
    pub fn set_trim_horizontal(&mut self, entity: Entity, on: bool) {
        self.cells
            .modify(entity, move |c| c.formatting.horizontal_trim = on);
    }

    /// Get a formatting settings for a given cell by [Position].
    pub fn get_trim_horizonal(&self, pos: Position) -> bool {
        self.cells.get(pos).formatting.horizontal_trim
    }

    /// Set a formatting to a given cells.
    pub fn set_trim_vertical(&mut self, entity: Entity, on: bool) {
        self.cells
            .modify(entity, move |c| c.formatting.vertical_trim = on);
    }

    /// Get a formatting settings for a given cell by [Position].
    pub fn get_trim_vertical(&self, pos: Position) -> bool {
        self.cells.get(pos).formatting.vertical_trim
    }

    /// Set a formatting to a given cells.
    pub fn set_line_alignment(&mut self, entity: Entity, on: bool) {
        self.cells
            .modify(entity, move |c| c.formatting.allow_lines_alignment = on);
    }

    /// Get a formatting settings for a given cell by [Position].
    pub fn get_line_alignment(&self, pos: Position) -> bool {
        self.cells.get(pos).formatting.allow_lines_alignment
    }

    /// Get a formatting settings for a given cell by [Position].
    pub fn get_formatting(&self, pos: Position) -> Formatting {
        self.cells.get(pos).formatting
    }

    /// Set a vertical alignment to a given cells.
    pub fn set_alignment_vertical(&mut self, entity: Entity, alignment: AlignmentVertical) {
        self.cells
            .modify(entity, move |c| c.alignment_vertical = alignment);
    }

    /// Get a vertical alignment for a given cell by [Position].
    pub fn get_alignment_vertical(&self, pos: Position) -> &AlignmentVertical {
        &self.cells.get(pos).alignment_vertical
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_horizontal(&mut self, entity: Entity, alignment: AlignmentHorizontal) {
        self.cells
            .modify(entity, move |c| c.alignment_horizontal = alignment);
    }

    /// Get a horizontal alignment for a given cell by [Position].
    pub fn get_alignment_horizontal(&self, pos: Position) -> &AlignmentHorizontal {
        &self.cells.get(pos).alignment_horizontal
    }

    /// Set border set a border value to all cells in [`Entity`].
    pub fn set_border(&mut self, pos: Position, border: Border<char>) {
        self.borders.insert_border(pos, border);
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<char> {
        self.borders.get_border(pos, shape).copied()
    }

    /// Returns a border color of a cell.
    pub fn get_border_color(&self, pos: Position, shape: (usize, usize)) -> Border<&ANSIBuf> {
        self.borders_colors.get_border(pos, shape)
    }

    /// Set a character which will be used in case any misconfiguration of borders.
    ///
    /// It will be usde for example when you set a left char for border frame and top but didn't set a top left corner.
    pub fn set_borders_missing(&mut self, c: char) {
        self.borders_missing_char = c;
    }

    /// Get a character which will be used in case any misconfiguration of borders.
    pub fn get_borders_missing(&self) -> char {
        self.borders_missing_char
    }

    /// Gets a color of all borders on the grid.
    pub fn get_border_color_default(&self) -> Option<&ANSIBuf> {
        self.borders_colors.get_global()
    }

    /// Sets a color of all borders on the grid.
    pub fn set_border_color_default(&mut self, clr: ANSIBuf) {
        self.borders_colors = BordersConfig::default();
        self.borders_colors.set_global(clr);
        self.summary.border_color = true;
    }

    /// Gets colors of a borders carcass on the grid.
    pub fn get_color_borders(&self) -> &Borders<ANSIBuf> {
        self.borders_colors.get_borders()
    }

    /// Sets colors of border carcass on the grid.
    pub fn set_borders_color(&mut self, clrs: Borders<ANSIBuf>) {
        self.borders_colors.set_borders(clrs);
        self.summary.border_color = true;
    }

    /// Sets a color of border of a cell on the grid.
    pub fn set_border_color(&mut self, pos: Position, border: Border<ANSIBuf>) {
        self.borders_colors.insert_border(pos, border);
        self.summary.border_color = true;
    }

    /// Sets off all borders possible on the [`Entity`].
    ///
    /// It doesn't changes globally set borders through [`SpannedConfig::set_borders`].
    //
    // todo: would be great to remove a shape
    pub fn remove_border(&mut self, pos: Position, shape: (usize, usize)) {
        self.borders.remove_border(pos, shape);
    }

    /// Gets a color of border of a cell on the grid.
    //
    // todo: would be great to remove a shape
    pub fn remove_border_color(&mut self, pos: Position, shape: (usize, usize)) {
        self.borders_colors.remove_border(pos, shape);
    }

    /// Get a justification which will be used while expanding cells width/height.
    pub fn get_justification(&self, pos: Position) -> char {
        self.cells.get(pos).justification
    }

    /// Set a justification which will be used while expanding cells width/height.
    pub fn set_justification(&mut self, entity: Entity, s: char) {
        self.cells.modify(entity, move |c| c.justification = s);
        self.summary.justification = true;
    }

    /// Get a justification color which will be used while expanding cells width/height.
    ///
    /// `None` means no color.
    pub fn get_justification_color(&self, pos: Position) -> Option<&ANSIBuf> {
        self.cells.get(pos).justification_color.as_ref()
    }

    /// Set a justification color which will be used while expanding cells width/height.
    ///
    /// `None` removes it.
    pub fn set_justification_color(&mut self, entity: Entity, color: Option<ANSIBuf>) {
        self.cells
            .modify(entity, move |c| c.justification_color = color.clone());
        self.summary.justification = true;
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_column_spans(&self) -> HashMap<Position, usize> {
        self.spans
            .iter()
            .map(|(pos, spans)| (*pos, spans.col))
            .collect()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_row_spans(&self) -> HashMap<Position, usize> {
        self.spans
            .iter()
            .map(|(pos, spans)| (*pos, spans.row))
            .collect()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_column_span(&self, pos: Position) -> Option<usize> {
        self.spans
            .get(&pos)
            .map(|spans| spans.col)
            .filter(|&span| span > 1)
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_row_span(&self, pos: Position) -> Option<usize> {
        self.spans
            .get(&pos)
            .map(|spans| spans.row)
            .filter(|&span| span > 1)
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_span(&self, pos: Position) -> Option<(usize, usize)> {
        self.spans.get(&pos).map(|spans| (spans.col, spans.row))
    }

    /// Removes all spans.
    pub fn remove_spans(&mut self) {
        self.spans.clear()
    }

    /// Set a column span to a given cells.
    ///
    /// BEWARE
    ///
    /// IT'S CALLER RESPONSIBILITY TO MAKE SURE
    /// THAT THERE NO INTERSECTIONS IN PLACE AND THE SPAN VALUE IS CORRECT
    pub fn set_column_span(&mut self, pos: Position, span: usize) {
        set_cell_column_span(self, pos, span);
    }

    /// Verifies if there's any spans set.
    pub fn has_spans(&self) -> bool {
        !self.spans.is_empty()
    }

    /// Verifies if there's any spans set.
    pub fn has_column_spans(&self) -> bool {
        self.spans.values().any(|spans| spans.col > 1)
    }

    /// Set a column span to a given cells.
    ///
    /// BEWARE
    ///
    /// IT'S CALLER RESPONSIBILITY TO MAKE SURE
    /// THAT THERE NO INTERSECTIONS IN PLACE AND THE SPAN VALUE IS CORRECT
    pub fn set_row_span(&mut self, pos: Position, span: usize) {
        set_cell_row_span(self, pos, span);
    }

    /// Verifies if there's any spans set.
    pub fn has_row_spans(&self) -> bool {
        self.spans.values().any(|spans| spans.row > 1)
    }

    /// Verifies if there's any colors set for a borders.
    pub fn has_border_colors(&self) -> bool {
        !self.borders_colors.is_empty()
    }

    /// Verifies if there's any colors set for a borders.
    pub fn has_offset_chars(&self) -> bool {
        !self.chars_horizontal.is_empty() || !self.chars_vertical.is_empty()
    }

    /// Verifies if there's any colors set for a borders.
    pub fn has_justification(&self) -> bool {
        self.summary.justification
    }

    /// Verifies if there's any custom padding set.
    pub fn has_padding_color(&self) -> bool {
        self.summary.padding_color
    }

    /// Gets an intersection character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_intersection(&self, pos: Position, shape: (usize, usize)) -> Option<char> {
        let c = self.borders.get_intersection(pos, shape);
        if let Some(c) = c {
            return Some(*c);
        }

        if self.has_horizontal(pos.0, shape.0) && self.has_vertical(pos.1, shape.1) {
            return Some(self.get_borders_missing());
        }

        None
    }

    /// Gets a horizontal character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<char> {
        let c = self.borders.get_horizontal(pos, count_rows);
        if let Some(c) = c {
            return Some(*c);
        }

        if self.has_horizontal(pos.0, count_rows) {
            return Some(self.get_borders_missing());
        }

        None
    }

    /// Gets a vertical character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_vertical(&self, pos: Position, count_columns: usize) -> Option<char> {
        if let Some(c) = self.borders.get_vertical(pos, count_columns) {
            return Some(*c);
        }

        if self.has_vertical(pos.1, count_columns) {
            return Some(self.get_borders_missing());
        }

        None
    }

    /// Gets a color of a cell horizontal.
    pub fn get_horizontal_color(&self, pos: Position, count_rows: usize) -> Option<&ANSIBuf> {
        self.borders_colors.get_horizontal(pos, count_rows)
    }

    /// Gets a color of a cell vertical.
    pub fn get_vertical_color(&self, pos: Position, count_columns: usize) -> Option<&ANSIBuf> {
        self.borders_colors.get_vertical(pos, count_columns)
    }

    /// Gets a color of a cell vertical.
    pub fn get_intersection_color(&self, pos: Position, shape: (usize, usize)) -> Option<&ANSIBuf> {
        self.borders_colors.get_intersection(pos, shape)
    }

    /// Checks if grid would have a horizontal border with the current configuration.
    ///
    /// grid: crate::Grid
    pub fn has_horizontal(&self, row: usize, count_rows: usize) -> bool {
        self.borders.has_horizontal(row, count_rows)
    }

    /// Checks if grid would have a vertical border with the current configuration.
    ///
    /// grid: crate::Grid
    pub fn has_vertical(&self, col: usize, count_columns: usize) -> bool {
        self.borders.has_vertical(col, count_columns)
    }

    /// Calculates an amount of horizontal lines would present on the grid.
    ///
    /// grid: crate::Grid
    pub fn count_horizontal(&self, count_rows: usize) -> usize {
        (0..=count_rows)
            .filter(|&row| self.has_horizontal(row, count_rows))
            .count()
    }

    /// Calculates an amount of vertical lines would present on the grid.
    ///
    /// grid: crate::Grid
    pub fn count_vertical(&self, count_columns: usize) -> usize {
        (0..=count_columns)
            .filter(|&col| self.has_vertical(col, count_columns))
            .count()
    }

    /// The function returns whether the cells will be rendered or it will be hidden because of a span.
    pub fn is_cell_visible(&self, pos: Position) -> bool {
        !(self.is_cell_covered_by_column_span(pos)
            || self.is_cell_covered_by_row_span(pos)
            || self.is_cell_covered_by_both_spans(pos))
    }

    /// The function checks if a cell is hidden because of a row span.
    pub fn is_cell_covered_by_row_span(&self, pos: Position) -> bool {
        is_cell_covered_by_row_span(self, pos)
    }

    /// The function checks if a cell is hidden because of a column span.
    pub fn is_cell_covered_by_column_span(&self, pos: Position) -> bool {
        is_cell_covered_by_column_span(self, pos)
    }

    /// The function checks if a cell is hidden indirectly because of a row and column span combination.
    pub fn is_cell_covered_by_both_spans(&self, pos: Position) -> bool {
        is_cell_covered_by_both_spans(self, pos)
    }

    /// Return configuration of a given cell.
    pub fn get_cell_settings(&self, pos: Position) -> &CellConfig {
        self.cells.get(pos)
    }
}

impl From<CompactConfig> for SpannedConfig {
    fn from(compact: CompactConfig) -> Self {
        use Entity::Global;

        let mut cfg = Self::default();

        cfg.set_padding(Global, *compact.get_padding());
        cfg.set_padding_color(Global, to_ansi_color(*compact.get_padding_color()));
        cfg.set_margin(*compact.get_margin());
        cfg.set_margin_color(to_ansi_color(*compact.get_margin_color()));
        cfg.set_alignment_horizontal(Global, compact.get_alignment_horizontal());
        cfg.set_borders(*compact.get_borders());
        cfg.set_borders_color(compact.get_borders_color().convert_into());

        cfg
    }
}

fn to_ansi_color(b: Sides<ANSIStr<'_>>) -> Sides<Option<ANSIBuf>> {
    Sides::new(
        Some(b.left.into()),
        Some(b.right.into()),
        Some(b.top.into()),
        Some(b.bottom.into()),
    )
}

fn set_cell_row_span(cfg: &mut SpannedConfig, pos: Position, span: usize) {
    // such spans aren't supported
    if span == 0 {
        return;
    }

    match cfg.spans.get_mut(&pos) {
        Some(spans) => {
            // It's a default span so we can do nothing.
            // but we check if it's an override of a span.
            if span == 1 && spans.col == 0 {
                cfg.spans.remove(&pos);
                return;
            }

            spans.row = span;
        }
        None => {
            // It's a default span so we can do nothing.
            // but we check if it's an override of a span.
            if span == 1 {
                return;
            }

            cfg.spans.insert(pos, CellSpan::new(span, 0));
        }
    }
}

fn set_cell_column_span(cfg: &mut SpannedConfig, pos: Position, span: usize) {
    // such spans aren't supported
    if span == 0 {
        return;
    }

    match cfg.spans.get_mut(&pos) {
        Some(spans) => {
            // It's a default span so we can do nothing.
            // but we check if it's an override of a span.
            if span == 1 && spans.row == 0 {
                cfg.spans.remove(&pos);
                return;
            }

            spans.col = span;
        }
        None => {
            // It's a default span so we can do nothing.
            // but we check if it's an override of a span.
            if span == 1 {
                return;
            }

            cfg.spans.insert(pos, CellSpan::new(0, span));
        }
    }
}

fn is_cell_covered_by_column_span(cfg: &SpannedConfig, pos: Position) -> bool {
    cfg.spans.iter().any(|(&(row, col), span)| {
        let span = span.col;
        pos.1 > col && pos.1 < col + span && row == pos.0
    })
}

fn is_cell_covered_by_row_span(cfg: &SpannedConfig, pos: Position) -> bool {
    cfg.spans.iter().any(|(&(row, col), span)| {
        let span = span.row;
        pos.0 > row && pos.0 < row + span && col == pos.1
    })
}

fn is_cell_covered_by_both_spans(cfg: &SpannedConfig, pos: Position) -> bool {
    if !cfg.has_spans() {
        return false;
    }

    let rows = cfg.get_row_spans();
    let cols = cfg.get_column_spans();

    rows.into_iter().any(|(p1, row_span)| {
        cols.iter()
            .filter(|(&p2, _)| p1 == p2)
            .any(|(_, col_span)| {
                pos.0 > p1.0 && pos.0 < p1.0 + row_span && pos.1 > p1.1 && pos.1 < p1.1 + col_span
            })
    })
}

fn hm_offset_get<T>(
    hm: &mut HashMap<Position, HashMap<Offset, T>>,
    pos: Position,
    offset: Offset,
) -> &mut T
where
    T: Default,
{
    hm.entry(pos)
        .or_insert_with(|| HashMap::with_capacity(1))
        .entry(offset)
        .or_default()
}

fn hm_offset_lookup<T>(
    hm: &HashMap<Position, HashMap<Offset, T>>,
    pos: Position,
    offset: usize,
    size: usize,
) -> Option<&T> {
    let values = hm.get(&pos)?;
    match values.get(&Offset::Begin(offset)) {
        Some(value) => Some(value),
        None => {
            if size > offset {
                let pos_from_end = size - offset - 1;
                values.get(&Offset::End(pos_from_end))
            } else {
                None
            }
        }
    }
}

/// A colorefull margin indent.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MarginIndent {
    /// An indent value.
    indent: Indent,
    /// An offset value.
    offset: Offset,
    /// An color value.
    color: Option<ANSIBuf>,
}

impl Default for MarginIndent {
    fn default() -> Self {
        Self {
            indent: Indent::default(),
            offset: Offset::Begin(0),
            color: None,
        }
    }
}
