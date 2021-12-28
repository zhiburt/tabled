use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};
use std::cmp;

pub struct Join {
    table: Table,
    mode: JoinMode,
    strict_size: bool,
}
enum JoinMode {
    Vertical,
    Horizontal,
}

impl Join {
    fn new(table: Table, mode: JoinMode, strict_size: bool) -> Self {
        Self {
            table,
            mode,
            strict_size,
        }
    }

    pub fn vertical(table: Table) -> Self {
        Self::new(table, JoinMode::Vertical, false)
    }

    pub fn horizontal(table: Table) -> Self {
        Self::new(table, JoinMode::Horizontal, false)
    }

    pub fn strict_size(mut self) -> Self {
        self.strict_size = true;
        self
    }
}

impl TableOption for Join {
    fn change(&mut self, other: &mut Grid) {
        match &mut self.mode {
            JoinMode::Vertical => {
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

                *other = new_grid;
            }
            JoinMode::Horizontal => {
                let new_row_size = cmp::max(self.table.grid.count_rows(), other.count_rows());
                let new_column_size = self.table.grid.count_columns() + other.count_columns();
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
                            &Entity::Cell(row, column + other.count_columns()),
                            settings.border_restriction(false),
                        );
                    }
                }

                *other = new_grid;
            }
        }
    }
}
