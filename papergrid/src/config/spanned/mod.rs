//! A module which contains configuration options for a [`Grid`].
//!
//! [`Grid`]: crate::grid::iterable::Grid

mod borders_config;
mod entity_map;
mod formatting;
mod offset;

use std::collections::HashMap;

use crate::color::{AnsiColor, StaticColor};
use crate::config::compact::CompactConfig;
use crate::config::{
    AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity, Indent, Position, Sides,
};
use borders_config::BordersConfig;

pub use self::{entity_map::EntityMap, formatting::Formatting, offset::Offset};

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpannedConfig {
    margin: Sides<ColoredMarginIndent>,
    padding: EntityMap<Sides<ColoredIndent>>,
    alignment_h: EntityMap<AlignmentHorizontal>,
    alignment_v: EntityMap<AlignmentVertical>,
    formatting: EntityMap<Formatting>,
    span_columns: HashMap<Position, usize>,
    span_rows: HashMap<Position, usize>,
    borders: BordersConfig<char>,
    borders_colors: BordersConfig<AnsiColor<'static>>,
    borders_missing_char: char,
    horizontal_chars: HashMap<Position, HashMap<Offset, char>>,
    horizontal_colors: HashMap<Position, HashMap<Offset, AnsiColor<'static>>>,
    vertical_chars: HashMap<Position, HashMap<Offset, char>>,
    vertical_colors: HashMap<Position, HashMap<Offset, AnsiColor<'static>>>,
    justification: EntityMap<char>,
    justification_color: EntityMap<Option<AnsiColor<'static>>>,
}

impl Default for SpannedConfig {
    fn default() -> Self {
        Self {
            margin: Sides::default(),
            padding: EntityMap::default(),
            formatting: EntityMap::default(),
            alignment_h: EntityMap::new(AlignmentHorizontal::Left),
            alignment_v: EntityMap::new(AlignmentVertical::Top),
            span_columns: HashMap::default(),
            span_rows: HashMap::default(),
            borders: BordersConfig::default(),
            borders_colors: BordersConfig::default(),
            borders_missing_char: ' ',
            horizontal_chars: HashMap::default(),
            horizontal_colors: HashMap::default(),
            vertical_chars: HashMap::default(),
            vertical_colors: HashMap::default(),
            justification: EntityMap::new(' '),
            justification_color: EntityMap::default(),
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
    }

    /// Set a color of margin of a grid.
    pub fn set_margin_color(&mut self, margin: Sides<Option<AnsiColor<'static>>>) {
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
    pub fn get_margin_color(&self) -> Sides<Option<AnsiColor<'static>>> {
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

    /// Clears all theme changes.
    /// And sets it to default.
    pub fn clear_theme(&mut self) {
        self.borders = BordersConfig::default();
        self.horizontal_chars.clear();
        self.vertical_chars.clear();
        self.horizontal_colors.clear();
        self.vertical_colors.clear();
    }

    /// Set the [`Borders`] value as correct one.
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

    /// Override a character on a horizontal line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn set_horizontal_char(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .horizontal_chars
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn lookup_horizontal_char(&self, pos: Position, offset: usize, end: usize) -> Option<char> {
        self.horizontal_chars
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
        self.horizontal_chars.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but line as row and column of a cell;
    /// So its range is line <= count_rows && col < count_columns.
    pub fn remove_overridden_horizontal(&mut self, pos: Position) {
        self.horizontal_chars.remove(&pos);
    }

    /// Override a vertical split line.
    ///
    /// If borders are not set the char won't be used.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn set_vertical_char(&mut self, pos: Position, c: char, offset: Offset) {
        let chars = self
            .vertical_chars
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn lookup_vertical_char(&self, pos: Position, offset: usize, end: usize) -> Option<char> {
        self.vertical_chars
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
        self.vertical_chars.get(&pos).is_some()
    }

    /// Removes a list of overridden chars in a horizontal border.
    ///
    /// It takes not cell position but cell row and column of a line;
    /// So its range is row < count_rows && col <= count_columns.
    pub fn remove_overridden_vertical(&mut self, pos: Position) {
        self.vertical_chars.remove(&pos);
    }

    /// Override a character color on a horizontal line.
    pub fn set_horizontal_color(&mut self, pos: Position, c: AnsiColor<'static>, offset: Offset) {
        let chars = self
            .horizontal_colors
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a overridden color in a horizontal border.
    pub fn lookup_horizontal_color(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<&AnsiColor<'static>> {
        self.horizontal_colors.get(&pos).and_then(|chars| {
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
    }

    /// Override a character color on a vertical line.
    pub fn set_vertical_color(&mut self, pos: Position, c: AnsiColor<'static>, offset: Offset) {
        let chars = self
            .vertical_colors
            .entry(pos)
            .or_insert_with(|| HashMap::with_capacity(1));

        chars.insert(offset, c);
    }

    /// Get a overridden color in a vertical border.
    pub fn lookup_vertical_color(
        &self,
        pos: Position,
        offset: usize,
        end: usize,
    ) -> Option<&AnsiColor<'static>> {
        self.vertical_colors.get(&pos).and_then(|chars| {
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
    }

    /// Set a padding to a given cells.
    pub fn set_padding(&mut self, entity: Entity, padding: Sides<Indent>) {
        let mut pad = self.padding.get(entity).clone();
        pad.left.indent = padding.left;
        pad.right.indent = padding.right;
        pad.top.indent = padding.top;
        pad.bottom.indent = padding.bottom;

        self.padding.insert(entity, pad);
    }

    /// Set a padding to a given cells.
    pub fn set_padding_color(
        &mut self,
        entity: Entity,
        padding: Sides<Option<AnsiColor<'static>>>,
    ) {
        let mut pad = self.padding.get(entity).clone();
        pad.left.color = padding.left;
        pad.right.color = padding.right;
        pad.top.color = padding.top;
        pad.bottom.color = padding.bottom;

        self.padding.insert(entity, pad);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> Sides<Indent> {
        let pad = self.padding.get(entity);
        Sides::new(
            pad.left.indent,
            pad.right.indent,
            pad.top.indent,
            pad.bottom.indent,
        )
    }

    /// Get a padding color for a given [Entity].
    pub fn get_padding_color(&self, entity: Entity) -> Sides<Option<AnsiColor<'static>>> {
        let pad = self.padding.get(entity);
        Sides::new(
            pad.left.color.clone(),
            pad.right.color.clone(),
            pad.top.color.clone(),
            pad.bottom.color.clone(),
        )
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
    pub fn set_border(&mut self, pos: Position, border: Border<char>) {
        self.borders.insert_border(pos, border);
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<char> {
        self.borders.get_border(pos, shape).copied()
    }

    /// Returns a border color of a cell.
    pub fn get_border_color(
        &self,
        pos: Position,
        shape: (usize, usize),
    ) -> Border<&AnsiColor<'static>> {
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
    pub fn get_border_color_global(&self) -> Option<&AnsiColor<'static>> {
        self.borders_colors.get_global()
    }

    /// Sets a color of all borders on the grid.
    pub fn set_border_color_global(&mut self, clr: AnsiColor<'static>) {
        self.borders_colors = BordersConfig::default();
        self.borders_colors.set_global(clr);
    }

    /// Gets colors of a borders carcass on the grid.
    pub fn get_color_borders(&self) -> &Borders<AnsiColor<'static>> {
        self.borders_colors.get_borders()
    }

    /// Sets colors of border carcass on the grid.
    pub fn set_borders_color(&mut self, clrs: Borders<AnsiColor<'static>>) {
        self.borders_colors.set_borders(clrs);
    }

    /// Sets a color of border of a cell on the grid.
    pub fn set_border_color(&mut self, pos: Position, border: Border<AnsiColor<'static>>) {
        self.borders_colors.insert_border(pos, border)
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
    pub fn get_justification(&self, entity: Entity) -> char {
        *self.justification.get(entity)
    }

    /// Get a justification color which will be used while expanding cells width/height.
    ///
    /// `None` means no color.
    pub fn get_justification_color(&self, entity: Entity) -> Option<&AnsiColor<'static>> {
        self.justification_color.get(entity).as_ref()
    }

    /// Set a justification which will be used while expanding cells width/height.
    pub fn set_justification(&mut self, entity: Entity, c: char) {
        self.justification.insert(entity, c);
    }

    /// Set a justification color which will be used while expanding cells width/height.
    ///
    /// `None` removes it.
    pub fn set_justification_color(&mut self, entity: Entity, color: Option<AnsiColor<'static>>) {
        self.justification_color.insert(entity, color);
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_column_spans(&self) -> HashMap<Position, usize> {
        self.span_columns.clone()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_row_spans(&self) -> HashMap<Position, usize> {
        self.span_rows.clone()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_column_span(&self, pos: Position) -> Option<usize> {
        self.span_columns.get(&pos).copied()
    }

    /// Get a span value of the cell, if any is set.
    pub fn get_row_span(&self, pos: Position) -> Option<usize> {
        self.span_rows.get(&pos).copied()
    }

    /// Removes column spans.
    pub fn remove_column_spans(&mut self) {
        self.span_columns.clear()
    }

    /// Removes row spans.
    pub fn remove_row_spans(&mut self) {
        self.span_rows.clear()
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
    pub fn get_horizontal_color(
        &self,
        pos: Position,
        count_rows: usize,
    ) -> Option<&AnsiColor<'static>> {
        self.borders_colors.get_horizontal(pos, count_rows)
    }

    /// Gets a color of a cell vertical.
    pub fn get_vertical_color(
        &self,
        pos: Position,
        count_columns: usize,
    ) -> Option<&AnsiColor<'static>> {
        self.borders_colors.get_vertical(pos, count_columns)
    }

    /// Gets a color of a cell vertical.
    pub fn get_intersection_color(
        &self,
        pos: Position,
        shape: (usize, usize),
    ) -> Option<&AnsiColor<'static>> {
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
}

impl From<CompactConfig> for SpannedConfig {
    fn from(compact: CompactConfig) -> Self {
        use Entity::Global;

        let mut cfg = Self::default();

        cfg.set_padding(Global, *compact.get_padding());
        cfg.set_padding_color(Global, to_ansi_color(compact.get_padding_color()));
        cfg.set_margin(*compact.get_margin());
        cfg.set_margin_color(to_ansi_color(compact.get_margin_color()));
        cfg.set_alignment_horizontal(Global, compact.get_alignment_horizontal());
        cfg.set_borders(*compact.get_borders());
        cfg.set_borders_color(borders_static_color_to_ansi_color(
            *compact.get_borders_color(),
        ));

        if let Some(line) = compact.get_first_horizontal_line() {
            cfg.insert_horizontal_line(
                1,
                HorizontalLine {
                    intersection: line.intersection,
                    left: line.connect1,
                    right: line.connect2,
                    main: Some(line.main),
                },
            );
        }

        cfg
    }
}

fn to_ansi_color(b: Sides<StaticColor>) -> Sides<Option<AnsiColor<'static>>> {
    Sides::new(
        Some(b.left.into()),
        Some(b.right.into()),
        Some(b.top.into()),
        Some(b.bottom.into()),
    )
}

fn borders_static_color_to_ansi_color(b: Borders<StaticColor>) -> Borders<AnsiColor<'static>> {
    Borders {
        left: b.left.map(|c| c.into()),
        right: b.right.map(|c| c.into()),
        top: b.top.map(|c| c.into()),
        bottom: b.bottom.map(|c| c.into()),
        bottom_intersection: b.bottom_intersection.map(|c| c.into()),
        bottom_left: b.bottom_left.map(|c| c.into()),
        bottom_right: b.bottom_right.map(|c| c.into()),
        horizontal: b.horizontal.map(|c| c.into()),
        intersection: b.intersection.map(|c| c.into()),
        left_intersection: b.left_intersection.map(|c| c.into()),
        right_intersection: b.right_intersection.map(|c| c.into()),
        top_intersection: b.top_intersection.map(|c| c.into()),
        top_left: b.top_left.map(|c| c.into()),
        top_right: b.top_right.map(|c| c.into()),
        vertical: b.vertical.map(|c| c.into()),
    }
}

fn set_cell_row_span(cfg: &mut SpannedConfig, pos: Position, span: usize) {
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

fn set_cell_column_span(cfg: &mut SpannedConfig, pos: Position, span: usize) {
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

fn is_cell_covered_by_column_span(cfg: &SpannedConfig, pos: Position) -> bool {
    cfg.span_columns
        .iter()
        .any(|(&(row, col), span)| pos.1 > col && pos.1 < col + span && row == pos.0)
}

fn is_cell_covered_by_row_span(cfg: &SpannedConfig, pos: Position) -> bool {
    cfg.span_rows
        .iter()
        .any(|(&(row, col), span)| pos.0 > row && pos.0 < row + span && col == pos.1)
}

fn is_cell_covered_by_both_spans(cfg: &SpannedConfig, pos: Position) -> bool {
    if !cfg.has_column_spans() || !cfg.has_row_spans() {
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct ColoredIndent {
    indent: Indent,
    color: Option<AnsiColor<'static>>,
}

/// A colorefull margin indent.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ColoredMarginIndent {
    /// An indent value.
    indent: Indent,
    /// An offset value.
    offset: Offset,
    /// An color value.
    color: Option<AnsiColor<'static>>,
}

impl Default for ColoredMarginIndent {
    fn default() -> Self {
        Self {
            indent: Indent::default(),
            offset: Offset::Begin(0),
            color: None,
        }
    }
}

/// HorizontalLine represents a horizontal border line.
pub type HorizontalLine = borders_config::HorizontalLine<char>;

/// HorizontalLine represents a vertical border line.
pub type VerticalLine = borders_config::VerticalLine<char>;
