#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Entity, Grid};

/// Rotate can be used to rotate a table by 90 degrees.
#[derive(Debug)]
pub enum Rotate {
    Left,
    Right,
    Top,
    Bottom,
}

impl TableOption for Rotate {
    fn change(&mut self, grid: &mut Grid) {
        match self {
            Self::Left => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                for row in 0..grid.count_rows() {
                    for (lhs_column, rhs_column) in
                        (0..grid.count_columns()).zip((0..grid.count_columns()).rev())
                    {
                        let settings = grid.get_settings(row, lhs_column).border_restriction(false);
                        new.set(&Entity::Cell(rhs_column, row), settings)
                    }
                }

                *grid = new;
            }
            Self::Right => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                let mut last_row = grid.count_rows();
                for row in 0..grid.count_rows() {
                    last_row -= 1;
                    for column in 0..grid.count_columns() {
                        let border = grid.get_settings(row, column).border_restriction(false);
                        new.set(&Entity::Cell(column, last_row), border);
                    }
                }

                *grid = new;
            }
            Self::Bottom => {
                let mut new = Grid::new(grid.count_rows(), grid.count_columns());
                for column in 0..grid.count_columns() {
                    for row in 0..grid.count_rows() {
                        let last_row = grid.count_rows() - 1 - row;
                        let border = grid.get_settings(row, column).border_restriction(false);
                        new.set(&Entity::Cell(last_row, column), border)
                    }
                }

                *grid = new;
            }
            Self::Top => Self::Bottom.change(grid),
        }
    }
}
