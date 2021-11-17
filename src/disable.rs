#[allow(unused)]
use crate::Table;
use crate::{bounds_to_usize, TableOption};
use papergrid::{Entity, Grid};
use std::ops::RangeBounds;

/// Disable removes particular rows/columns from a [Table].
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

                for row in 0..=new_grid.count_rows() {
                    if grid.is_horizontal_split_set(row) {
                        new_grid.add_horizontal_split(row);
                    }
                }

                let mut new_column_index = 0;
                for column in 0..=grid.count_columns() {
                    let is_column_deleted = column >= x && column < y;
                    if is_column_deleted {
                        continue;
                    }

                    if grid.is_vertical_split_set(column) {
                        new_grid.add_vertical_split(new_column_index);
                    }

                    new_column_index += 1;
                }

                for row in 0..grid.count_rows() {
                    let mut new_column_index = 0;
                    for column in 0..grid.count_columns() {
                        let is_column_deleted = column >= x && column < y;
                        if is_column_deleted {
                            continue;
                        }

                        let cell_settings = grid.get_settings(row, column);
                        new_grid.set(&Entity::Cell(row, new_column_index), cell_settings);
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
                println!("{:?} {} {}", (x, y), removal_size, new_row_size);
                let mut new_grid = Grid::new(new_row_size, grid.count_columns());

                for column in 0..=new_grid.count_columns() {
                    if grid.is_vertical_split_set(column) {
                        new_grid.add_vertical_split(column);
                    }
                }

                let mut new_row_index = 0;
                for row in 0..=grid.count_rows() {
                    let is_row_deleted = row >= x && row < y;
                    if is_row_deleted {
                        continue;
                    }

                    if grid.is_horizontal_split_set(row) {
                        new_grid.add_horizontal_split(new_row_index);
                    }

                    new_row_index += 1;
                }

                for column in 0..grid.count_columns() {
                    let mut new_row_index = 0;
                    for row in 0..grid.count_rows() {
                        let is_row_deleted = row >= x && row < y;
                        println!("{} {} {}", row, x, y);
                        if is_row_deleted {
                            continue;
                        }

                        let cell_settings = grid.get_settings(row, column);
                        new_grid.set(&Entity::Cell(new_row_index, column), cell_settings);
                        new_row_index += 1;
                    }
                }

                *grid = new_grid;
            }
        }
    }
}
