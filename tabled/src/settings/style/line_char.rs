use crate::{
    grid::config::{ColoredConfig, Entity, Offset, Position, SpannedConfig},
    grid::records::{ExactRecords, Records},
    settings::CellOption,
};

/// [`LineChar`] sets a char to a specific location on a horizontal line.
///
/// # Example
///
/// ```rust
/// use tabled::{
///     Table,
///     grid::config::Offset,
///     assert::assert_table,
///     settings::{style::{Style, LineChar}, object::{Object, Rows, Columns}}
/// };
///
/// let mut table = Table::new(["Hello World"]);
/// table
///     .with(Style::markdown())
///     .modify(
///         Rows::one(1),
///         (LineChar::horizontal(':', Offset::Start(0)), LineChar::horizontal(':', Offset::End(0))),
///     )
///     .modify((1, 0).and((1, 1)), LineChar::vertical('#', Offset::Start(0)));
///
/// assert_table!(
///     table,
///     "| &str        |"
///     "|:-----------:|"
///     "# Hello World #"
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
    for pos in cells {
        cfg.set_vertical_char(pos, offset, c);
    }
}

fn add_char_horizontal<I: Iterator<Item = Position>>(
    cfg: &mut SpannedConfig,
    c: char,
    offset: Offset,
    cells: I,
) {
    for pos in cells {
        cfg.set_horizontal_char(pos, offset, c);
    }
}
