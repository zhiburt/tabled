use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};
use std::cmp;

/// Concat concatenate tables along a particular axis [Horizontal | Vertical].
/// It doesn't do any key or column comparisions like SQL's join does.
///
/// When the tables has different sizes, empty cells will be created by default.
///
/// [Concat] in horizontal mode has simmilar behaiviour to tuples `(a, b)`.
/// But it behaives on tables rather than on an actuall data.
///
/// ```
/// use tabled::{TableIteratorExt, concat::Concat};
/// let table1 = [0, 1, 2, 3].table();
/// let table2 = ["A", "B", "C", "D"].table();
///
/// let table3 = table1.with(Concat::horizontal(table2));
/// ```
pub struct Concat {
    table: Table,
    mode: ConcatMode,
    default_cell: String,
}
enum ConcatMode {
    Vertical,
    Horizontal,
}

impl Concat {
    fn new(table: Table, mode: ConcatMode) -> Self {
        Self {
            table,
            mode,
            default_cell: String::new(),
        }
    }
    /// Concatenate 2 tables horizontally (along axis=0)
    pub fn vertical(table: Table) -> Self {
        Self::new(table, ConcatMode::Vertical)
    }

    /// Concatenate 2 tables vertically (along axis=1)
    pub fn horizontal(table: Table) -> Self {
        Self::new(table, ConcatMode::Horizontal)
    }

    /// Sets a cell's content for cases where 2 tables has different sizes.
    pub fn default_cell(mut self, cell: impl Into<String>) -> Self {
        self.default_cell = cell.into();
        self
    }
}

impl TableOption for Concat {
    fn change(&mut self, other: &mut Grid) {
        match &mut self.mode {
            ConcatMode::Vertical => {
                let new_row_size = self.table.grid.count_rows() + other.count_rows();
                let new_column_size =
                    cmp::max(self.table.grid.count_columns(), other.count_columns());
                let mut new_grid = Grid::new(new_row_size, new_column_size);

                for row in 0..other.count_rows() {
                    for column in 0..other.count_columns() {
                        let settings = other.get_settings(row, column);
                        new_grid.set(
                            &Entity::Cell(row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                for row in 0..self.table.grid.count_rows() {
                    for column in 0..self.table.grid.count_columns() {
                        let settings = self.table.grid.get_settings(row, column);
                        new_grid.set(
                            &Entity::Cell(other.count_rows() + row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                for row in 0..new_grid.count_rows() {
                    for column in 0..new_grid.count_columns() {
                        if (column >= other.count_columns() && row < other.count_rows())
                            || (column >= self.table.grid.count_columns()
                                && row > other.count_rows())
                        {
                            let settings =
                                new_grid.get_settings(row, column).text(&self.default_cell);
                            new_grid.set(&Entity::Cell(row, column), settings);
                        }
                    }
                }

                *other = new_grid;
            }
            ConcatMode::Horizontal => {
                let new_row_size = cmp::max(self.table.grid.count_rows(), other.count_rows());
                let new_column_size = self.table.grid.count_columns() + other.count_columns();
                let mut new_grid = Grid::new(new_row_size, new_column_size);

                for column in 0..other.count_columns() {
                    for row in 0..other.count_rows() {
                        let settings = other.get_settings(row, column);
                        new_grid.set(
                            &Entity::Cell(row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                for column in 0..self.table.grid.count_columns() {
                    for row in 0..self.table.grid.count_rows() {
                        let settings = self.table.grid.get_settings(row, column);
                        new_grid.set(
                            &Entity::Cell(row, column + other.count_columns()),
                            settings.border_restriction(false),
                        );
                    }
                }

                for row in 0..new_grid.count_rows() {
                    for column in 0..new_grid.count_columns() {
                        if (row >= other.count_rows() && column < other.count_columns())
                            || (row >= self.table.grid.count_rows()
                                && column > other.count_columns())
                        {
                            let settings =
                                new_grid.get_settings(row, column).text(&self.default_cell);
                            new_grid.set(&Entity::Cell(row, column), settings);
                        }
                    }
                }

                *other = new_grid;
            }
        }
    }
}
