//! A module which contains [GridConfig] which is responsible for grid configuration.

mod alignment;
mod border;
mod borders;
mod entity;
mod entity_map;
mod formatting;
mod offset;
mod sides;

use std::collections::HashMap;

use self::borders::BordersConfig;
use crate::color::AnsiColor;

pub use self::{
    alignment::{AlignmentHorizontal, AlignmentVertical},
    border::Border,
    borders::{Borders, HorizontalLine, VerticalLine},
    entity::{Entity, EntityIterator, Position},
    entity_map::EntityMap,
    formatting::Formatting,
    offset::Offset,
    sides::{Indent, Sides},
};

/// Margin represent a 4 indents of table as a whole.
pub type Margin = Sides<Indent>;

/// Padding represent a 4 indents of cell.
pub type Padding = Sides<Indent>;

/// Margin represent a 4 indents of table as a whole.
pub type MarginColor<'a> = Sides<AnsiColor<'a>>;

/// PaddingColor represent a 4 indents of a cell.
pub type PaddingColor<'a> = Sides<AnsiColor<'a>>;

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, Clone)]
pub struct GridConfig {
    tab_width: usize,
    margin: Margin,
    margin_offset: Sides<Offset>,
    padding: EntityMap<Padding>,
    alignment_h: EntityMap<AlignmentHorizontal>,
    alignment_v: EntityMap<AlignmentVertical>,
    formatting: EntityMap<Formatting>,
    span_columns: HashMap<Position, usize>,
    span_rows: HashMap<Position, usize>,
    borders: BordersConfig<char>,
    borders_missing_char: char,
    override_horizontal_lines: HashMap<usize, (String, Offset)>,
    override_horizontal_borders: HashMap<Position, HashMap<Offset, char>>,
    override_vertical_borders: HashMap<Position, HashMap<Offset, char>>,
    margin_color: MarginColor<'static>,
    padding_color: EntityMap<PaddingColor<'static>>,
    border_colors: BordersConfig<AnsiColor<'static>>,
}

impl Default for GridConfig {
    fn default() -> Self {
        let margin_offset = Sides::new(
            Offset::Begin(0),
            Offset::Begin(0),
            Offset::Begin(0),
            Offset::Begin(0),
        );

        Self {
            tab_width: 4,
            margin: Margin::default(),
            margin_offset,
            padding: EntityMap::default(),
            formatting: EntityMap::default(),
            alignment_h: EntityMap::new(AlignmentHorizontal::Left),
            alignment_v: EntityMap::new(AlignmentVertical::Top),
            borders: BordersConfig::default(),
            borders_missing_char: ' ',
            span_columns: HashMap::default(),
            span_rows: HashMap::default(),
            override_horizontal_lines: HashMap::default(),
            override_horizontal_borders: HashMap::default(),
            override_vertical_borders: HashMap::default(),
            margin_color: MarginColor::default(),
            padding_color: EntityMap::default(),
            border_colors: BordersConfig::default(),
        }
    }
}

impl GridConfig {
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
    pub fn has_column_spans(&self) -> bool {
        !self.span_columns.is_empty()
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
        !self.span_rows.is_empty()
    }

    /// Set a [`Margin`] value.
    pub fn set_margin(&mut self, margin: Margin) {
        self.margin = margin;
    }

    /// Returns a [`Margin`] value currently set.
    pub fn get_margin(&self) -> &Margin {
        &self.margin
    }

    /// Set [`Margin`] offset.
    pub fn set_margin_offset(&mut self, margin: Sides<Offset>) {
        self.margin_offset = margin;
    }

    /// Returns a [`Margin`] offset.
    pub fn get_margin_offset(&self) -> &Sides<Offset> {
        &self.margin_offset
    }

    /// Clears all theme changes.
    /// And sets it to default.
    pub fn clear_theme(&mut self) {
        self.borders = BordersConfig::default();
        self.override_horizontal_lines.clear();
        self.override_horizontal_borders.clear();
        self.override_vertical_borders.clear();
    }

    /// Set the [`Borders`] value as currect one.
    pub fn set_borders(&mut self, borders: Borders<char>) {
        self.borders.set_borders(borders);
    }

    /// Gets a global border value if set.
    pub fn get_global_border(&self) -> Option<&char> {
        self.borders.get_global()
    }

    /// Set the all [`Borders`] values to a char.
    pub fn set_global_border(&mut self, c: char) {
        self.borders.set_global(c);
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
    pub fn set_horizontal_line(&mut self, line: usize, val: HorizontalLine<char>) {
        self.borders.insert_horizontal_line(line, val);
    }

    /// Sets off the border line by row index if any were set
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn remove_horizontal_line(&mut self, line: usize) {
        self.borders.remove_horizontal_line(line);
    }

    /// Gets a overridden vertical line.
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn get_vertical_line(&self, line: usize) -> Option<&VerticalLine<char>> {
        self.borders.get_vertical_line(line)
    }

    /// Set the border line by column index.
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn set_vertical_line(&mut self, line: usize, val: VerticalLine<char>) {
        self.borders.insert_vertical_line(line, val);
    }

    /// Sets off the border line by column index if any were set
    ///
    /// Row `0` means the left row.
    /// Row `grid.count_columns()` means the right most row.
    pub fn remove_vertical_line(&mut self, line: usize) {
        self.borders.remove_vertical_line(line);
    }

    /// Gets a overridden line.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_horizontal_line(&self, line: usize) -> Option<&HorizontalLine<char>> {
        self.borders.get_horizontal_line(line)
    }

    /// Override the split line with a custom text.
    ///
    /// If borders are not set the string won't be rendered.
    pub fn override_split_line(&mut self, line: usize, text: impl Into<String>, offset: Offset) {
        self.override_horizontal_lines
            .insert(line, (text.into(), offset));
    }

    /// Gets a set text to a border line by index
    pub fn get_split_line_text(&self, line: usize) -> Option<&str> {
        self.override_horizontal_lines
            .get(&line)
            .map(|(s, _)| s.as_str())
    }

    /// Gets a set text to a border line by index
    pub fn get_split_line_offset(&self, line: usize) -> Option<Offset> {
        self.override_horizontal_lines
            .get(&line)
            .map(|(_, offset)| offset)
            .copied()
    }

    /// Removes a split line text if any set.
    pub fn remove_split_line_text(&mut self, line: usize) -> Option<(String, Offset)> {
        self.override_horizontal_lines.remove(&line)
    }

    /// Override a character on a horizontal line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn override_horizontal_border(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .override_horizontal_borders
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn lookup_overridden_horizontal(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<char> {
        self.override_horizontal_borders
            .get(&pos)
            .and_then(|chars| {
                chars.get(&Offset::Begin(offset)).or_else(|| {
                    if end > offset {
                        if end == 0 {
                            chars.get(&Offset::End(0))
                        } else {
                            chars.get(&Offset::End(end - offset - 1))
                        }
                    } else {
                        None
                    }
                })
            })
            .copied()
    }

    /// Checks if there any char in a horizontal border being overridden.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn is_overridden_horizontal(&self, pos: Position) -> bool {
        self.override_horizontal_borders.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn remove_overridden_horizontal(&mut self, pos: Position) {
        self.override_horizontal_borders.remove(&pos);
    }

    /// Override a vertical split line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn override_vertical_border(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .override_vertical_borders
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn lookup_overridden_vertical(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<char> {
        self.override_vertical_borders
            .get(&pos)
            .and_then(|chars| {
                chars.get(&Offset::Begin(offset)).or_else(|| {
                    if end > offset {
                        if end == 0 {
                            chars.get(&Offset::End(0))
                        } else {
                            chars.get(&Offset::End(end - offset - 1))
                        }
                    } else {
                        None
                    }
                })
            })
            .copied()
    }

    /// Checks if there any char in a horizontal border being overridden.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn is_overridden_vertical(&self, pos: Position) -> bool {
        self.override_vertical_borders.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn remove_overridden_vertical(&mut self, pos: Position) {
        self.override_vertical_borders.remove(&pos);
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Padding) {
        self.padding.insert(entity, padding);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> &Padding {
        self.padding.get(entity)
    }

    /// Set a formatting to a given cells.
    pub fn set_formatting(&mut self, entity: Entity, formatting: Formatting) {
        self.formatting.insert(entity, formatting);
    }

    /// Get a formatting settings for a given [Entity].
    pub fn get_formatting(&self, entity: Entity) -> &Formatting {
        self.formatting.get(entity)
    }

    /// Set a vertical alignment to a given cells.
    pub fn set_alignment_vertical(&mut self, entity: Entity, alignment: AlignmentVertical) {
        self.alignment_v.insert(entity, alignment);
    }

    /// Get a vertical alignment for a given [Entity].
    pub fn get_alignment_vertical(&self, entity: Entity) -> &AlignmentVertical {
        self.alignment_v.get(entity)
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_horizontal(&mut self, entity: Entity, alignment: AlignmentHorizontal) {
        self.alignment_h.insert(entity, alignment);
    }

    /// Get a horizontal alignment for a given [Entity].
    pub fn get_alignment_horizontal(&self, entity: Entity) -> &AlignmentHorizontal {
        self.alignment_h.get(entity)
    }

    /// Set border set a border value to all cells in [`Entity`].
    pub fn set_border(&mut self, pos: Position, border: Border) {
        self.borders.insert_border(pos, border);
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
    pub fn get_border_color_global(&self) -> Option<&AnsiColor<'_>> {
        self.border_colors.get_global()
    }

    /// Sets a color of all borders on the grid.
    pub fn set_border_color_global(&mut self, clr: AnsiColor<'static>) {
        self.border_colors = BordersConfig::default();
        self.border_colors.set_global(clr);
    }

    /// Gets colors of a borders carcass on the grid.
    pub fn get_color_borders(&self) -> &Borders<AnsiColor<'_>> {
        self.border_colors.get_borders()
    }

    /// Sets colors of border carcass on the grid.
    pub fn set_borders_color(&mut self, clrs: Borders<AnsiColor<'static>>) {
        self.border_colors.set_borders(clrs);
    }

    /// Sets a color of border of a cell on the grid.
    pub fn set_border_color(&mut self, pos: Position, border: Border<AnsiColor<'static>>) {
        self.border_colors.insert_border(pos, border)
    }

    /// Get colors for a [`Margin`] value.
    pub fn get_margin_color(&self) -> &MarginColor<'_> {
        &self.margin_color
    }

    /// Set colors for a [`Margin`] value.
    pub fn set_margin_color(&mut self, color: MarginColor<'static>) {
        self.margin_color = color;
    }

    /// Get a padding to a given cells.
    pub fn get_padding_color(&self, entity: Entity) -> &PaddingColor<'static> {
        self.padding_color.get(entity)
    }

    /// Set a padding to a given cells.
    pub fn set_padding_color(&mut self, entity: Entity, color: PaddingColor<'static>) {
        self.padding_color.insert(entity, color);
    }

    /// Get a span value of the cell, if any is set.
    pub fn iter_span_rows(&self) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.span_rows.iter().map(|(&pos, &span)| (pos, span))
    }

    /// Get a span value of the cell, if any is set.
    pub fn iter_span_columns(&self) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.span_columns.iter().map(|(&pos, &span)| (pos, span))
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_span_column(&self, pos: Position) -> Option<usize> {
        self.span_columns.get(&pos).copied()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_span_row(&self, pos: Position) -> Option<usize> {
        self.span_rows.get(&pos).copied()
    }

    /// Removes column spans.
    pub fn clear_span_column(&mut self) {
        self.span_columns.clear()
    }

    /// Removes row spans.
    pub fn clear_span_row(&mut self) {
        self.span_rows.clear()
    }

    /// Sets off all borders possible on the [`Entity`].
    ///
    /// It doesn't changes globally set borders through [`GridConfig::set_borders`].
    //
    // todo: would be great to remove a shape
    pub fn remove_border(&mut self, pos: Position, shape: (usize, usize)) {
        self.get_border_config_mut().remove_border(pos, shape);
    }

    /// Gets a color of border of a cell on the grid.
    //
    // todo: would be great to remove a shape
    pub fn remove_border_color(&mut self, pos: Position, shape: (usize, usize)) {
        self.get_border_color_config_mut().remove_border(pos, shape);
    }

    pub(crate) fn get_border_config(&self) -> &BordersConfig<char> {
        &self.borders
    }

    pub(crate) fn get_border_config_mut(&mut self) -> &mut BordersConfig<char> {
        &mut self.borders
    }

    pub(crate) fn get_border_color_config(&self) -> &BordersConfig<AnsiColor<'static>> {
        &self.border_colors
    }

    pub(crate) fn get_border_color_config_mut(&mut self) -> &mut BordersConfig<AnsiColor<'static>> {
        &mut self.border_colors
    }
}

impl From<&GridConfig> for GridConfig {
    fn from(value: &GridConfig) -> Self {
        value.clone()
    }
}

fn set_cell_row_span(cfg: &mut GridConfig, pos: Position, span: usize) {
    // such spans aren't supported
    if span == 0 {
        return;
    }

    // It's a default span so we can do nothing.
    // but we check if it's an override of a span.
    if span == 1 {
        cfg.span_rows.remove(&pos);
        return;
    }

    cfg.span_rows.insert(pos, span);
}

fn set_cell_column_span(cfg: &mut GridConfig, pos: Position, span: usize) {
    // such spans aren't supported
    if span == 0 {
        return;
    }

    // It's a default span so we can do nothing.
    // but we check if it's an override of a span.
    if span == 1 {
        cfg.span_columns.remove(&pos);
        return;
    }

    cfg.span_columns.insert(pos, span);
}
