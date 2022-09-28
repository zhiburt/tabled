use papergrid::records::Records;

use crate::{style::Offset, CellOption, Table};

/// [`BorderChar`] sets a char to a specific location on a horizontal line.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::{Style, BorderChar, Offset}, Modify, object::Rows};
///
/// let mut table = Table::new(["Hello World"]);
/// table
///     .with(Style::markdown())
///     .with(Modify::new(Rows::single(1))
///         .with(BorderChar::horizontal(':', Offset::Begin(0)))
///         .with(BorderChar::horizontal(':', Offset::End(0)))
///         .with(BorderChar::vertical('#', Offset::Begin(0)))
///     );
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "| &str        |\n",
///         "|:-----------:|\n",
///         "# Hello World #",
///     ),
/// );
/// ```
#[derive(Debug)]
pub struct BorderChar {
    c: char,
    offset: Offset,
    horizontal: bool,
}

impl BorderChar {
    /// Creates a [`BorderChar`] which overrides horizontal line.
    pub fn horizontal(c: char, offset: Offset) -> Self {
        Self {
            c,
            offset,
            horizontal: true,
        }
    }

    /// Creates a [`BorderChar`] which overrides vertical line.
    pub fn vertical(c: char, offset: Offset) -> Self {
        Self {
            c,
            offset,
            horizontal: false,
        }
    }
}

impl<R> CellOption<R> for BorderChar
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: papergrid::Entity) {
        let offset = self.offset.into();
        for pos in entity.iter(table.count_rows(), table.count_rows()) {
            match self.horizontal {
                true => {
                    table
                        .get_config_mut()
                        .override_horizontal_border(pos, self.c, offset);
                }
                false => {
                    table
                        .get_config_mut()
                        .override_vertical_border(pos, self.c, offset);
                }
            }
        }
    }
}
