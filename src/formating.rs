use papergrid::{Entity, Grid, Settings};

use crate::{Object, TableOption};

/// ChangeRing a structure which modifies a `Grid` in a series of rounds.
/// It calls a function in a cycle for a set of cells.
///
/// So for example if we have 2 function in a pool `[foo, bar]` and we run it against a [`Head`](./struct.Head.html),
/// where `Grid` will contain 5 columns. `ChangeRing` will call `foo` on the first cell, `bar` on the second then
/// it again run `foo` on the third and so on.
///
/// # Example
///
/// Note that settings isn't applied by row they applied to cells in a row.
/// Look at these two examples.
///
/// ```rust
///    # use tabled::{table, ChangeRing, Row};
///     let data = vec![
///         (0, "Grodno", true),    
///         (1, "Minsk", true),    
///         (2, "Hamburg", false),    
///         (3, "Brest", true),    
///     ];
///
///     let table = table!(
///         &data,
///         ChangeRing(Row(1..), vec![
///             Box::new(|s| { format!(": {} :", s) }),
///             Box::new(|s| { format!("++ {} ++", s) }),
///         ]),
///     );
///
///     assert_eq!(table, "+---------+---------------+------------+\n\
///                        |   i32   |     &str      |    bool    |\n\
///                        +---------+---------------+------------+\n\
///                        |  : 0 :  | ++ Grodno ++  |  : true :  |\n\
///                        +---------+---------------+------------+\n\
///                        | ++ 1 ++ |   : Minsk :   | ++ true ++ |\n\
///                        +---------+---------------+------------+\n\
///                        |  : 2 :  | ++ Hamburg ++ | : false :  |\n\
///                        +---------+---------------+------------+\n\
///                        | ++ 3 ++ |   : Brest :   | ++ true ++ |\n\
///                        +---------+---------------+------------+\n");
/// ```
///
pub struct ChangeRing<O: Object>(pub O, pub Vec<Box<dyn Fn(&str) -> String>>);

impl<O: Object> TableOption for ChangeRing<O> {
    fn change(&self, grid: &mut Grid) {
        if self.1.is_empty() {
            return;
        }

        let mut ring = self.1.iter().cycle();

        let cells = self.0.cells(grid.count_rows(), grid.count_columns());
        for (row, column) in cells {
            let change_function = ring.next().unwrap();
            let content = grid.get_cell_content(row, column);
            let content = change_function(content);
            grid.set(Entity::Cell(row, column), Settings::new().text(content))
        }
    }
}

/// Multiline a helper function for changing multiline content of cell by rows not as a whole.
///
/// ```rust,no_run
///     use tabled::{table, ChangeRing, multiline, Full};
///     let data: Vec<&'static str> = Vec::new();
///     table!(
///         &data,
///         ChangeRing(
///             Full,
///             vec![
///                 multiline(Box::new(|s| { format!("{}", s) })),
///             ]
///         ),
///     );
pub fn multiline(f: Box<dyn Fn(&str) -> String>) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s: &str| s.lines().map(|s| f(s)).collect::<Vec<_>>().join("\n"))
}
