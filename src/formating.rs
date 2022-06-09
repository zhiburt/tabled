//! This module contains a list of primitives to help to modify a [Table].
//!
//! [Table]: crate::Table

use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

/// Formatting function of particular cells on a [Table].
///
/// [Table]: crate::Table
pub struct Format<F> {
    f: F,
}

impl Format<()> {
    /// This function creates a new [Format] instance, so
    /// it can be used as a grid setting.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Format, object::Rows, Modify};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::new(1..)).with(Format::new(|s| format!(": {} :", s))))
    ///                .to_string();
    ///
    /// assert_eq!(table, "+-------+-------------+-----------+\n\
    ///                    |  i32  |    &str     |   bool    |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 0 : | : Grodno :  | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 1 : |  : Minsk :  | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 2 : | : Hamburg : | : false : |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 3 : |  : Brest :  | : true :  |\n\
    ///                    +-------+-------------+-----------+\n");
    /// ```
    ///
    pub fn new<F>(f: F) -> Format<F>
    where
        F: FnMut(&str) -> String,
    {
        Format { f }
    }

    /// This function creates a new [FormatWithIndex], so
    /// it can be used as a grid setting.
    ///
    /// It's different from [Self::new] that it also provides a row and column index.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, Format, object::Rows, Modify};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::single(0)).with(Format::with_index(|_, (_, column)| column.to_string())))
    ///                .to_string();
    ///
    /// assert_eq!(table, "+---+---------+-------+\n\
    ///                    | 0 |    1    |   2   |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 0 | Grodno  | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 1 |  Minsk  | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 2 | Hamburg | false |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 3 |  Brest  | true  |\n\
    ///                    +---+---------+-------+\n");
    /// ```
    pub fn with_index<F>(f: F) -> FormatWithIndex<F>
    where
        F: FnMut(&str, (usize, usize)) -> String,
    {
        FormatWithIndex::new(f)
    }

    /// Multiline a helper function for changing multiline content of cell.
    /// Using this formatting applied for all rows not to a string as a whole.
    ///
    /// ```rust,no_run
    /// use tabled::{Table, Format, object::Segment, Modify};
    /// let data: Vec<&'static str> = Vec::new();
    /// let table = Table::new(&data)
    ///     .with(Modify::new(Segment::all()).with(Format::multiline(|s| format!("{}", s))))
    ///     .to_string();
    /// ```
    pub fn multiline<F>(f: F) -> Format<impl Fn(&str) -> String>
    where
        F: Fn(&str) -> String,
    {
        let closure = move |s: &str| {
            let mut v = Vec::new();
            for line in s.lines() {
                v.push(f(line));
            }

            v.join("\n")
        };

        Format::new(closure)
    }
}

impl<F> CellOption for Format<F>
where
    F: FnMut(&str) -> String,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self.f)(content);
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}

/// FormatWithIndex is like a [Format] an abstraction over a function you can use against a cell.
///
/// It differerent from Format that it provides a row and column index.
pub struct FormatWithIndex<F> {
    f: F,
}

impl<F> FormatWithIndex<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
{
    fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> CellOption for FormatWithIndex<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self.f)(content, (row, column));
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}

impl<F> CellOption for F
where
    F: FnMut(&str) -> String,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self)(content);
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}
