//! A module which contains configuration options for a [`Grid`].
//!
//! [`Grid`]: crate::grid::iterable::Grid

mod borders_config;
mod entity_map;
mod formatting;
mod offset;

use std::collections::HashMap;

use crate::color::{AnsiColor, StaticColor};
use crate::config::{
    AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity, Indent, Position, Sides,
};
use crate::config::compact::CompactConfig;
use borders_config::BordersConfig;

pub use self::{entity_map::EntityMap, formatting::Formatting, offset::Offset};

/// This structure represents a settings of a grid.
///
/// grid: crate::Grid.
#[derive(Debug, Clone)]
pub struct SpannedConfig {
    margin: Sides<ColoredMarginIndent>,
    padding: EntityMap<Sides<ColoredIndent>>,
    alignment_h: EntityMap<AlignmentHorizontal>,
    alignment_v: EntityMap<AlignmentVertical>,
    formatting: EntityMap<Formatting>,
    span_columns: HashMap<Position, usize>,
    span_rows: HashMap<Position, usize>,
    borders: BordersConfig<char>,
    borders_missing_char: char,
    override_horizontal_borders: HashMap<Position, HashMap<Offset, char>>,
    override_vertical_borders: HashMap<Position, HashMap<Offset, char>>,
    border_colors: BordersConfig<AnsiColor<'static>>,
}

impl Default for SpannedConfig {
    fn default() -> Self {
        Self {
            margin: Sides::default(),
            padding: EntityMap::default(),
            formatting: EntityMap::default(),
            alignment_h: EntityMap::new(AlignmentHorizontal::Left),
            alignment_v: EntityMap::new(AlignmentVertical::Top),
            borders: BordersConfig::default(),
            borders_missing_char: ' ',
            span_columns: HashMap::default(),
            span_rows: HashMap::default(),
            override_horizontal_borders: HashMap::default(),
            override_vertical_borders: HashMap::default(),
            border_colors: BordersConfig::default(),
        }
    }
}

impl SpannedConfig {
    /// Returns a [`Margin`] value currently set.
    pub fn get_margin(&self) -> &Sides<ColoredMarginIndent> {
        &self.margin
    }

    /// Set a [`Margin`] value.
    pub fn get_margin_mut(&mut self) -> &mut Sides<ColoredMarginIndent> {
        &mut self.margin
    }

    /// Clears all theme changes.
    /// And sets it to default.
    pub fn clear_theme(&mut self) {
        self.borders = BordersConfig::default();
        self.override_horizontal_borders.clear();
        self.override_vertical_borders.clear();
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
    pub fn remove_horizontal_line(&mut self, line: usize) {
        self.borders.remove_horizontal_line(line);
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
    pub fn remove_vertical_line(&mut self, line: usize) {
        self.borders.remove_vertical_line(line);
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
    pub fn set_padding(&mut self, entity: Entity, pad: Sides<impl Into<ColoredIndent>>) {
        let val = Sides::new(
            pad.left.into(),
            pad.right.into(),
            pad.top.into(),
            pad.bottom.into(),
        );
        self.padding.insert(entity, val);
    }

    /// Get a padding for a given [Entity].
    pub fn get_padding(&self, entity: Entity) -> &Sides<ColoredIndent> {
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
    pub fn set_border(&mut self, pos: Position, border: Border<char>) {
        self.borders.insert_border(pos, border);
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position, shape: (usize, usize)) -> Border<char> {
        self.borders.get_border(pos, shape).copied()
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
        self.border_colors.get_global()
    }

    /// Sets a color of all borders on the grid.
    pub fn set_border_color_global(&mut self, clr: AnsiColor<'static>) {
        self.border_colors = BordersConfig::default();
        self.border_colors.set_global(clr);
    }

    /// Gets colors of a borders carcass on the grid.
    pub fn get_color_borders(&self) -> &Borders<AnsiColor<'static>> {
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
        self.border_colors.remove_border(pos, shape);
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
}

impl SpannedConfig {
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
        self.border_colors.get_horizontal(pos, count_rows)
    }

    /// Gets a color of a cell vertical.
    pub fn get_vertical_color(
        &self,
        pos: Position,
        count_columns: usize,
    ) -> Option<&AnsiColor<'static>> {
        self.border_colors.get_vertical(pos, count_columns)
    }

    /// Gets a color of a cell vertical.
    pub fn get_intersection_color(
        &self,
        pos: Position,
        shape: (usize, usize),
    ) -> Option<&AnsiColor<'static>> {
        self.border_colors.get_intersection(pos, shape)
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

        let pad = to_padding(compact.get_padding(), compact.get_padding_color());
        cfg.set_padding(Global, pad);
        *cfg.get_margin_mut() = to_margin(compact.get_margin(), compact.get_margin_color());
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

fn to_margin(pad: &Sides<Indent>, colors: Sides<StaticColor>) -> Sides<ColoredMarginIndent> {
    let colors = to_ansi_color(colors);
    Sides::new(
        ColoredMarginIndent::new(pad.left, Offset::Begin(0), Some(colors.left)),
        ColoredMarginIndent::new(pad.right, Offset::Begin(0), Some(colors.right)),
        ColoredMarginIndent::new(pad.top, Offset::Begin(0), Some(colors.top)),
        ColoredMarginIndent::new(pad.bottom, Offset::Begin(0), Some(colors.bottom)),
    )
}

fn to_padding(pad: &Sides<Indent>, colors: Sides<StaticColor>) -> Sides<ColoredIndent> {
    let colors = to_ansi_color(colors);
    Sides::new(
        ColoredIndent::new(pad.left, Some(colors.left)),
        ColoredIndent::new(pad.right, Some(colors.right)),
        ColoredIndent::new(pad.top, Some(colors.top)),
        ColoredIndent::new(pad.bottom, Some(colors.bottom)),
    )
}

fn to_ansi_color(b: Sides<StaticColor>) -> Sides<AnsiColor<'static>> {
    Sides::new(b.left.into(), b.right.into(), b.top.into(), b.bottom.into())
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

/// A colorefull indent.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ColoredIndent {
    /// An indent value.
    pub indent: Indent,
    /// An color value.
    pub color: Option<AnsiColor<'static>>,
}

impl ColoredIndent {
    /// An creates a new colored indent.
    pub fn new(indent: Indent, color: Option<AnsiColor<'static>>) -> Self {
        Self { indent, color }
    }
}

impl From<Indent> for ColoredIndent {
    fn from(indent: Indent) -> Self {
        Self::new(indent, None)
    }
}

/// A colorefull margin indent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColoredMarginIndent {
    /// An indent value.
    pub indent: Indent,
    /// An offset value.
    pub offset: Offset,
    /// An color value.
    pub color: Option<AnsiColor<'static>>,
}

impl ColoredMarginIndent {
    /// An creates a new colored margin indent.
    pub fn new(indent: Indent, offset: Offset, color: Option<AnsiColor<'static>>) -> Self {
        Self {
            indent,
            offset,
            color,
        }
    }
}

impl Default for ColoredMarginIndent {
    fn default() -> Self {
        Self::new(Indent::default(), Offset::Begin(0), None)
    }
}

impl From<Indent> for ColoredMarginIndent {
    fn from(indent: Indent) -> Self {
        Self::new(indent, Offset::Begin(0), None)
    }
}

/// HorizontalLine represents a horizontal border line.
pub type HorizontalLine = borders_config::HorizontalLine<char>;

/// HorizontalLine represents a vertical border line.
pub type VerticalLine = borders_config::VerticalLine<char>;
