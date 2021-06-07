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
///    # use tabled::{table, Format, Row};
///     let data = vec![
///         (0, "Grodno", true),    
///         (1, "Minsk", true),    
///         (2, "Hamburg", false),    
///         (3, "Brest", true),    
///     ];
///
///     let table = table!(&data, Format(Row(1..), |s| { format!(": {} :", s) }));
///
///     assert_eq!(table, "+-------+-------------+-----------+\n\
///                        |  i32  |    &str     |   bool    |\n\
///                        +-------+-------------+-----------+\n\
///                        | : 0 : | : Grodno :  | : true :  |\n\
///                        +-------+-------------+-----------+\n\
///                        | : 1 : |  : Minsk :  | : true :  |\n\
///                        +-------+-------------+-----------+\n\
///                        | : 2 : | : Hamburg : | : false : |\n\
///                        +-------+-------------+-----------+\n\
///                        | : 3 : |  : Brest :  | : true :  |\n\
///                        +-------+-------------+-----------+\n");
/// ```
///
pub struct Format<O: Object, F: Fn(&str) -> String>(pub O, pub F);

impl<O: Object, F: Fn(&str) -> String> TableOption for Format<O, F> {
    fn change(&self, grid: &mut Grid) {
        println!("-----");
        let cells = self.0.cells(grid.count_rows(), grid.count_columns());
        for (row, column) in cells {
            let content = grid.get_cell_content(row, column);
            println!("{:?}", content);
            let content = (self.1)(content);
            println!("new {:?}", content);
            grid.set(Entity::Cell(row, column), Settings::new().text(content))
        }
    }
}

/// Multiline a helper function for changing multiline content of cell by rows not as a whole.
///
/// ```rust,no_run
///     use tabled::{table, Format, multiline, Full};
///     let data: Vec<&'static str> = Vec::new();
///     table!(&data, Format(Full, multiline(|s| { format!("{}", s) })));
pub fn multiline<F: 'static + Fn(&str) -> String>(f: F) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s: &str| s.lines().map(|s| f(s)).collect::<Vec<_>>().join("\n"))
}
