//! This module contains a [Concat] primitive which can be in order to combine 2 [Table]s into 1.
//!
//! # Example
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Table, Tabled, Style, Concat};
//!
//! #[derive(Tabled)]
//! struct Message {
//!     id: &'static str,
//!     text: &'static str,
//! }
//!
//! #[derive(Tabled)]
//! struct Department(#[tabled(rename = "department")] &'static str);
//!
//! let messages = [
//!     Message { id: "0", text: "Hello World" },
//!     Message { id: "1", text: "Do do do something", },
//! ];
//!
//! let departments = [
//!     Department("Admins"),
//!     Department("DevOps"),
//!     Department("R&D"),
//! ];
//!
//! let table = Table::new(messages)
//!     .with(Concat::horizontal(Table::new(departments)))
//!     .with(Style::extended());
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "╔════╦════════════════════╦════════════╗\n",
//!         "║ id ║        text        ║ department ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║ 0  ║    Hello World     ║   Admins   ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║ 1  ║ Do do do something ║   DevOps   ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║    ║                    ║    R&D     ║\n",
//!         "╚════╩════════════════════╩════════════╝",
//!     )
//! )
//! ```

use std::cmp;

use papergrid::{Border, Entity, Grid};

use crate::{Table, TableOption};

/// Concat concatenate tables along a particular axis [Horizontal | Vertical].
/// It doesn't do any key or column comparisons like SQL's join does.
///
/// When the tables has different sizes, empty cells will be created by default.
///
/// [Concat] in horizontal mode has similar behaiviour to tuples `(a, b)`.
/// But it behaives on tables rather than on an actual data.
///
/// ```
/// use tabled::{TableIteratorExt, Concat};
/// let table1 = [0, 1, 2, 3].table();
/// let table2 = ["A", "B", "C", "D"].table();
///
/// let table3 = table1.with(Concat::horizontal(table2));
/// ```
#[derive(Debug)]
pub struct Concat {
    table: Table,
    mode: ConcatMode,
    default_cell: String,
}
#[derive(Debug)]
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
    fn change(&mut self, lhs: &mut Grid) {
        let rhs = &self.table.grid;

        match &mut self.mode {
            ConcatMode::Vertical => {
                let new_row_size = lhs.count_rows() + rhs.count_rows();
                let new_column_size = cmp::max(lhs.count_columns(), rhs.count_columns());
                let mut new_grid = Grid::new(new_row_size, new_column_size);
                new_grid.set_borders(lhs.get_borders().clone());

                for row in 0..new_grid.count_rows() {
                    for column in 0..new_grid.count_columns() {
                        let is_lhs_side = row < lhs.count_rows();
                        let is_rhs_side = row >= lhs.count_rows();

                        let is_new_to_lhs = column >= rhs.count_columns() && is_rhs_side;
                        let is_new_to_rhs = column >= lhs.count_columns() && is_lhs_side;
                        let is_new_cell = is_new_to_lhs || is_new_to_rhs;

                        let settings = if is_new_cell {
                            new_grid.get_settings(row, column).text(&self.default_cell)
                        } else if is_lhs_side {
                            lhs.get_settings(row, column)
                        } else {
                            rhs.get_settings(row - lhs.count_rows(), column)
                                .border(Border::default())
                        };

                        new_grid.set(Entity::Cell(row, column), settings);
                    }
                }

                *lhs = new_grid;
            }
            ConcatMode::Horizontal => {
                let new_row_size = cmp::max(lhs.count_rows(), rhs.count_rows());
                let new_column_size = lhs.count_columns() + rhs.count_columns();
                let mut new_grid = Grid::new(new_row_size, new_column_size);
                new_grid.set_borders(lhs.get_borders().clone());

                for row in 0..new_grid.count_rows() {
                    for column in 0..new_grid.count_columns() {
                        let is_lhs_side = column < lhs.count_columns();
                        let is_rhs_side = column >= lhs.count_columns();

                        let is_new_to_lhs = row >= rhs.count_rows() && is_rhs_side;
                        let is_new_to_rhs = row >= lhs.count_rows() && is_lhs_side;
                        let is_new_cell = is_new_to_lhs || is_new_to_rhs;

                        let settings = if is_new_cell {
                            new_grid.get_settings(row, column).text(&self.default_cell)
                        } else if is_lhs_side {
                            lhs.get_settings(row, column)
                        } else {
                            rhs.get_settings(row, column - lhs.count_columns())
                                .border(Border::default())
                        };

                        new_grid.set(Entity::Cell(row, column), settings);
                    }
                }

                *lhs = new_grid;
            }
        }
    }
}
