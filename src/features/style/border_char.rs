use papergrid::records::Records;

use crate::{style::Offset, CellOption, Table};

/// [`BorderChar`] sets a char to a specific location on a horizontal line.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::{Style, BorderChar, Offset}, Modify, object::Rows};
///
/// let table = Table::new(["Hello World"])
///     .with(Style::markdown())
///     .with(Modify::new(Rows::single(1))
///         .with(BorderChar::new(':', Offset::Begin(0)))
///         .with(BorderChar::new(':', Offset::End(0)))
///     );
///
/// assert_eq!(
///     table.to_string(),
///     "| &str        |\n\
///      |:-----------:|\n\
///      | Hello World |"
/// );
/// ```
#[derive(Debug)]
pub struct BorderChar {
    c: char,
    offset: Offset,
}

impl BorderChar {
    /// Creates a [`BorderChar`].
    pub fn new(c: char, offset: Offset) -> Self {
        Self { c, offset }
    }
}

impl<R> CellOption<R> for BorderChar
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: papergrid::Entity) {
        let offset = self.offset.into();
        for pos in entity.iter(table.count_rows(), table.count_rows()) {
            table
                .get_config_mut()
                .override_horizontal_border(pos, self.c, offset);
        }
    }
}
