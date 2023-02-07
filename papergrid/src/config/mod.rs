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
    pub fn set_column_span(&mut self, pos: Position, span: usize) {
        set_cell_column_span(self, pos, span);
    }

    /// Verifies if there's any spans set.
    pub fn has_column_spans(&self) -> bool {
        !self.span_columns.is_empty()
    }

    /// Set a column span to a given cells.
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
    pub fn set_horizontal_line(&mut self, row: usize, line: HorizontalLine<char>) {
        self.borders.insert_horizontal_line(row, line);
    }

    /// Sets off the border line by row index if any were set
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn remove_horizontal_line(&mut self, row: usize) {
        self.borders.remove_horizontal_line(row);
    }

    /// Gets a overridden vertical line.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_vertical_line(&self, row: usize) -> Option<&VerticalLine<char>> {
        self.borders.get_vertical_line(row)
    }

    /// Set the border line by column index.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn set_vertical_line(&mut self, row: usize, line: VerticalLine<char>) {
        self.borders.insert_vertical_line(row, line);
    }

    /// Sets off the border line by row index if any were set
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn remove_vertical_line(&mut self, row: usize) {
        self.borders.remove_vertical_line(row);
    }

    /// Gets a overridden line.
    ///
    /// Row `0` means the top row.
    /// Row `grid.count_rows()` means the bottom row.
    pub fn get_horizontal_line(&self, row: usize) -> Option<&HorizontalLine<char>> {
        self.borders.get_horizontal_line(row)
    }

    /// Override the split line with a custom text.
    ///
    /// If borders are not set the string won't be rendered.
    pub fn override_split_line(&mut self, row: usize, line: impl Into<String>, offset: Offset) {
        self.override_horizontal_lines
            .insert(row, (line.into(), offset));
    }

    /// Gets a set text to a border line by index
    pub fn get_split_line_text(&self, row: usize) -> Option<&str> {
        self.override_horizontal_lines
            .get(&row)
            .map(|(s, _)| s.as_str())
    }

    /// Gets a set text to a border line by index
    pub fn get_split_line_offset(&self, row: usize) -> Option<Offset> {
        self.override_horizontal_lines
            .get(&row)
            .map(|(_, offset)| offset)
            .copied()
    }

    /// Removes a split line text if any set.
    pub fn remove_split_line_text(&mut self, row: usize) -> Option<(String, Offset)> {
        self.override_horizontal_lines.remove(&row)
    }

    /// Override a character on a horizontal line.
    ///
    /// If borders are not set the char won't be used.
    pub fn override_horizontal_border(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .override_horizontal_borders
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
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
    pub fn is_overridden_horizontal(&self, pos: Position) -> bool {
        self.override_horizontal_borders.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    pub fn remove_overridden_horizontal(&mut self, pos: Position) {
        self.override_horizontal_borders.remove(&pos);
    }

    /// Override a vertical split line.
    ///
    /// If borders are not set the char won't be used.
    pub fn override_vertical_border(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .override_vertical_borders
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
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
    pub fn is_overridden_vertical(&self, pos: Position) -> bool {
        self.override_vertical_borders.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    pub fn remove_overridden_vertical(&mut self, pos: Position) {
        self.override_vertical_borders.remove(&pos);
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Padding) {
        self.padding.set(entity, padding);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> &Padding {
        self.padding.get(entity)
    }

    /// Set a formatting to a given cells.
    pub fn set_formatting(&mut self, entity: Entity, formatting: Formatting) {
        self.formatting.set(entity, formatting);
    }

    /// Get a formatting settings for a given [Entity].
    pub fn get_formatting(&self, entity: Entity) -> &Formatting {
        self.formatting.get(entity)
    }

    /// Set a vertical alignment to a given cells.
    pub fn set_alignment_vertical(&mut self, entity: Entity, alignment: AlignmentVertical) {
        self.alignment_v.set(entity, alignment);
    }

    /// Get a vertical alignment for a given [Entity].
    pub fn get_alignment_vertical(&self, entity: Entity) -> &AlignmentVertical {
        self.alignment_v.get(entity)
    }

    /// Set a horizontal alignment to a given cells.
    pub fn set_alignment_horizontal(&mut self, entity: Entity, alignment: AlignmentHorizontal) {
        self.alignment_h.set(entity, alignment);
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
        self.padding_color.set(entity, color);
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

    pub fn clear_span_column(&mut self) {
        self.span_columns.clear()
    }

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

fn set_cell_row_span(cfg: &mut GridConfig, (mut row, col): Position, mut span: usize) {
    // such spans aren't supported
    if row == 0 && span == 0 {
        return;
    }

    // It's a default span so we can do nothing.
    // but we check if it's an override of a span.
    if span == 1 {
        cfg.span_rows.remove(&(row, col));
        return;
    }

    if span == 0 && row > 0 {
        match closest_visible_row(cfg, (row - 1, col)) {
            Some(c) => {
                span += 1 + row - c;
                row = c;
            }
            None => return,
        }
    }

    cfg.span_rows.insert((row, col), span);
}

fn closest_visible_row(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
    loop {
        if is_cell_visible(cfg, pos) {
            return Some(pos.0);
        }

        if pos.0 == 0 {
            return None;
        }

        pos.0 -= 1;
    }
}

fn closest_visible_column(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
    loop {
        if is_cell_visible(cfg, pos) {
            return Some(pos.1);
        }

        if pos.1 == 0 {
            return None;
        }

        pos.1 -= 1;
    }
}

fn set_cell_column_span(cfg: &mut GridConfig, (row, mut col): Position, mut span: usize) {
    // such spans aren't supported
    if col == 0 && span == 0 {
        return;
    }

    // It's a default span so we can do nothing.
    // but we check if it's an override of a span.
    if span == 1 {
        cfg.span_columns.remove(&(row, col));
        return;
    }

    if span == 0 && col > 0 {
        match closest_visible_column(cfg, (row, col - 1)) {
            Some(c) => {
                span += 1 + col - c;
                col = c;
            }
            None => return,
        }
    }

    cfg.span_columns.insert((row, col), span);
}

fn is_cell_visible(cfg: &GridConfig, pos: Position) -> bool {
    !(is_cell_covered_by_column_span(cfg, pos)
        || is_cell_covered_by_row_span(cfg, pos)
        || is_cell_covered_by_both_spans(cfg, pos))
}

fn is_cell_covered_by_column_span(cfg: &GridConfig, pos: Position) -> bool {
    if cfg.span_columns.is_empty() {
        return false;
    }

    cfg.span_columns
        .iter()
        .any(|(&(row, col), span)| pos.1 > col && pos.1 < col + span && row == pos.0)
}

fn is_cell_covered_by_row_span(cfg: &GridConfig, pos: Position) -> bool {
    if cfg.span_rows.is_empty() {
        return false;
    }

    cfg.span_rows
        .iter()
        .any(|(&(row, col), span)| pos.0 > row && pos.0 < row + span && col == pos.1)
}

fn is_cell_covered_by_both_spans(cfg: &GridConfig, pos: Position) -> bool {
    if cfg.span_rows.is_empty() || cfg.span_columns.is_empty() {
        return false;
    }

    cfg.span_rows.iter().any(|(p1, row_span)| {
        cfg.span_columns
            .iter()
            .filter(|(p2, _)| &p1 == p2)
            .any(|(_, col_span)| {
                pos.0 > p1.0 && pos.0 < p1.0 + row_span && pos.1 > p1.1 && pos.1 < p1.1 + col_span
            })
    })
}
