//! This module contains a [`Concat`] primitive which can be in order to combine 2 [`Table`]s into 1.
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
//! let mut table = Table::new(messages);
//! table
//!     .with(Concat::horizontal(Table::new(departments)))
//!     .with(Style::extended());
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "╔════╦════════════════════╦════════════╗\n",
//!         "║ id ║ text               ║ department ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║ 0  ║ Hello World        ║ Admins     ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║ 1  ║ Do do do something ║ DevOps     ║\n",
//!         "╠════╬════════════════════╬════════════╣\n",
//!         "║    ║                    ║ R&D        ║\n",
//!         "╚════╩════════════════════╩════════════╝",
//!     )
//! )
//! ```

use papergrid::{config::Position, GridConfig};

use crate::{
    records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
    Table,
};

/// [`Concat`] concatenate tables along a particular axis [Horizontal | Vertical].
/// It doesn't do any key or column comparisons like SQL's join does.
///
/// When the tables has different sizes, empty cells will be created by default.
///
/// [`Concat`] in horizontal mode has similar behaiviour to tuples `(a, b)`.
/// But it behaives on tables rather than on an actual data.
///
/// ```
/// use tabled::{TableIteratorExt, Concat};
/// let table1 = [0, 1, 2, 3].table();
/// let table2 = ["A", "B", "C", "D"].table();
///
/// let mut table3 = table1;
/// table3.with(Concat::horizontal(table2));
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

impl<R, D> TableOption<R, D> for Concat
where
    R: Records + ExactRecords + Resizable + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        let rhs = &mut self.table;
        match self.mode {
            ConcatMode::Horizontal => {
                for _ in 0..rhs.count_columns() {
                    records.push_column();
                }

                for row in count_rows..rhs.count_rows() {
                    records.push_row();

                    for col in 0..records.count_columns() {
                        records.set((row, col), self.default_cell.clone());
                    }
                }

                for row in 0..rhs.shape().0 {
                    for col in 0..rhs.shape().1 {
                        let mut cell_text = GetCell(String::new(), (row, col));
                        rhs.with(&mut cell_text);
                        let text = cell_text.0;

                        let col = col + count_cols;
                        records.set((row, col), text);
                    }
                }
            }
            ConcatMode::Vertical => {
                for _ in 0..rhs.count_rows() {
                    records.push_row();
                }

                for col in count_cols..rhs.shape().1 {
                    records.push_column();

                    for row in 0..records.count_rows() {
                        records.set((row, col), self.default_cell.clone());
                    }
                }

                for row in 0..rhs.shape().0 {
                    for col in 0..rhs.shape().1 {
                        let mut cell_text = GetCell(String::new(), (row, col));
                        rhs.with(&mut cell_text);
                        let text = cell_text.0;

                        let row = row + count_rows;
                        records.set((row, col), text);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct GetCell(String, Position);

impl<R, D> TableOption<R, D> for GetCell
where
    R: ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        let cell = records.get_cell(self.1).as_ref().to_string();
        self.0 = cell;
    }
}
