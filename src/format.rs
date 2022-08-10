//! This module contains a list of primitives to help to modify a [`Table`].
//!
//! [`Table`]: crate::Table

use papergrid::{
    records::{Records, RecordsMut},
    width::CfgWidthFunction,
    Entity,
};

use crate::{CellOption, Table};

/// A formatting function of particular cells on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Format<F> {
    f: F,
}

impl Format<()> {
    /// This function creates a new [`Format`] instance, so
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
    ///                    | i32   | &str        | bool      |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 0 : | : Grodno :  | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 1 : | : Minsk :   | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 2 : | : Hamburg : | : false : |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 3 : | : Brest :   | : true :  |\n\
    ///                    +-------+-------------+-----------+");
    /// ```
    ///
    pub fn new<F>(f: F) -> Format<F>
    where
        F: FnMut(&str) -> String,
    {
        Format { f }
    }

    /// This function creates a new [`FormatWithIndex`], so
    /// it can be used as a grid setting.
    ///
    /// It's different from [`Format::new`] as it also provides a row and column index.
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
    ///                    | 0 | 1       | 2     |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 0 | Grodno  | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 1 | Minsk   | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 2 | Hamburg | false |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 3 | Brest   | true  |\n\
    ///                    +---+---------+-------+");
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
    ///
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

impl<F, R> CellOption<R> for Format<F>
where
    F: FnMut(&str) -> String,
    R: RecordsMut,
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let width_fn = CfgWidthFunction::new(table.get_config());
        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let content = records.get_text(pos);
            let content = (self.f)(content);
            table.get_records_mut().set_text(pos, content, &width_fn);
        }

        table.destroy_width_cache();
    }
}

/// [`FormatWithIndex`] is like a [`Format`] an abstraction over a function you can use against a cell.
///
/// It differerent from [`Format`] that it provides a row and column index.
#[derive(Debug)]
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

impl<F, R> CellOption<R> for FormatWithIndex<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
    R: RecordsMut,
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let width_fn = CfgWidthFunction::new(table.get_config());
        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let content = records.get_text(pos);
            let content = (self.f)(content, pos);
            table.get_records_mut().set_text(pos, content, &width_fn);
        }

        table.destroy_width_cache();
    }
}

impl<F, R> CellOption<R> for F
where
    F: FnMut(&str) -> String,
    R: RecordsMut,
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        Format::new(self).change_cell(table, entity);
    }
}
