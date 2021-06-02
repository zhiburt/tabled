use papergrid::{Alignment, Entity, Grid, Settings};

use crate::TableOption;

/// HorizontalAlignment represent a horizontal alignemt setting for a [`table` macros](./macro.table.html)
///
/// ```rust,no_run
///   # use tabled::{Style, HorizontalAlignment, AlignmentObject, Alignment, table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = table!(
///         &data,
///         HorizontalAlignment::new(Alignment::Center, AlignmentObject::Header)
///     );
/// ```
///
#[derive(Debug)]
pub struct HorizontalAlignment {
    alignment: Alignment,
    object: AlignmentObject,
}

impl HorizontalAlignment {
    /// New creates a `HorizontalAlignment` object settings of which will be applied to a [`AlignmentObject`](./enum.AlignmentObject.html).
    pub fn new(alignment: Alignment, object: AlignmentObject) -> Self {
        Self { alignment, object }
    }
}

/// AlignmentObject represent a set of cells/rows which should be aligned.
#[derive(Debug)]
pub enum AlignmentObject {
    /// Header means a first row which contains names of columns
    Header,
    /// Data means all cells except the ones in a header
    Data,
    /// Full means all cells on a `Grid`
    Full,
}

impl TableOption for HorizontalAlignment {
    fn change(&self, grid: &mut Grid) {
        match self.object {
            AlignmentObject::Data => {
                for row in 1..grid.count_rows() {
                    grid.set(
                        Entity::Row(row),
                        Settings::new().alignment(self.alignment.clone()),
                    )
                }
            }
            AlignmentObject::Header => grid.set(
                Entity::Row(0),
                Settings::new().alignment(self.alignment.clone()),
            ),
            AlignmentObject::Full => grid.set(
                Entity::Global,
                Settings::new().alignment(self.alignment.clone()),
            ),
        }
    }
}
