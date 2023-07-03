//! This module contains an [`Dup`] setting the [`Table`].
//!
//! # Example
//!
//! ```
//! # use tabled::{Table, settings::{Dup, object::{Columns, Rows}}};
//! # let data: Vec<&'static str> = Vec::new();
//! let mut table = Table::new(&data);
//! table.with(Dup::new(Rows::first(), Columns::first()));
//! ```
//!
//! [`Table`]: crate::Table

use papergrid::config::Position;

use crate::{
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{object::Object, TableOption},
};

/// [`Dup`] duplicates a given set of cells into another set of ones [`Table`].
///
/// # Example
///
/// ```
/// use tabled::{Table, settings::{object::Rows, Dup}};
///
/// let data = [
///     ["1", "2", "3"],
///     ["Some\nMulti\nLine\nText", "and a line", "here"],
///     ["4", "5", "6"],
/// ];
///
/// let mut table = Table::new(&data);
/// table.with(Dup::new(Rows::single(1), Rows::single(2)));
///
/// assert_eq!(
///     table.to_string(),
///     "+-------+------------+------+\n\
///      | 0     | 1          | 2    |\n\
///      +-------+------------+------+\n\
///      | Some  | and a line | here |\n\
///      | Multi |            |      |\n\
///      | Line  |            |      |\n\
///      | Text  |            |      |\n\
///      +-------+------------+------+\n\
///      | Some  | and a line | here |\n\
///      | Multi |            |      |\n\
///      | Line  |            |      |\n\
///      | Text  |            |      |\n\
///      +-------+------------+------+\n\
///      | 4     | 5          | 6    |\n\
///      +-------+------------+------+",
/// )
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dup<Dst, Src> {
    src: Src,
    dst: Dst,
}

impl<Dst, Src> Dup<Dst, Src> {
    /// New creates a new [`Dup`] modifier.
    ///
    /// # Example
    ///
    /// ```
    /// # use tabled::{Table, settings::{Dup, object::{Columns, Rows}}};
    /// # let data: Vec<&'static str> = Vec::new();
    /// let mut table = Table::new(&data);
    /// table.with(Dup::new(Rows::first(), Columns::last()));
    /// ```
    pub fn new(dst: Dst, src: Src) -> Self {
        Self { src, dst }
    }
}

impl<Dst, Src, R, D, C> TableOption<R, D, C> for Dup<Dst, Src>
where
    Dst: Object<R>,
    Src: Object<R>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let input = collect_input(records, self.src);
        set_cells(records, &input, self.dst);
    }
}

fn collect_input<R, O>(records: &mut R, src: O) -> Vec<String>
where
    O: Object<R>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    let count_rows = records.count_rows();
    let count_columns = records.count_columns();

    let mut input = Vec::new();
    for entity in src.cells(records) {
        for pos in entity.iter(count_rows, count_columns) {
            if !is_valid_cell(pos, count_rows, count_columns) {
                continue;
            }

            let text = records.get_text(pos).to_owned();
            input.push(text);
        }
    }

    input
}

fn set_cells<R, O>(records: &mut R, src: &[String], dst: O)
where
    O: Object<R>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    if src.is_empty() {
        return;
    }

    let count_rows = records.count_rows();
    let count_columns = records.count_columns();

    for entity in dst.cells(records) {
        let mut source = src.iter().cycle();
        for pos in entity.iter(count_rows, count_columns) {
            if !is_valid_cell(pos, count_rows, count_columns) {
                continue;
            }

            let text = source.next().unwrap().clone();
            records.set(pos, text);
        }
    }
}

fn is_valid_cell((row, col): Position, count_rows: usize, count_columns: usize) -> bool {
    if row > count_rows {
        return false;
    }

    if col > count_columns {
        return false;
    }

    true
}
