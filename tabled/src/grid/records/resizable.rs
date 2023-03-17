use papergrid::config::Position;

#[cfg(feature = "std")]
use crate::grid::records::vec_records::VecRecords;

/// A records representation which can be modified by moving rows/columns around.
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
    /// Inserts a row at index.
    fn insert_row(&mut self, row: usize);
    /// Inserts column at index.
    fn insert_column(&mut self, column: usize);
}

impl<T> Resizable for &'_ mut T
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

    fn insert_column(&mut self, column: usize) {
        T::insert_column(self, column)
    }
}

#[cfg(feature = "std")]
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
        let _ = self.remove(row);
    }

    fn remove_column(&mut self, column: usize) {
        for row in self.iter_mut() {
            let _ = row.remove(column);
        }
    }

    fn insert_row(&mut self, row: usize) {
        let count_columns = self.get(0).map(|l| l.len()).unwrap_or(0);
        self.insert(row, vec![T::default(); count_columns]);
    }

    fn insert_column(&mut self, column: usize) {
        for row in self {
            row.insert(column, T::default());
        }
    }
}

#[cfg(feature = "std")]
impl<T> Resizable for VecRecords<T>
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
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        let count_columns = data.get(0).map(|l| l.len()).unwrap_or(0);
        data.push(vec![T::default(); count_columns]);

        *self = VecRecords::new(data);
    }

    fn push_column(&mut self) {
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        for row in &mut data {
            row.push(T::default());
        }

        *self = VecRecords::new(data);
    }

    fn remove_row(&mut self, row: usize) {
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        let _ = data.remove(row);

        *self = VecRecords::new(data);
    }

    fn remove_column(&mut self, column: usize) {
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        for row in &mut data {
            let _ = row.remove(column);
        }

        *self = VecRecords::new(data);
    }

    fn insert_row(&mut self, row: usize) {
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        let count_columns = data.get(0).map(|l| l.len()).unwrap_or(0);
        data.insert(row, vec![T::default(); count_columns]);

        *self = VecRecords::new(data);
    }

    fn insert_column(&mut self, column: usize) {
        let records = std::mem::replace(self, VecRecords::new(vec![]));
        let mut data: Vec<Vec<_>> = records.into();

        for row in &mut data {
            row.insert(column, T::default());
        }

        *self = VecRecords::new(data);
    }
}
