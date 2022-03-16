use std::ops::RangeBounds;

use crate::TableOption;

/// Returns a new [Table] that reflects a segment of the referenced [Table]
///
/// The segment is defined by [RangeBounds<usize>] for Rows and Columns
///
/// ```rust,no_run
/// let rows = 1..3;
/// let columns = 1..;
/// Extract::new(rows, columns);
/// ```
/// 
/// # Range
/// 
/// A [RangeBounds] argument can be less than or equal to the shape of a [Table]
/// 
/// If a [RangeBounds] argument is malformed or too large the thread will panic
/// 
/// ```
/// // Empty                     Full                  Out of bounds
///    Extract::new(0..0, 0..0)  Extract::new(.., ..)  Extract::new(0..1, ..4)
///    [].   .   .               [O   O   O            [O   O   O  X] //ERROR            
///      .   .   .                O   O   O             .   .   .             
///      .   .   .                O   O   O]            .   .   .          
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
///                .with(Extract::new(1..=2, 1..))
///                .to_string();
///
/// assert_eq!(table, "+-------------+-----------+\n\
///                    | : Grodno :  | : true :  |\n\
///                    +-------------+-----------+\n\
///                    |  : Minsk :  | : true :  |\n\
///                    +-------------+-----------+\n");
///
/// assert_ne!(table, "+-------+-------------+-----------+\n\
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
pub struct Extract<R, C> {
    rows: R,
    columns: C,
}

impl<R, C> Extract<R, C> {
    pub fn new(rows: R, columns: C) -> Extract<R, C> {
        Extract { rows, columns }
    }
}

impl<R, C> TableOption for Extract<R, C>
where
    R: RangeBounds<usize>,
    C: RangeBounds<usize>,
{
    fn change(&mut self, grid: &mut papergrid::Grid) {
        *grid = grid.extract(&self.rows, &self.columns);
    }
}
