use std::ops::RangeBounds;

use crate::TableOption;

/// Returns a new [Table] that reflects a segment of the referenced [Table]
///
/// The segment is defined by [RangeBounds<usize>] for Rows and Columns
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
pub struct Extract<R, C>
where
    R: RangeBounds<usize>,
    C: RangeBounds<usize>,
{
    rows: R,
    columns: C,
}

impl<R, C> Extract<R, C>
where
    R: RangeBounds<usize>,
    C: RangeBounds<usize>,
{
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
