#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};

pub enum Join {
    VerticalTop(Table),
    VerticalBottom(Table),
    HorizontalLeft(Table),
    HorizontalRight(Table),
}

impl TableOption for Join {
    fn change(&mut self, other: &mut Grid) {
        match self {
            Join::VerticalTop(table) => {
                let new_row_size = table.grid.count_rows() + other.count_rows();
                let mut new_grid = Grid::new(new_row_size,table.grid.count_rows());

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if row < table.grid.count_rows() {
                            table.grid.get_settings(row, column)
                        } else {
                            let row = row - table.grid.count_rows();
                            other.get_settings(row, column)
                        };
                        new_grid.set(&Entity::Cell(row,column),settings.border_restriction(false));
                    }
                }
            
                *other = new_grid;
            },
            Join::VerticalBottom(table) => {
                let new_row_size = table.grid.count_rows() + other.count_rows();
                let mut new_grid = Grid::new(new_row_size,table.grid.count_rows());

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if row < other.count_rows() {
                            other.get_settings(row, column)
                        } else {
                            let row = row - other.count_rows();
                            table.grid.get_settings(row, column)
                        };
                        new_grid.set(&Entity::Cell(row,column),settings.border_restriction(false));
                    }
                }

                *other = new_grid;
            },
            Join::HorizontalLeft(table) => {
                let new_column_size = table.grid.count_columns() + other.count_columns();
                let mut new_grid = Grid::new(table.grid.count_rows(),new_column_size);

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if column < table.grid.count_columns() {
                            table.grid.get_settings(row, column)
                        } else {
                            let column = column - table.grid.count_columns();
                            other.get_settings(row, column)
                        };
                        new_grid.set(&Entity::Cell(row,column),settings.border_restriction(false));
                    }
                }
            
                *other = new_grid;
            },
            Join::HorizontalRight(table) => {
                let new_column_size = table.grid.count_columns() + other.count_columns();
                let mut new_grid = Grid::new(table.grid.count_rows(),new_column_size);

                for column in 0..new_grid.count_columns() {
                    for row in 0..new_grid.count_rows() {
                        let settings = if column < other.count_columns() {
                            other.get_settings(row, column)
                        } else {
                            let column = column - other.count_columns();
                            table.grid.get_settings(row, column)
                        };
                        new_grid.set(&Entity::Cell(row,column),settings.border_restriction(false));
                    }
                }
            
                *other = new_grid;
            }
        }
    }
}
