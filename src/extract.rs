use std::ops::{RangeBounds, RangeFull};

use crate::TableOption;

/// Returns a new [Table] that reflects a segment of the referenced [Table]
///
/// The segment is defined by [RangeBounds<usize>] for Rows and Columns
///
/// # Range
///
/// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
///
/// If a [RangeBounds] argument is malformed or too large the thread will panic
///
///
/// ```
/// // Empty                         Full                      Out of bounds
///    Extract::segment(0..0, 0..0)  Extract::segment(.., ..)  Extract::segment(0..1, ..4)
///    [].   .   .                   [O   O   O                [O   O   O  X] //ERROR            
///      .   .   .                    O   O   O                 .   .   .             
///      .   .   .                    O   O   O]                .   .   .          
/// ```
///
/// # Example
///
/// ```
/// use tabled::{Table, Format, Row, Modify};
///
/// let data = vec![
///     (0, "Grodno", true),
///     (1, "Minsk", true),
///     (2, "Hamburg", false),
///     (3, "Brest", true),
/// ];
///
/// let table = Table::new(&data)
///                .with(Modify::new(Row(1..)).with(Format(|s| format!(": {} :", s))))
///                .with(Extract::segment(1..=2, 1..))
///                .to_string();
///
/// assert_eq!(table, "+-------------+-----------+\n\
///                    | : Grodno :  | : true :  |\n\
///                    +-------------+-----------+\n\
///                    |  : Minsk :  | : true :  |\n\
///                    +-------------+-----------+\n");
/// ```
///
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
    /// ```
    /// // Empty                         Full                      Out of bounds
    ///    Extract::segment(0..0, 0..0)  Extract::segment(.., ..)  Extract::segment(0..1, ..4)
    ///    [].   .   .                   [O   O   O                [O   O   O  X] //ERROR            
    ///      .   .   .                    O   O   O                 .   .   .             
    ///      .   .   .                    O   O   O]                .   .   .          
    /// ```
    ///
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
    /// Extract::rows(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
    ///
    /// If a [RangeBounds] argument is malformed or too large the thread will panic
    ///
    /// ```
    /// // Empty                Full               Out of bounds
    ///    Extract::rows(0..0)  Extract::rows(..)  Extract::rows(0..4)
    ///    [].   .   .          [O   O   O         [O   O   O             
    ///      .   .   .           O   O   O          O   O   O              
    ///      .   .   .           O   O   O]         O   O   O
    ///                                             X   X   X] // ERROR          
    /// ```
    ///
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
    /// Extract::columns(1..3);
    /// ```
    ///
    /// # Range
    ///
    /// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
    ///
    /// If a [RangeBounds] argument is malformed or too large the thread will panic
    ///
    /// ```
    /// // Empty                   Full                  Out of bounds
    ///    Extract::columns(0..0)  Extract::columns(..)  Extract::columns(0..4)
    ///    [].   .   .             [O   O   O            [O   O   O   X          
    ///      .   .   .              O   O   O             O   O   O   X          
    ///      .   .   .              O   O   O]            O   O   O   X] // ERROR
    /// ```
    ///
    pub fn columns(columns: C) -> Self {
        Extract { rows: .., columns }
    }
}

impl<R, C> TableOption for Extract<R, C>
where
    R: RangeBounds<usize> + Clone,
    C: RangeBounds<usize> + Clone,
{
    fn change(&mut self, grid: &mut papergrid::Grid) {
        *grid = grid.extract(self.rows.clone(), self.columns.clone());
    }
}
