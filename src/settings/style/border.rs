use crate::{
    grid::config::{Border as GridBorder, Entity, GridConfig},
    records::{ExactRecords, Records},
    CellOption,
};

/// Border represents a border of a Cell.
///
/// ```rust,no_run
/// # use tabled::{Style, Border, object::Rows, Table, Modify};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(Border::default().top('x')));
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Border(GridBorder<char>);

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
        Self(GridBorder::full(
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
    /// It behaives like [`Border::full`] with the same character set to each side.
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

impl<R> CellOption<R> for Border
where
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.set_border(pos, self.0.clone());
        }
    }
}

impl From<GridBorder> for Border {
    fn from(b: GridBorder) -> Border {
        Border(b)
    }
}

impl From<Border> for GridBorder {
    fn from(value: Border) -> Self {
        value.0
    }
}

pub struct EmptyBorder;

impl<R> CellOption<R> for EmptyBorder
where
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let shape = (records.count_rows(), records.count_columns());

        for pos in entity.iter(shape.0, shape.1) {
            cfg.remove_border(pos, shape);
        }
    }
}
