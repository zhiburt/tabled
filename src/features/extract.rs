//! This module contains an [`Extract`] structure which is used to
//! obtain an ordinary segment from the [`Table`].
//!
//! There's a similar structure [`Highlight`] which does a highlighting a of segments.
//!
//! [`Table`]: crate::Table
//! [`Highlight`]: crate::Highlight

use std::ops::{RangeBounds, RangeFull};

use papergrid::records::{Records, Resizable};

use crate::{object::bounds_to_usize, Table, TableOption};

/// Returns a new [`Table`] that reflects a segment of the referenced [`Table`]
///
/// # Example
///
/// ```
/// use tabled::{Table, format::Format, object::Rows, Modify, Extract};
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
///                .with(Extract::segment(1..=2, 1..))
///                .to_string();
///
/// assert_eq!(table, "+------------+----------+\n\
///                    | : Grodno : | : true : |\n\
///                    +------------+----------+\n\
///                    | : Minsk :  | : true : |\n\
///                    +------------+----------+");
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Extract<R, C> {
    rows: R,
    columns: C,
}

impl<R, C> Extract<R, C>
where
    R: RangeBounds<usize>,
    C: RangeBounds<usize>,
{
    /// Returns a new [`Table`] that reflects a segment of the referenced [`Table`]
    ///
    /// ```rust,no_run
    /// # use tabled::Extract;
    /// let rows = 1..3;
    /// let columns = 1..;
    /// Extract::segment(rows, columns);
    /// ```
    ///
    /// # Range
    ///
    /// A [`RangeBounds`] argument can be less than or equal to the shape of a [`Table`]
    ///
    /// If a [`RangeBounds`] argument is malformed or too large the thread will panic
    ///
    /// ```text
    /// // Empty                         Full                      Out of bounds
    ///    Extract::segment(0..0, 0..0)  Extract::segment(.., ..)  Extract::segment(0..1, ..4)
    ///    [].   .   .                   [O   O   O                [O   O   O  X] //ERROR            
    ///      .   .   .                    O   O   O                 .   .   .             
    ///      .   .   .                    O   O   O]                .   .   .          
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn segment(rows: R, columns: C) -> Self {
        Extract { rows, columns }
    }
}

impl<R> Extract<R, RangeFull>
where
    R: RangeBounds<usize>,
{
    /// Returns a new [`Table`] that reflects a segment of the referenced [`Table`]
    ///
    /// The segment is defined by [`RangeBounds`<usize>] for Rows
    ///
    /// ```rust,no_run
    /// # use tabled::Extract;
    /// Extract::rows(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [`RangeBounds`] argument can be less than or equal to the shape of a [`Table`]
    ///
    /// If a [`RangeBounds`] argument is malformed or too large the thread will panic
    ///
    /// ```text
    /// // Empty                Full               Out of bounds
    ///    Extract::rows(0..0)  Extract::rows(..)  Extract::rows(0..4)
    ///    [].   .   .          [O   O   O         [O   O   O             
    ///      .   .   .           O   O   O          O   O   O              
    ///      .   .   .           O   O   O]         O   O   O
    ///                                             X   X   X] // ERROR          
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn rows(rows: R) -> Self {
        Extract { rows, columns: .. }
    }
}

impl<C> Extract<RangeFull, C>
where
    C: RangeBounds<usize>,
{
    /// Returns a new [`Table`] that reflects a segment of the referenced [`Table`]
    ///
    /// The segment is defined by [`RangeBounds`<usize>] for Columns
    ///
    /// ```rust,no_run
    /// # use tabled::Extract;
    /// Extract::columns(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [`RangeBounds`] argument can be less than or equal to the shape of a [`Table`]
    ///
    /// If a [`RangeBounds`] argument is malformed or too large the thread will panic
    ///
    /// ```text
    /// // Empty                   Full                  Out of bounds
    ///    Extract::columns(0..0)  Extract::columns(..)  Extract::columns(0..4)
    ///    [].   .   .             [O   O   O            [O   O   O   X          
    ///      .   .   .              O   O   O             O   O   O   X          
    ///      .   .   .              O   O   O]            O   O   O   X] // ERROR
    /// ```
    ///
    /// [`Table`]: crate::Table
    pub fn columns(columns: C) -> Self {
        Extract { rows: .., columns }
    }
}

impl<R, C, RR> TableOption<RR> for Extract<R, C>
where
    R: RangeBounds<usize> + Clone,
    C: RangeBounds<usize> + Clone,
    RR: Records + Resizable,
{
    fn change(&mut self, table: &mut Table<RR>) {
        let shape = table.shape();
        let mut rows = bounds_to_usize(self.rows.start_bound(), self.rows.end_bound(), shape.0);
        let mut cols = bounds_to_usize(
            self.columns.start_bound(),
            self.columns.end_bound(),
            shape.1,
        );

        // Cleanup table in case if boundries are exeeded.
        //
        // todo: can be optimized by adding a clear() method to Resizable
        rows.0 = std::cmp::min(rows.0, shape.0);
        cols.0 = std::cmp::min(cols.0, shape.1);

        extract(table.get_records_mut(), shape, rows, cols);
    }
}

/// Returns a new [`Grid`] that reflects a segment of the referenced [`Grid`].
///
/// # Example
///
/// ```text
/// grid
/// +---+---+---+
/// |0-0|0-1|0-2|
/// +---+---+---+
/// |1-0|1-1|1-2|
/// +---+---+---+
/// |2-0|2-1|2-2|
/// +---+---+---+
///
/// let rows = ..;
/// let columns = ..1;
/// grid.extract(rows, columns)
///
/// grid
/// +---+
/// |0-0|
/// +---+
/// |1-0|
/// +---+
/// |2-0|
/// +---+
/// ```
fn extract<R>(
    records: &mut R,
    (count_rows, count_cols): (usize, usize),
    (start_row, end_row): (usize, usize),
    (start_col, end_col): (usize, usize),
) where
    R: Resizable,
{
    for (i, row) in (0..start_row).enumerate() {
        let row = row - i;
        records.remove_row(row);
    }

    let count_rows = count_rows - start_row;
    let end_row = end_row - start_row;
    for (i, row) in (end_row..count_rows).enumerate() {
        let row = row - i;
        records.remove_row(row);
    }

    for (i, col) in (0..start_col).enumerate() {
        let col = col - i;
        records.remove_column(col);
    }

    let count_cols = count_cols - start_col;
    let end_col = end_col - start_col;
    for (i, col) in (end_col..count_cols).enumerate() {
        let col = col - i;
        records.remove_column(col);
    }
}
