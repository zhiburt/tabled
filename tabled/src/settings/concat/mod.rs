//! This module contains a [`Concat`] primitive which can be in order to combine 2 [`Table`]s into 1.
//!
//! # Example
//!
//! ```
//! use tabled::{Table, settings::Concat};
//! let table1 = Table::new([0, 1, 2, 3]);
//! let table2 = Table::new(["A", "B", "C", "D"]);
//!
//! let mut table3 = table1;
//! table3.with(Concat::horizontal(table2));
//! ```

use std::borrow::Cow;

use crate::{
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
    Table,
};

/// [`Concat`] concatenates tables along a particular axis [Horizontal | Vertical].
/// It doesn't do any key or column comparisons like SQL's join does.
///
/// When the tables has different sizes, empty cells will be created by default.
///
/// [`Concat`] in horizontal mode has similar behaviour to tuples `(a, b)`.
/// But it behaves on tables rather than on an actual data.
///
/// # Example
///
///
#[cfg_attr(feature = "derive", doc = "```")]
#[cfg_attr(not(feature = "derive"), doc = "```ignore")]
/// use tabled::{Table, Tabled, settings::{Style, Concat}};
///
/// #[derive(Tabled)]
/// struct Message {
///     id: &'static str,
///     text: &'static str,
/// }
///
/// #[derive(Tabled)]
/// struct Department(#[tabled(rename = "department")] &'static str);
///
/// let messages = [
///     Message { id: "0", text: "Hello World" },
///     Message { id: "1", text: "Do do do something", },
/// ];
///
/// let departments = [
///     Department("Admins"),
///     Department("DevOps"),
///     Department("R&D"),
/// ];
///
/// let mut table = Table::new(messages);
/// table
///     .with(Concat::horizontal(Table::new(departments)))
///     .with(Style::extended());
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "╔════╦════════════════════╦════════════╗\n",
///         "║ id ║ text               ║ department ║\n",
///         "╠════╬════════════════════╬════════════╣\n",
///         "║ 0  ║ Hello World        ║ Admins     ║\n",
///         "╠════╬════════════════════╬════════════╣\n",
///         "║ 1  ║ Do do do something ║ DevOps     ║\n",
///         "╠════╬════════════════════╬════════════╣\n",
///         "║    ║                    ║ R&D        ║\n",
///         "╚════╩════════════════════╩════════════╝",
///     )
/// )
/// ```

#[derive(Debug)]
pub struct Concat {
    table: Table,
    mode: ConcatMode,
    default_cell: Cow<'static, str>,
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
            default_cell: Cow::Borrowed(""),
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
    pub fn default_cell(mut self, cell: impl Into<Cow<'static, str>>) -> Self {
        self.default_cell = cell.into();
        self
    }
}

impl<R, D, C> TableOption<R, D, C> for Concat
where
    R: Records + ExactRecords + Resizable + PeekableRecords + RecordsMut<String>,
{
    fn change(mut self, records: &mut R, _: &mut C, _: &mut D) {
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
                        records.set((row, col), self.default_cell.to_string());
                    }
                }

                for row in 0..rhs.shape().0 {
                    for col in 0..rhs.shape().1 {
                        let text = rhs.get_records().get_text((row, col)).to_string();
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
                        records.set((row, col), self.default_cell.to_string());
                    }
                }

                for row in 0..rhs.shape().0 {
                    for col in 0..rhs.shape().1 {
                        let text = rhs.get_records().get_text((row, col)).to_string();
                        let row = row + count_rows;
                        records.set((row, col), text);
                    }
                }
            }
        }
    }
}
