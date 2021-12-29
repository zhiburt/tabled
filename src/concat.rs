use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};
use std::cmp;

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

    pub fn vertical(table: Table) -> Self {
        Self::new(table, ConcatMode::Vertical)
    }

    pub fn horizontal(table: Table) -> Self {
        Self::new(table, ConcatMode::Horizontal)
    }

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
