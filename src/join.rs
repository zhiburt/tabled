#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};
use std::cmp;

pub struct Join {
    pub mode: JoinMode,
    pub strict_size: bool,
}

pub enum JoinMode {
    Vertical(Table),
    Horizontal(Table),
}

impl TableOption for Join {
    fn change(&mut self, other: &mut Grid) {
        match &mut self.mode {
            JoinMode::Vertical(table) if self.strict_size => {
                let new_row_size = table.grid.count_rows() + other.count_rows();
                let mut new_grid = Grid::new(new_row_size, table.grid.count_rows());

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if row < other.count_rows() {
                            other.get_settings(row, column)
                        } else {
                            let row = row - other.count_rows();
                            table.grid.get_settings(row, column)
                        };
                        new_grid.set(
                            &Entity::Cell(row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                *other = new_grid;
            }
            JoinMode::Horizontal(table) if self.strict_size => {
                let new_column_size = table.grid.count_columns() + other.count_columns();
                let mut new_grid = Grid::new(table.grid.count_rows(), new_column_size);

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if column < other.count_columns() {
                            other.get_settings(row, column)
                        } else {
                            let column = column - other.count_columns();
                            table.grid.get_settings(row, column)
                        };
                        new_grid.set(
                            &Entity::Cell(row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                *other = new_grid;
            }
            JoinMode::Vertical(table) => {
                let new_row_size = table.grid.count_rows() + other.count_rows();
                let new_column_size = cmp::max(table.grid.count_columns(), other.count_columns());
                let mut new_grid = Grid::new(new_row_size, new_column_size);

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if row < other.count_rows() {
                            other.get_settings(row, column)
                        } else {
                            let row = row - other.count_rows();
                            table.grid.get_settings(row, column)
                        };
                        new_grid.set(
                            &Entity::Cell(row, column),
                            settings.border_restriction(false),
                        );
                    }
                }

                *other = new_grid;
            }
            JoinMode::Horizontal(table) => {
                let new_row_size = cmp::max(table.grid.count_rows(), other.count_rows());
                let new_column_size = table.grid.count_columns() + other.count_columns();
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

                for column in 0..table.grid.count_columns() {
                    for row in 0..table.grid.count_rows() {
                        let settings = table.grid.get_settings(row, column);
                        new_grid.set(
                            &Entity::Cell(row, other.count_columns() + column),
                            settings.border_restriction(false),
                        );
                    }
                }

                *other = new_grid;
            }
        }
    }
}
