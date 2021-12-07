#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};

pub enum Join {
    Vertical(Table),
    Horizontal(Table),
}

impl TableOption for Join {
    fn change(&mut self, other: &mut Grid) {
        match self {
            Join::Vertical(table) => {
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
            Join::Horizontal(table) => {
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
            }
        }
    }
}
