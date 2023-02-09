//! The module contains [`Records`], [`ExactRecords`], [`RecordsMut`], [`Resizable`] traits
//! and its implementations.
//!
//! Also it provies a list of helpers for a user built [`Records`] via [`into_records`].

mod empty_records;
mod vec_records;

pub mod into_records;

use std::ops::Index;

use crate::grid::config::Position;

pub use papergrid::records::{IntoRecords, IterRecords, Records};

pub use empty_records::EmptyRecords;
pub use vec_records::VecRecords;

/// [Records] extension which guarantess the amount of rows.
pub trait ExactRecords {
    /// A cell represented by a string value.
    type Cell: AsRef<str>;

    /// Returns an exact amount of rows in records.
    ///
    /// It must be guarated that an iterator will yield this amount.
    fn count_rows(&self) -> usize;

    /// Returns a text of a cell by an index.
    fn get_cell(&self, pos: Position) -> &Self::Cell;
}

impl<'a, T> ExactRecords for &'a T
where
    T: ExactRecords,
{
    type Cell = T::Cell;

    fn count_rows(&self) -> usize {
        T::count_rows(self)
    }

    fn get_cell(&self, pos: Position) -> &Self::Cell {
        T::get_cell(self, pos)
    }
}

impl<T, I> ExactRecords for Vec<I>
where
    T: AsRef<str>,
    I: Index<usize, Output = T>,
{
    type Cell = T;

    fn count_rows(&self) -> usize {
        self.len()
    }

    fn get_cell(&self, (row, col): Position) -> &Self::Cell {
        &self[row][col]
    }
}

impl<T, I> ExactRecords for [I]
where
    T: AsRef<str>,
    I: Index<usize, Output = T>,
{
    type Cell = T;

    fn count_rows(&self) -> usize {
        self.len()
    }

    fn get_cell(&self, (row, col): Position) -> &Self::Cell {
        &self[row][col]
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

impl<T> Resizable for Vec<Vec<T>>
where
    T: Default + Clone,
{
    fn swap(&mut self, lhs: Position, rhs: Position) {
        if lhs == rhs {
            return;
        }

        let t = std::mem::take(&mut self[lhs.0][lhs.1]);
        let t = std::mem::replace(&mut self[rhs.0][rhs.1], t);
        let _ = std::mem::replace(&mut self[lhs.0][lhs.1], t);
    }

    fn swap_row(&mut self, lhs: usize, rhs: usize) {
        let t = std::mem::take(&mut self[lhs]);
        let t = std::mem::replace(&mut self[rhs], t);
        let _ = std::mem::replace(&mut self[lhs], t);
    }

    fn swap_column(&mut self, lhs: usize, rhs: usize) {
        for row in self.iter_mut() {
            row.swap(lhs, rhs);
        }
    }

    fn push_row(&mut self) {
        let count_columns = self.get(0).map(|l| l.len()).unwrap_or(0);
        self.push(vec![T::default(); count_columns]);
    }

    fn push_column(&mut self) {
        for row in self.iter_mut() {
            row.push(T::default());
        }
    }

    fn remove_row(&mut self, row: usize) {
        self.remove(row);
    }

    fn remove_column(&mut self, column: usize) {
        for row in self.iter_mut() {
            row.remove(column);
        }
    }

    fn insert_row(&mut self, row: usize) {
        let count_columns = self.get(0).map(|l| l.len()).unwrap_or(0);
        self.insert(row, vec![T::default(); count_columns]);
    }
}

/// A [`Grid`] representation of a data set which can be modified.
///
/// [`Grid`]: crate::Grid
pub trait RecordsMut<Text> {
    /// Sets a text to a given cell by index.
    fn set(&mut self, pos: Position, text: Text);
}

impl<'a, T, Text> RecordsMut<Text> for &'a mut T
where
    T: RecordsMut<Text>,
{
    fn set(&mut self, pos: Position, text: Text) {
        T::set(self, pos, text)
    }
}
