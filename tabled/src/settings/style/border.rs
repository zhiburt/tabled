use crate::{
    grid::{
        config::{Border as GBorder, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::CellOption,
};

/// Border represents a border of a Cell.
///
/// ```text
///                         top border
///                             |
///                             V
/// corner top left ------> +_______+  <---- corner top left
///                         |       |
/// left border ----------> |  cell |  <---- right border
///                         |       |
/// corner bottom right --> +_______+  <---- corner bottom right
///                             ^
///                             |
///                        bottom border
/// ```
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{Modify, style::{Style, Border}, object::Rows}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Border(GBorder<char>);

impl Border {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self(GBorder::full(
            top,
            bottom,
            left,
            right,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ))
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub const fn filled(c: char) -> Self {
        Self::full(c, c, c, c, c, c, c, c)
    }

    /// Using this function you deconstruct the existing borders.
    pub const fn empty() -> EmptyBorder {
        EmptyBorder
    }

    /// Set a top border character.
    pub const fn top(mut self, c: char) -> Self {
        self.0.top = Some(c);
        self
    }

    /// Set a bottom border character.
    pub const fn bottom(mut self, c: char) -> Self {
        self.0.bottom = Some(c);
        self
    }

    /// Set a left border character.
    pub const fn left(mut self, c: char) -> Self {
        self.0.left = Some(c);
        self
    }

    /// Set a right border character.
    pub const fn right(mut self, c: char) -> Self {
        self.0.right = Some(c);
        self
    }

    /// Set a top left intersection character.
    pub const fn corner_top_left(mut self, c: char) -> Self {
        self.0.left_top_corner = Some(c);
        self
    }

    /// Set a top right intersection character.
    pub const fn corner_top_right(mut self, c: char) -> Self {
        self.0.right_top_corner = Some(c);
        self
    }

    /// Set a bottom left intersection character.
    pub const fn corner_bottom_left(mut self, c: char) -> Self {
        self.0.left_bottom_corner = Some(c);
        self
    }

    /// Set a bottom right intersection character.
    pub const fn corner_bottom_right(mut self, c: char) -> Self {
        self.0.right_bottom_corner = Some(c);
        self
    }
}

impl<R> CellOption<R, ColoredConfig> for Border
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.set_border(pos, self.0);
        }
    }
}

impl From<GBorder<char>> for Border {
    fn from(b: GBorder<char>) -> Border {
        Border(b)
    }
}

impl From<Border> for GBorder<char> {
    fn from(value: Border) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptyBorder;

impl<R> CellOption<R, ColoredConfig> for EmptyBorder
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.remove_border(pos, shape);
        }
    }
}
