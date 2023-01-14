//! The module contains a [Records] abstraction of a [`Grid`] trait and its implementers.
//!
//! [`Grid`]: crate::Grid

use crate::{width::WidthFunc, Position};

pub mod cell_info;
pub mod vec_records;
pub mod empty;

/// The representation of data, rows and columns of a [`Grid`].
///
/// [`Grid`]: crate::Grid
pub trait Records {
    type Cell: RecordCell;
    type Cells: Iterator<Item = Self::Cell>;
    type IntoRecords: Iterator<Item = Self::Cells>;

    /// Returns amount of columns on a grid.
    fn count_columns(&self) -> usize;

    /// Returns an iterator over rows.
    fn iter_rows(&self) -> Self::IntoRecords;

    /// Returns amount of rows on a grid.
    fn hint_rows(&self) -> Option<usize> {
        None
    }
}

impl<'a, T> Records for &'a T
where
    T: Records,
{
    type Cell = T::Cell;
    type Cells = T::Cells;
    type IntoRecords = T::IntoRecords;

    fn count_columns(&self) -> usize {
        T::count_columns(self)
    }

    fn iter_rows(&self) -> Self::IntoRecords {
        T::iter_rows(self)
    }

    fn hint_rows(&self) -> Option<usize> {
        T::hint_rows(self)
    }
}

pub trait ExactRecords: Records {
    /// Returns an exact amount of rows in records.
    ///
    /// It must be guarated that an iterator will yield this amount.
    fn count_rows(&self) -> usize;

    /// Returns a text of a cell by an index.
    fn get(&self, pos: Position) -> Option<Self::Cell>;
}

impl<'a, T> ExactRecords for &'a T
where
    T: ExactRecords,
{
    fn count_rows(&self) -> usize {
        T::count_rows(self)
    }

    fn get(&self, pos: Position) -> Option<Self::Cell> {
        T::get(self, pos)
    }
}

pub trait RecordCell {
    type Text: AsRef<str>;
    type Line: AsRef<str>;
    type Lines: IntoIterator<Item = Self::Line>;

    /// Returns a text of a cell by an index.
    fn get_text(&self) -> Self::Text;

    /// Returns a line of a text of a cell by an index.
    fn get_line(&self, i: usize) -> Self::Line;

    /// Returns a line of a text of a cell by an index.
    fn get_lines(&self) -> Self::Lines;

    /// Returns an amount of lines of a text of a cell by an index.
    fn count_lines(&self) -> usize;

    /// Returns a width of a text of a cell by an index.
    fn get_width<W: WidthFunc>(&self, width_ctrl: W) -> usize {
        width_ctrl.width_multiline(self.get_text().as_ref())
    }

    /// Returns a width of line of a text of a cell by an index.
    fn get_line_width<W: WidthFunc>(&self, i: usize, width_ctrl: W) -> usize {
        width_ctrl.width_multiline(self.get_line(i).as_ref())
    }
}

impl<'a, T> RecordCell for &'a T
where
    T: RecordCell,
{
    type Text = T::Text;
    type Line = T::Line;
    type Lines = T::Lines;

    fn get_text(&self) -> Self::Text {
        T::get_text(self)
    }

    fn get_line(&self, i: usize) -> Self::Line {
        T::get_line(self, i)
    }

    fn get_lines(&self) -> Self::Lines {
        T::get_lines(self)
    }

    fn count_lines(&self) -> usize {
        T::count_lines(self)
    }

    fn get_width<W: WidthFunc>(&self, width_ctrl: W) -> usize {
        T::get_width(self, width_ctrl)
    }

    fn get_line_width<W: WidthFunc>(&self, i: usize, width_ctrl: W) -> usize {
        T::get_line_width(self, i, width_ctrl)
    }
}

/// A [`Grid`] representation of a data set which can be modified.
///
/// [`Grid`]: crate::Grid
pub trait RecordsMut {
    /// Sets a text to a given cell by index.
    fn set<W: WidthFunc>(&mut self, pos: Position, text: String, width_ctrl: W);

    /// Updates a given cell by index.
    ///
    /// Maybe used if width function was changed.
    fn update<W: WidthFunc>(&mut self, pos: Position, width_ctrl: W);
}

impl<'a, T> RecordsMut for &'a mut T
where
    T: RecordsMut,
{
    fn set<W: WidthFunc>(&mut self, pos: Position, text: String, width_ctrl: W) {
        T::set(self, pos, text, width_ctrl)
    }

    fn update<W: WidthFunc>(&mut self, pos: Position, width_ctrl: W) {
        T::update(self, pos, width_ctrl)
    }
}

/// A [`Grid`] representation of a data set which can be modified by moving rows/columns around.
///
/// [`Grid`]: crate::Grid
pub trait Resizable {
    /// Swap cells with one another.
    fn swap(&mut self, lhs: Position, rhs: Position);
    /// Swap rows with one another.
    fn swap_row(&mut self, lhs: usize, rhs: usize);
    /// Swap columns with one another.
    fn swap_column(&mut self, lhs: usize, rhs: usize);
    /// Adds a new row to a data set.
    fn push_row(&mut self);
    /// Adds a new column to a data set.
    fn push_column(&mut self);
    /// Removes a row from a data set by index.
    fn remove_row(&mut self, row: usize);
    /// Removes a column from a data set by index.
    fn remove_column(&mut self, column: usize);
    /// Inserts a row to specific by row index.
    fn insert_row(&mut self, row: usize);
}

impl<'a, T> Resizable for &'a mut T
where
    T: Resizable,
{
    fn swap(&mut self, lhs: Position, rhs: Position) {
        T::swap(self, lhs, rhs)
    }

    fn swap_row(&mut self, lhs: usize, rhs: usize) {
        T::swap_row(self, lhs, rhs)
    }

    fn swap_column(&mut self, lhs: usize, rhs: usize) {
        T::swap_column(self, lhs, rhs)
    }

    fn push_row(&mut self) {
        T::push_row(self)
    }

    fn push_column(&mut self) {
        T::push_column(self)
    }

    fn remove_row(&mut self, row: usize) {
        T::remove_row(self, row)
    }

    fn remove_column(&mut self, column: usize) {
        T::remove_column(self, column)
    }

    fn insert_row(&mut self, row: usize) {
        T::insert_row(self, row)
    }
}
