//! This module contains a [Disable] structure which helps to
//! remove an etheir column or row from a [Table].
//!
//! Essentially it is better to provide a correct data initially and not use disable.
//!
//! # Example
//!
//! ```
//! use tabled::{Disable, TableIteratorExt};
//!
//! let data = vec!["Hello", "World", "!!!"];
//!
//! let table = data.table().with(Disable::Row(1..2)).to_string();
//!
//! assert_eq!(
//!     table,
//!     "+-------+\n\
//!      | &str  |\n\
//!      +-------+\n\
//!      | World |\n\
//!      +-------+\n\
//!      |  !!!  |\n\
//!      +-------+\n"
//! );
//! ```

#[allow(unused)]
use crate::Table;
use crate::{object::bounds_to_usize, TableOption};
use papergrid::{Entity, Grid};
use std::ops::RangeBounds;

/// Disable removes particular rows/columns from a [Table].
///
/// It tries to keeps track of style changes which may occur.
/// But it's not guaranteed will be the way you would expect it to be.
///
/// # Example
///
/// ```rust,no_run
///   # use tabled::{Disable, Table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data).with(Disable::Row(..1));
/// ```
#[derive(Debug)]
pub enum Disable<R: RangeBounds<usize>> {
    /// Columns of the grid.
    /// Range is used to locate columns.
    Column(R),
    /// Rows of the grid.
    /// Range is used to locate rows.
    Row(R),
}

impl<R: RangeBounds<usize>> TableOption for Disable<R> {
    fn change(&mut self, grid: &mut Grid) {
        match self {
            Self::Column(range) => {
                let (x, y) =
                    bounds_to_usize(range.start_bound(), range.end_bound(), grid.count_columns());

                let removal_size = y - x;
                let new_column_size = grid.count_columns() - removal_size;
                let mut new_grid = Grid::new(grid.count_rows(), new_column_size);

                for row in 0..grid.count_rows() {
                    let mut new_column_index = 0;
                    for column in 0..grid.count_columns() {
                        let is_column_deleted = column >= x && column < y;
                        if is_column_deleted {
                            continue;
                        }

                        let cell_settings =
                            grid.get_settings(row, column).border_restriction(false);
                        new_grid.set(Entity::Cell(row, new_column_index), cell_settings);
                        new_column_index += 1;
                    }
                }

                *grid = new_grid;
            }
            Self::Row(range) => {
                let (x, y) =
                    bounds_to_usize(range.start_bound(), range.end_bound(), grid.count_rows());

                let removal_size = y - x;
                let new_row_size = grid.count_rows() - removal_size;
                let mut new_grid = Grid::new(new_row_size, grid.count_columns());

                for column in 0..grid.count_columns() {
                    let mut new_row_index = 0;
                    for row in 0..grid.count_rows() {
                        let is_row_deleted = row >= x && row < y;
                        if is_row_deleted {
                            continue;
                        }

                        let cell_settings =
                            grid.get_settings(row, column).border_restriction(false);
                        new_grid.set(Entity::Cell(new_row_index, column), cell_settings);
                        new_row_index += 1;
                    }
                }

                *grid = new_grid;
            }
        }
    }
}
