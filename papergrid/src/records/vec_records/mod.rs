//! Module contains [`VecRecords`].

mod cell;
mod cell_info;

use crate::{
    config::Position,
    records::{ExactRecords, IntoRecords, Records},
};
use std::ops::{Deref, DerefMut};

use super::PeekableRecords;

pub use cell::Cell;
pub use cell_info::{CellInfo, StrWithWidth};

/// A [Records] implementation based on allocated buffers.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecRecords<T> {
    data: Vec<Vec<T>>,
    shape: (usize, usize),
}

impl<T> VecRecords<T> {
    /// Creates new [`VecRecords`] structure.
    ///
    /// It assumes that data vector has all rows has the same length().
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let count_columns = data.get(0).map_or(0, |row| row.len());
        let count_rows = data.len();
        let shape = (count_rows, count_columns);

        Self { data, shape }
    }
}

impl<T> Records for VecRecords<T>
where
    T: AsRef<str>,
{
    type Iter = Vec<Vec<T>>;

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        self.data.iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.shape.1
    }

    fn hint_count_rows(&self) -> Option<usize> {
        Some(self.shape.0)
    }
}

impl<'a, T> Records for &'a VecRecords<T>
where
    T: AsRef<str>,
{
    type Iter = &'a [Vec<T>];

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        (&self.data).iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.shape.1
    }

    fn hint_count_rows(&self) -> Option<usize> {
        Some(self.shape.0)
    }
}

impl<T> ExactRecords for VecRecords<T> {
    fn count_rows(&self) -> usize {
        self.shape.0
    }
}

impl<T> PeekableRecords for VecRecords<T>
where
    T: Cell,
{
    fn get_text(&self, (row, col): Position) -> &str {
        self[row][col].text()
    }

    fn count_lines(&self, (row, col): Position) -> usize {
        self[row][col].count_lines()
    }

    fn get_line(&self, (row, col): Position, line: usize) -> &str {
        self[row][col].line(line)
    }

    fn get_line_width(&self, (row, col): Position, line: usize) -> usize {
        self[row][col].line_width(line)
    }

    fn get_width(&self, (row, col): Position) -> usize {
        self[row][col].width()
    }
}

impl<T> Deref for VecRecords<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for VecRecords<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> From<VecRecords<T>> for Vec<Vec<T>> {
    fn from(records: VecRecords<T>) -> Self {
        records.data
    }
}
