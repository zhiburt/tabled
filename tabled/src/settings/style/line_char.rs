use crate::{
    grid::config::{ColoredConfig, Entity, Position, SpannedConfig},
    grid::records::{ExactRecords, Records},
    settings::CellOption,
};

use super::Offset;

/// [`LineChar`] sets a char to a specific location on a horizontal line.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, settings::{style::{Style, LineChar, Offset}, Modify, object::{Object, Rows, Columns}}};
///
/// let mut table = Table::new(["Hello World"]);
/// table
///     .with(Style::markdown())
///     .with(Modify::new(Rows::single(1))
///         .with(LineChar::horizontal(':', Offset::Begin(0)))
///         .with(LineChar::horizontal(':', Offset::End(0)))
///     )
///     .with(Modify::new((1, 0).and((1, 1))).with(LineChar::vertical('#', Offset::Begin(0))));
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineChar {
    c: char,
    offset: Offset,
    horizontal: bool,
}

impl LineChar {
    /// Creates a [`LineChar`] which overrides horizontal line.
    pub fn horizontal(c: char, offset: impl Into<Offset>) -> Self {
        let offset = offset.into();
        let horizontal = true;

        Self {
            c,
            offset,
            horizontal,
        }
    }

    /// Creates a [`LineChar`] which overrides vertical line.
    pub fn vertical(c: char, offset: impl Into<Offset>) -> Self {
        let offset = offset.into();
        let horizontal = false;

        Self {
            c,
            offset,
            horizontal,
        }
    }
}

impl<R> CellOption<R, ColoredConfig> for LineChar
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let cells = entity.iter(records.count_rows(), records.count_columns());

        match self.horizontal {
            true => add_char_horizontal(cfg, self.c, self.offset, cells),
            false => add_char_vertical(cfg, self.c, self.offset, cells),
        }
    }
}

fn add_char_vertical<I: Iterator<Item = Position>>(
    cfg: &mut SpannedConfig,
    c: char,
    offset: Offset,
    cells: I,
) {
    let offset = offset.into();

    for pos in cells {
        cfg.set_vertical_char(pos, c, offset);
    }
}

fn add_char_horizontal<I: Iterator<Item = Position>>(
    cfg: &mut SpannedConfig,
    c: char,
    offset: Offset,
    cells: I,
) {
    let offset = offset.into();

    for pos in cells {
        cfg.set_horizontal_char(pos, c, offset);
    }
}
