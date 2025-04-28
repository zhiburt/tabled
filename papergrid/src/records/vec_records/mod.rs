//! Module contains [`VecRecords`].

mod cell;
mod text;

use crate::{
    config::Position,
    records::{ExactRecords, IntoRecords, Records},
};
use std::ops::{Deref, DerefMut};

use super::PeekableRecords;

pub use cell::Cell;
pub use text::{StrWithWidth, Text};

/// A [Records] implementation based on allocated buffers.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecRecords<T> {
    data: Vec<Vec<T>>,
    count_rows: usize,
    count_cols: usize,
}

impl<T> VecRecords<T> {
    /// Creates new [`VecRecords`] structure.
    ///
    /// It assumes that data vector has all rows has the same length().
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let count_cols = data.first().map_or(0, |row| row.len());
        let count_rows = data.len();

        Self::with_size(data, count_rows, count_cols)
    }

    /// Creates new [`VecRecords`] structure.
    pub fn with_size(data: Vec<Vec<T>>, count_rows: usize, count_cols: usize) -> Self {
        Self {
            data,
            count_cols,
            count_rows,
        }
    }
}

impl<T> Records for VecRecords<T> {
    type Iter = Vec<Vec<T>>;

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        self.data.iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.count_cols
    }

    fn hint_count_rows(&self) -> Option<usize> {
        Some(self.count_rows)
    }
}

impl<'a, T> Records for &'a VecRecords<T> {
    type Iter = &'a [Vec<T>];

    fn iter_rows(self) -> <Self::Iter as IntoRecords>::IterRows {
        (&self.data).iter_rows()
    }

    fn count_columns(&self) -> usize {
        self.count_cols
    }

    fn hint_count_rows(&self) -> Option<usize> {
        Some(self.count_rows)
    }
}

impl<T> ExactRecords for VecRecords<T> {
    fn count_rows(&self) -> usize {
        self.count_rows
    }
}

impl<T> PeekableRecords for VecRecords<T>
where
    T: Cell,
{
    fn get_text(&self, pos: Position) -> &str {
        self[pos.row][pos.col].text()
    }

    fn count_lines(&self, pos: Position) -> usize {
        self[pos.row][pos.col].count_lines()
    }

    fn get_line(&self, pos: Position, line: usize) -> &str {
        self[pos.row][pos.col].line(line)
    }

    fn get_line_width(&self, pos: Position, line: usize) -> usize {
        self[pos.row][pos.col].line_width(line)
    }

    fn get_width(&self, pos: Position) -> usize {
        self[pos.row][pos.col].width()
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
