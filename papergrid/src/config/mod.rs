mod alignment;
mod border;
mod borders;
mod entity;
mod entity_map;
mod formatting;
mod sides;

use std::collections::HashMap;

pub use self::{
    alignment::{AlignmentHorizontal, AlignmentVertical},
    border::Border,
    borders::{Borders, Line},
    entity::{Entity, EntityIterator, Position},
    formatting::Formatting,
    sides::Indent,
};

#[cfg(feature = "color")]
use crate::AnsiColor;

use self::{borders::BordersConfig, entity_map::EntityMap, sides::Sides};

/// This structure represents a settings of a [`Grid`].
///
/// [`Grid`]: crate::Grid.
#[derive(Debug, Clone)]
pub struct GridConfig {
    tab_width: usize,
    margin: Margin,
    padding: EntityMap<Padding>,
    alignment_h: EntityMap<AlignmentHorizontal>,
    alignment_v: EntityMap<AlignmentVertical>,
    formatting: EntityMap<Formatting>,
    span_columns: HashMap<Position, usize>,
    span_rows: HashMap<Position, usize>,
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

impl GridConfig {
    /// Set a column span to a given cells.
    pub fn set_column_span(&mut self, pos: Position, span: usize) {
        set_cell_column_span(self, pos, span);
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_column_span(&self, pos: Position, shape: (usize, usize)) -> Option<usize> {
        match self.span_columns.get(&pos) {
            Some(&span) if is_column_span_valid(pos, span, shape) => Some(span),
            _ => None,
        }
    }

    /// Verifies if there's any spans set.
    pub fn has_column_spans(&self) -> bool {
        !self.span_columns.is_empty()
    }

    /// Get a span value of the cell, if any is set.
    pub fn iter_column_spans(
        &self,
        shape: (usize, usize),
    ) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.span_columns
            .iter()
            .map(|(&pos, &span)| (pos, span))
            .filter(move |&(pos, span)| is_column_span_valid(pos, span, shape))
    }

    /// Set a column span to a given cells.
    pub fn set_row_span(&mut self, pos: Position, span: usize) {
        set_cell_row_span(self, pos, span);
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_row_span(&self, pos: Position, shape: (usize, usize)) -> Option<usize> {
        match self.span_rows.get(&pos) {
            Some(&span) if is_row_span_valid(pos, span, shape) => Some(span),
            _ => None,
        }
    }

    /// Verifies if there's any spans set.
    pub fn has_row_spans(&self) -> bool {
        !self.span_rows.is_empty()
    }

    /// Get a span value of the cell, if any is set.
    pub fn iter_row_spans(
        &self,
        shape: (usize, usize),
    ) -> impl Iterator<Item = (Position, usize)> + '_ {
        self.span_rows
            .iter()
            .map(|(&pos, &span)| (pos, span))
            .filter(move |&(pos, span)| is_row_span_valid(pos, span, shape))
    }

    /// Set a [`Margin`] value.
    pub fn set_margin(&mut self, margin: Margin) {
        self.margin = margin;
    }

    /// Returns a [`Margin`] value currently set.
    pub fn get_margin(&self) -> &Margin {
        &self.margin
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

    /// Override the split line with a custom text.
    ///
    /// If borders are not set the string won't be rendered.
    pub fn get_split_line_text(&self, row: usize) -> Option<&str> {
        self.override_split_lines.get(&row).map(String::as_str)
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Padding) {
        self.padding.set(entity, padding);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> &Padding {
        self.padding.lookup(entity)
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

    /// The function returns whether the cells will be rendered or it will be hidden because of a span.
    pub fn is_cell_visible(&self, pos: Position, shape: (usize, usize)) -> bool {
        !(self.is_cell_covered_by_column_span(pos, shape)
            || self.is_cell_covered_by_row_span(pos, shape)
            || self.is_cell_covered_by_both_spans(pos, shape))
    }

    /// The function checks if a cell is hidden because of a row span.
    pub fn is_cell_covered_by_row_span(&self, pos: Position, shape: (usize, usize)) -> bool {
        is_cell_covered_by_row_span(self, pos, shape)
    }

    /// The function checks if a cell is hidden because of a column span.
    pub fn is_cell_covered_by_column_span(&self, pos: Position, shape: (usize, usize)) -> bool {
        is_cell_covered_by_column_span(self, pos, shape)
    }

    /// The function checks if a cell is hidden indirectly because of a row and column span combination.
    pub fn is_cell_covered_by_both_spans(&self, pos: Position, shape: (usize, usize)) -> bool {
        is_cell_covered_by_both_spans(self, pos, shape)
    }

    // todo: move to Grid as static methods

    /// Checks if [`Grid`] would have a vertical border with the current configuration.
    ///
    /// [`Grid`]: crate::Grid
    pub fn has_vertical(&self, col: usize, count_columns: usize) -> bool {
        self.borders.has_vertical(col, count_columns)
    }

    /// Checks if [`Grid`] would have a horizontal border with the current configuration.
    ///
    /// [`Grid`]: crate::Grid
    pub fn has_horizontal(&self, row: usize, count_rows: usize) -> bool {
        self.borders.has_horizontal(row, count_rows)
    }

    /// Set border set a border value to all cells in [`Entity`].
    pub fn set_border(&mut self, pos: Position, border: Border) {
        self.borders.insert_border(pos, border);
    }

    /// Sets off all borders possible on the [`Entity`].
    ///
    /// It doesn't changes globaly set borders through [`GridConfig::set_borders`].
    pub fn remove_border(&mut self, pos: Position, count_columns: usize) {
        self.borders.remove_border(pos, count_columns);
    }

    /// Set a character wich will be used in case any missconfiguration of borders.
    ///
    /// It will be usde for example when you set a left char for border frame and top but didn't set a top left corner.
    pub fn set_borders_missing(&mut self, c: char) {
        self.borders_missing_char = c;
    }

    /// Calculates an amount of vertical lines would present on the [`Grid`].
    ///
    /// [`Grid`]: crate::Grid
    pub fn count_vertical(&self, count_columns: usize) -> usize {
        (0..=count_columns)
            .filter(|&col| self.has_vertical(col, count_columns))
            .count()
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<char> {
        self.borders.get_border(pos, shape.0, shape.1).copied()
    }

    /// Gets a vertical character which would be rendered on the [`Grid`].
    ///
    /// [`Grid`]: crate::Grid
    pub fn get_vertical(&self, pos: Position, count_columns: usize) -> Option<&char> {
        let c = self.borders.get_vertical(pos, count_columns);
        if c.is_some() {
            return c;
        }

        if self.has_vertical(pos.1, count_columns) {
            return Some(&self.borders_missing_char);
        }

        None
    }

    /// Gets a horizontal character which would be rendered on the [`Grid`].
    ///
    /// [`Grid`]: crate::Grid
    pub fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<&char> {
        let c = self.borders.get_horizontal(pos, count_rows);
        if c.is_some() {
            return c;
        }

        if self.has_horizontal(pos.0, count_rows) {
            return Some(&self.borders_missing_char);
        }

        None
    }

    /// Gets an intersection character which would be rendered on the [`Grid`].
    ///
    /// [`Grid`]: crate::Grid
    pub fn get_intersection(&self, pos: Position, shape: (usize, usize)) -> Option<&char> {
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
    /// Gets a color of all borders on the [`Grid`].
    pub fn get_border_color_global(&self) -> Option<&AnsiColor> {
        self.border_colors.get_global()
    }

    /// Sets a color of all borders on the [`Grid`].
    pub fn set_border_color_global(&mut self, clr: AnsiColor) {
        self.border_colors = BordersConfig::default();
        self.border_colors.set_global(clr);
    }

    /// Gets colors of a borders carcass on the [`Grid`].
    pub fn get_color_borders(&self) -> &Borders<AnsiColor> {
        self.border_colors.get_borders()
    }

    /// Sets colors of border carcass on the [`Grid`].
    pub fn set_borders_color(&mut self, clrs: Borders<AnsiColor>) {
        self.border_colors.set_borders(clrs);
    }

    /// Sets a color of border of a cell on the [`Grid`].
    pub fn set_border_color(&mut self, pos: Position, border: Border<AnsiColor>) {
        self.border_colors.insert_border(pos, border)
    }

    /// Get colors for a [`Margin`] value.
    pub fn get_margin_color(&self) -> &MarginColor {
        &self.margin_color
    }

    /// Set colors for a [`Margin`] value.
    pub fn set_margin_color(&mut self, color: MarginColor) {
        self.margin_color = color;
    }

    /// Get a padding to a given cells.
    pub fn get_padding_color(&self, entity: Entity) -> &PaddingColor {
        self.padding_color.lookup(entity)
    }

    /// Set a padding to a given cells.
    pub fn set_padding_color(&mut self, entity: Entity, color: PaddingColor) {
        self.padding_color.set(entity, color);
    }

    /// Gets a color of a cell horizontal.
    pub fn get_horizontal_color(&self, pos: Position, count_rows: usize) -> Option<&AnsiColor> {
        self.border_colors.get_horizontal(pos, count_rows)
    }

    /// Gets a color of a cell vertical.
    pub fn get_vertical_color(&self, pos: Position, count_columns: usize) -> Option<&AnsiColor> {
        self.border_colors.get_vertical(pos, count_columns)
    }

    /// Gets a color of a cell intersection.
    pub fn get_intersection_color(
        &self,
        pos: Position,
        shape: (usize, usize),
    ) -> Option<&AnsiColor> {
        self.border_colors.get_intersection(pos, shape.0, shape.1)
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
            span_columns: HashMap::default(),
            span_rows: HashMap::default(),
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
        if cfg.is_cell_visible(pos, (std::usize::MAX, std::usize::MAX)) {
            return Some(pos.0);
        }

        if pos.0 == 0 {
            return None;
        }

        pos.0 -= 1;
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

fn closest_visible_column(cfg: &GridConfig, mut pos: Position) -> Option<usize> {
    loop {
        if cfg.is_cell_visible(pos, (std::usize::MAX, std::usize::MAX)) {
            return Some(pos.1);
        }

        if pos.1 == 0 {
            return None;
        }

        pos.1 -= 1;
    }
}

fn is_cell_covered_by_column_span(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    if cfg.span_columns.is_empty() {
        return false;
    }

    cfg.span_columns
        .iter()
        .filter(|(&pos, &span)| is_column_span_valid(pos, span, shape))
        .any(|(&(row, col), span)| pos.1 > col && pos.1 < col + span && row == pos.0)
}

fn is_cell_covered_by_row_span(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    if cfg.span_rows.is_empty() {
        return false;
    }

    cfg.span_rows
        .iter()
        .filter(|(&pos, &span)| is_row_span_valid(pos, span, shape))
        .any(|(&(row, col), span)| pos.0 > row && pos.0 < row + span && col == pos.1)
}

fn is_cell_covered_by_both_spans(cfg: &GridConfig, pos: Position, shape: (usize, usize)) -> bool {
    if cfg.span_rows.is_empty() || cfg.span_columns.is_empty() {
        return false;
    }

    cfg.span_rows
        .iter()
        .filter(|(&pos, &span)| is_row_span_valid(pos, span, shape))
        .any(|(p1, row_span)| {
            cfg.span_columns
                .iter()
                .filter(|(&pos, &span)| is_column_span_valid(pos, span, shape))
                .filter(|(p2, _)| &p1 == p2)
                .any(|(_, col_span)| {
                    pos.0 > p1.0
                        && pos.0 < p1.0 + row_span
                        && pos.1 > p1.1
                        && pos.1 < p1.1 + col_span
                })
        })
}

fn is_column_span_valid(
    pos: Position,
    span: usize,
    (count_rows, count_cols): (usize, usize),
) -> bool {
    // ignore spans which are invlid
    let pos_correct = pos.1 < count_cols && pos.0 < count_rows;
    // ignore a span range wich begger then count rows
    let span_correct = span + pos.1 <= count_cols;

    pos_correct && span_correct
}

fn is_row_span_valid(pos: Position, span: usize, (count_rows, count_cols): (usize, usize)) -> bool {
    // ignore spans which are invlid
    let pos_correct = pos.1 < count_cols && pos.0 < count_rows;
    // ignore a span range wich begger then count columns
    let span_correct = span + pos.0 <= count_rows;

    pos_correct && span_correct
}
