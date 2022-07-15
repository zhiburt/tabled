//! This module contains a [Rotate] primitive which can be used in order to rotate [Table].
//!
//! It's also possible to transpose the table at the point of construction.
//! See [`Builder::index`].
//!
//! # Example
//!
//! ```
//! use tabled::{Rotate, TableIteratorExt};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table().with(Rotate::Left).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 2 | 3 | 6 |\n",
//!         "+---+---+---+\n",
//!         "| 1 | 2 | 5 |\n",
//!         "+---+---+---+\n",
//!         "| 0 | 1 | 4 |\n",
//!         "+---+---+---+",
//!     )
//! );
//! ```
//!
//! [Table]: crate::Table
//! [Builder::index]: crate::builder::Builder::index

use papergrid::{Entity, Grid};

use crate::TableOption;

/// Rotate can be used to rotate a table by 90 degrees.
#[derive(Debug)]
pub enum Rotate {
    /// Rotatate [crate::Table] to the left.
    Left,
    /// Rotatate [crate::Table] to the right.
    Right,
    /// Rotatate [crate::Table] to the top.
    ///
    /// So the top becames the bottom.
    Top,
    /// Rotatate [crate::Table] to the bottom.
    ///
    /// So the top becames the bottom.
    Bottom,
}

impl TableOption for Rotate {
    fn change(&mut self, grid: &mut Grid) {
        match self {
            Self::Left => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                new.set_borders(grid.get_borders().clone());
                for row in 0..grid.count_rows() {
                    for (lhs_column, rhs_column) in
                        (0..grid.count_columns()).zip((0..grid.count_columns()).rev())
                    {
                        let settings = grid.get_settings(row, lhs_column);
                        new.set(Entity::Cell(rhs_column, row), settings);
                    }
                }

                *grid = new;
            }
            Self::Right => {
                let mut new = Grid::new(grid.count_columns(), grid.count_rows());
                new.set_borders(grid.get_borders().clone());
                let mut last_row = grid.count_rows();
                for row in 0..grid.count_rows() {
                    last_row -= 1;
                    for column in 0..grid.count_columns() {
                        let border = grid.get_settings(row, column);
                        new.set(Entity::Cell(column, last_row), border);
                    }
                }

                *grid = new;
            }
            Self::Bottom => {
                let mut new = Grid::new(grid.count_rows(), grid.count_columns());
                new.set_borders(grid.get_borders().clone());
                for column in 0..grid.count_columns() {
                    for row in 0..grid.count_rows() {
                        let last_row = grid.count_rows() - 1 - row;
                        let border = grid.get_settings(row, column);
                        new.set(Entity::Cell(last_row, column), border);
                    }
                }

                *grid = new;
            }
            Self::Top => Self::Bottom.change(grid),
        }
    }
}
