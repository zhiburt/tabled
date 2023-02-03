use std::mem;

use crate::{
    grid::config::Position,
    records::{ExactRecords, IntoRecords, Records, Resizable},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecRecords<T> {
    data: Vec<Vec<T>>,
    shape: (usize, usize),
}

impl<T> VecRecords<T> {
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

impl<T> ExactRecords for VecRecords<T>
where
    T: AsRef<str>,
{
    type Cell = T;

    fn get_cell(&self, pos: Position) -> &Self::Cell {
        &self.data[pos.0][pos.1]
    }

    fn count_rows(&self) -> usize {
        self.shape.0
    }
}

impl<T> Resizable for VecRecords<T>
where
    T: Default + Clone,
{
    fn swap(&mut self, lhs: Position, rhs: Position) {
        if lhs == rhs {
            return;
        }

        let t = mem::take(&mut self.data[lhs.0][lhs.1]);
        let t = mem::replace(&mut self.data[rhs.0][rhs.1], t);
        let _ = mem::replace(&mut self.data[lhs.0][lhs.1], t);
    }

    fn swap_row(&mut self, lhs: usize, rhs: usize) {
        let t = mem::take(&mut self.data[lhs]);
        let t = mem::replace(&mut self.data[rhs], t);
        let _ = mem::replace(&mut self.data[lhs], t);
    }

    fn swap_column(&mut self, lhs: usize, rhs: usize) {
        for row in &mut self.data {
            row.swap(lhs, rhs);
        }
    }

    fn push_row(&mut self) {
        self.shape.0 += 1;
        self.data.push(vec![T::default(); self.shape.1]);
    }

    fn push_column(&mut self) {
        self.shape.1 += 1;

        for row in &mut self.data {
            row.push(T::default());
        }
    }

    fn remove_row(&mut self, row: usize) {
        self.shape.0 -= 1;
        self.data.remove(row);
    }

    fn remove_column(&mut self, column: usize) {
        self.shape.1 -= 1;

        for row in &mut self.data {
            row.remove(column);
        }
    }

    fn insert_row(&mut self, row: usize) {
        self.data.insert(row, vec![T::default(); self.shape.1]);
    }
}
