//! This module contains an [Extract] structure which is used to
//! obtain an ordinary segment from the [Table].
//!
//! There's a similar structure [Highlight] which does a highlighting a of segments.
//!
//! [Table]: crate::Table
//! [Highlight]: crate::Highlight

use std::ops::{RangeBounds, RangeFull};

use papergrid::{Entity, Grid};

use crate::{object::bounds_to_usize, TableOption};

/// Returns a new [Table] that reflects a segment of the referenced [Table]
///
/// The segment is defined by [RangeBounds<usize>] for Rows and Columns.
///
/// # Example
///
/// ```
/// use tabled::{Table, Format, object::Rows, Modify, Extract};
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
/// [Table]: crate::Table
pub struct Extract<R, C> {
    rows: R,
    columns: C,
}

impl<R, C> Extract<R, C>
where
    R: RangeBounds<usize>,
    C: RangeBounds<usize>,
{
    /// Returns a new [Table] that reflects a segment of the referenced [Table]
    ///
    /// The segment is defined by [RangeBounds<usize>] for Rows and Columns
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
    /// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
    ///
    /// If a [RangeBounds] argument is malformed or too large the thread will panic
    ///
    /// ```text
    /// // Empty                         Full                      Out of bounds
    ///    Extract::segment(0..0, 0..0)  Extract::segment(.., ..)  Extract::segment(0..1, ..4)
    ///    [].   .   .                   [O   O   O                [O   O   O  X] //ERROR            
    ///      .   .   .                    O   O   O                 .   .   .             
    ///      .   .   .                    O   O   O]                .   .   .          
    /// ```
    ///
    /// [Table]: crate::Table
    pub fn segment(rows: R, columns: C) -> Self {
        Extract { rows, columns }
    }
}

impl<R> Extract<R, RangeFull>
where
    R: RangeBounds<usize>,
{
    /// Returns a new [Table] that reflects a segment of the referenced [Table]
    ///
    /// The segment is defined by [RangeBounds<usize>] for Rows
    ///
    /// ```rust,no_run
    /// # use tabled::Extract;
    /// Extract::rows(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
    ///
    /// If a [RangeBounds] argument is malformed or too large the thread will panic
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
    /// [Table]: crate::Table
    pub fn rows(rows: R) -> Self {
        Extract { rows, columns: .. }
    }
}

impl<C> Extract<RangeFull, C>
where
    C: RangeBounds<usize>,
{
    /// Returns a new [Table] that reflects a segment of the referenced [Table]
    ///
    /// The segment is defined by [RangeBounds<usize>] for Columns
    ///
    /// ```rust,no_run
    /// # use tabled::Extract;
    /// Extract::columns(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
    ///
    /// If a [RangeBounds] argument is malformed or too large the thread will panic
    ///
    /// ```text
    /// // Empty                   Full                  Out of bounds
    ///    Extract::columns(0..0)  Extract::columns(..)  Extract::columns(0..4)
    ///    [].   .   .             [O   O   O            [O   O   O   X          
    ///      .   .   .              O   O   O             O   O   O   X          
    ///      .   .   .              O   O   O]            O   O   O   X] // ERROR
    /// ```
    ///
    /// [Table]: crate::Table
    pub fn columns(columns: C) -> Self {
        Extract { rows: .., columns }
    }
}

impl<R, C> TableOption for Extract<R, C>
where
    R: RangeBounds<usize> + Clone,
    C: RangeBounds<usize> + Clone,
{
    fn change(&mut self, grid: &mut Grid) {
        let row_bounds = bounds_to_usize(
            self.rows.start_bound(),
            self.rows.end_bound(),
            grid.count_rows(),
        );
        let col_bounds = bounds_to_usize(
            self.columns.start_bound(),
            self.columns.end_bound(),
            grid.count_columns(),
        );

        *grid = extract(grid, row_bounds, col_bounds);
    }
}

/// Returns a new [Grid] that reflects a segment of the referenced [Grid]
///
/// The segment is defined by [RangeBounds<usize>] for Rows and Columns
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
fn extract(
    grid: &Grid,
    (start_row, end_row): (usize, usize),
    (start_col, end_col): (usize, usize),
) -> Grid {
    let new_count_rows = end_row - start_row;
    let new_count_columns = end_col - start_col;
    let mut new = grid.resize(new_count_rows, new_count_columns);

    for (new_row, row) in (start_row..end_row).enumerate() {
        for (new_col, col) in (start_col..end_col).enumerate() {
            let settings = grid.get_settings(row, col);
            new.set(Entity::Cell(new_row, new_col), settings);

            #[cfg(feature = "color")]
            {
                let colored_border = grid.get_colored_border((row, col));
                new.set_colored_border(Entity::Cell(new_row, new_col), colored_border);
            }
        }
    }

    new
}
