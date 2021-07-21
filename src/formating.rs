use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

/// Format a structure which modifies a [Grid]
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
///                .with(Modify::new(Row(1..)).with(Format(|s| { format!(": {} :", s) })))
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
pub struct Format<F: Fn(&str) -> String>(pub F);

impl<F: Fn(&str) -> String> CellOption for Format<F> {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self.0)(content);
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}

impl<F> CellOption for F
where
    F: for<'r> FnMut(&'r str) -> String,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self)(content);
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}

/// Multiline a helper function for changing multiline content of cell by rows not as a whole.
///
/// ```rust,no_run
///     use tabled::{Table, Format, multiline, Full, Modify};
///     let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Modify::new(Full).with(Format(multiline(|s| format!("{}", s)))))
///         .to_string();
/// ```
pub fn multiline<F: 'static + Fn(&str) -> String>(f: F) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s: &str| s.lines().map(|s| f(s)).collect::<Vec<_>>().join("\n"))
}

/// FormatFrom repeatedly uses first possible element
/// from given array unless there's any elements.
///
/// # Example
///
/// ```
/// use tabled::{Table, FormatFrom, Row, Modify};
///
/// let data = vec![
///     (0, "Grodno", true),
///     (1, "Minsk", true),
///     (2, "Hamburg", false),
///     (3, "Brest", true),
/// ];
///
/// let table = Table::new(&data)
///                .with(Modify::new(Row(..1)).with(FormatFrom(vec!["N", "City", "is in Belarus"])))
///                .to_string();
///
/// assert_eq!(table, "+---+---------+---------------+\n\
///                    | N |  City   | is in Belarus |\n\
///                    +---+---------+---------------+\n\
///                    | 0 | Grodno  |     true      |\n\
///                    +---+---------+---------------+\n\
///                    | 1 |  Minsk  |     true      |\n\
///                    +---+---------+---------------+\n\
///                    | 2 | Hamburg |     false     |\n\
///                    +---+---------+---------------+\n\
///                    | 3 |  Brest  |     true      |\n\
///                    +---+---------+---------------+\n");
/// ```
pub struct FormatFrom<A>(pub Vec<A>)
where
    A: Into<String>;

impl<A> CellOption for FormatFrom<A>
where
    A: Into<String>,
{
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        if !self.0.is_empty() {
            let new_content = self.0.remove(0).into();
            grid.set(Entity::Cell(row, column), Settings::new().text(new_content))
        }
    }
}

/// FormatWithIndex is like a [Format].
/// But it also provides a row and column index.
///
/// # Example
///
/// ```
/// use tabled::{Table, FormatWithIndex, Row, Modify};
///
/// let data = vec![
///     (0, "Grodno", true),
///     (1, "Minsk", true),
///     (2, "Hamburg", false),
///     (3, "Brest", true),
/// ];
///
/// let table = Table::new(&data)
///                .with(Modify::new(Row(..1))
///                     .with(FormatWithIndex(|_, _, column| column.to_string())))
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
pub struct FormatWithIndex<F: FnMut(&str, usize, usize) -> String>(pub F);

impl<F: FnMut(&str, usize, usize) -> String> CellOption for FormatWithIndex<F> {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let content = grid.get_cell_content(row, column);
        let content = (self.0)(content, row, column);
        grid.set(Entity::Cell(row, column), Settings::new().text(content))
    }
}
