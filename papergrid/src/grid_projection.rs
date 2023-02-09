//! A module which contains [GridProjection] a grid like structure for various checks.

use std::marker::PhantomData;

use crate::{
    color::AnsiColor,
    config::{Border, GridConfig, Position},
    dimension::Dimension,
};

/// A Grid mock structure but with no data.
///
/// It might be handy for some checks which infolves a shape of a grid.
#[derive(Debug, Clone)]
pub struct GridProjection<'a, R = WithRows, C = WithColumns> {
    cfg: &'a GridConfig,
    count_rows: usize,
    count_columns: usize,
    _rows: PhantomData<R>,
    _columns: PhantomData<C>,
}

/// A marker structure.
#[derive(Debug)]
pub struct WithRows;

/// A marker structure.
#[derive(Debug)]
pub struct WithColumns;

impl<'a> GridProjection<'a, (), ()> {
    /// Returns [`GridProjection`] with no shape.
    pub fn new(cfg: &'a GridConfig) -> Self {
        Self {
            cfg,
            count_columns: 0,
            count_rows: 0,
            _rows: PhantomData,
            _columns: PhantomData,
        }
    }
}

impl<'a> GridProjection<'a, WithRows, WithColumns> {
    /// Returns [`GridProjection`] with shape.
    pub fn with_shape(cfg: &'a GridConfig, shape: (usize, usize)) -> Self {
        Self {
            cfg,
            count_rows: shape.0,
            count_columns: shape.1,
            _rows: PhantomData,
            _columns: PhantomData,
        }
    }
}

impl GridProjection<'_, WithRows, WithColumns> {
    /// Gets an intersection character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_intersection(&self, pos: Position) -> Option<char> {
        let c = self
            .cfg
            .get_border_config()
            .get_intersection(pos, self.shape());
        if let Some(c) = c {
            return Some(*c);
        }

        if self.has_horizontal(pos.0) && self.has_vertical(pos.1) {
            return Some(self.cfg.get_borders_missing());
        }

        None
    }

    /// Gets a color of border of a cell on the grid.
    pub fn get_border_color(&self, pos: Position) -> Border<&AnsiColor<'_>> {
        self.cfg
            .get_border_color_config()
            .get_border(pos, self.shape())
    }

    /// Gets a color of a cell intersection.
    pub fn get_intersection_color(&self, pos: Position) -> Option<&AnsiColor<'_>> {
        self.cfg
            .get_border_color_config()
            .get_intersection(pos, self.shape())
    }

    /// Returns a border of a cell.
    pub fn get_border(&self, pos: Position) -> Border<char> {
        self.cfg
            .get_border_config()
            .get_border(pos, self.shape())
            .copied()
    }
}

impl<'a, R, C> GridProjection<'a, R, C> {
    /// Sets a rows size.
    pub fn count_rows(self, len: usize) -> GridProjection<'a, WithRows, C> {
        GridProjection {
            cfg: self.cfg,
            count_rows: len,
            count_columns: self.count_columns,
            _columns: self._columns,
            _rows: PhantomData,
        }
    }

    /// Sets a columns size.
    pub fn count_columns(self, len: usize) -> GridProjection<'a, R, WithColumns> {
        GridProjection {
            cfg: self.cfg,
            count_rows: self.count_rows,
            count_columns: len,
            _rows: self._rows,
            _columns: PhantomData,
        }
    }

    /// Returns a shape of a grid.
    pub fn shape(&self) -> (usize, usize) {
        (self.count_rows, self.count_columns)
    }

    /// Verifies if there's any spans set.
    pub fn has_span_columns(&self) -> bool {
        self.cfg.has_column_spans()
    }

    /// Verifies if there's any spans set.
    pub fn has_span_rows(&self) -> bool {
        self.cfg.has_row_spans()
    }

    /// The function returns whether the cells will be rendered or it will be hidden because of a span.
    pub fn is_cell_visible(&self, pos: Position) -> bool {
        !(self.is_cell_covered_by_column_span(pos)
            || self.is_cell_covered_by_row_span(pos)
            || self.is_cell_covered_by_both_spans(pos))
    }

    /// The function checks if a cell is hidden because of a row span.
    pub fn is_cell_covered_by_row_span(&self, pos: Position) -> bool {
        is_cell_covered_by_row_span(self.cfg, pos)
    }

    /// The function checks if a cell is hidden because of a column span.
    pub fn is_cell_covered_by_column_span(&self, pos: Position) -> bool {
        is_cell_covered_by_column_span(self.cfg, pos)
    }

    /// The function checks if a cell is hidden indirectly because of a row and column span combination.
    pub fn is_cell_covered_by_both_spans(&self, pos: Position) -> bool {
        is_cell_covered_by_both_spans(self.cfg, pos)
    }
}

impl<C> GridProjection<'_, WithRows, C> {
    /// Checks if grid would have a horizontal border with the current configuration.
    ///
    /// grid: crate::Grid
    pub fn has_horizontal(&self, row: usize) -> bool {
        self.cfg
            .get_border_config()
            .has_horizontal(row, self.count_rows)
    }

    /// Calculates an amount of horizontal lines would present on the grid.
    ///
    /// grid: crate::Grid
    pub fn count_horizontal(&self) -> usize {
        (0..=self.count_rows)
            .filter(|&row| self.has_horizontal(row))
            .count()
    }

    /// Gets a horizontal character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_horizontal(&self, pos: Position) -> Option<char> {
        let c = self
            .cfg
            .get_border_config()
            .get_horizontal(pos, self.count_rows);
        if let Some(c) = c {
            return Some(*c);
        }

        if self.has_horizontal(pos.0) {
            return Some(self.cfg.get_borders_missing());
        }

        None
    }

    /// Gets a color of a cell horizontal.
    pub fn get_horizontal_color(&self, pos: Position) -> Option<&AnsiColor<'_>> {
        self.cfg
            .get_border_color_config()
            .get_horizontal(pos, self.count_rows)
    }

    /// Calculates total height with a given dimensions.
    pub fn total_height<D: Dimension>(&self, dimension: &D) -> usize {
        total_height(self, dimension)
    }
}

impl<R> GridProjection<'_, R, WithColumns> {
    /// Checks if grid would have a vertical border with the current configuration.
    ///
    /// grid: crate::Grid
    pub fn has_vertical(&self, col: usize) -> bool {
        self.cfg
            .get_border_config()
            .has_vertical(col, self.count_columns)
    }

    /// Calculates an amount of vertical lines would present on the grid.
    ///
    /// grid: crate::Grid
    pub fn count_vertical(&self) -> usize {
        (0..=self.count_columns)
            .filter(|&col| self.has_vertical(col))
            .count()
    }

    /// Gets a vertical character which would be rendered on the grid.
    ///
    /// grid: crate::Grid
    pub fn get_vertical(&self, pos: Position) -> Option<char> {
        if let Some(c) = self
            .cfg
            .get_border_config()
            .get_vertical(pos, self.count_columns)
        {
            return Some(*c);
        }

        if self.has_vertical(pos.1) {
            return Some(self.cfg.get_borders_missing());
        }

        None
    }

    /// Gets a color of a cell vertical.
    pub fn get_vertical_color(&self, pos: Position) -> Option<&AnsiColor<'_>> {
        self.cfg
            .get_border_color_config()
            .get_vertical(pos, self.count_columns)
    }

    /// Calculates total width with a given dimensions.
    pub fn total_width<D: Dimension>(&self, dimension: &D) -> usize {
        total_width(self, dimension)
    }
}

impl AsRef<GridConfig> for GridProjection<'_> {
    fn as_ref(&self) -> &GridConfig {
        self.cfg
    }
}

fn is_cell_covered_by_column_span(cfg: &GridConfig, pos: Position) -> bool {
    if !cfg.has_column_spans() {
        return false;
    }

    cfg.iter_span_columns()
        .any(|((row, col), span)| pos.1 > col && pos.1 < col + span && row == pos.0)
}

fn is_cell_covered_by_row_span(cfg: &GridConfig, pos: Position) -> bool {
    if !cfg.has_row_spans() {
        return false;
    }

    cfg.iter_span_rows()
        .any(|((row, col), span)| pos.0 > row && pos.0 < row + span && col == pos.1)
}

fn is_cell_covered_by_both_spans(cfg: &GridConfig, pos: Position) -> bool {
    if !cfg.has_column_spans() || !cfg.has_row_spans() {
        return false;
    }

    cfg.iter_span_rows().any(|(p1, row_span)| {
        cfg.iter_span_columns()
            .filter(|(p2, _)| &p1 == p2)
            .any(|(_, col_span)| {
                pos.0 > p1.0 && pos.0 < p1.0 + row_span && pos.1 > p1.1 && pos.1 < p1.1 + col_span
            })
    })
}

fn total_width<D: Dimension, R>(gp: &GridProjection<'_, R, WithColumns>, dimension: &D) -> usize {
    (0..gp.count_columns)
        .map(|i| dimension.get_width(i))
        .sum::<usize>()
        + gp.count_vertical()
}

fn total_height<D: Dimension, C>(gp: &GridProjection<'_, WithRows, C>, dimension: &D) -> usize {
    (0..gp.count_rows)
        .map(|i| dimension.get_height(i))
        .sum::<usize>()
        + gp.count_horizontal()
}
