use papergrid::{records::Records, Entity};

use crate::{CellOption, Table};

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
pub struct Border {
    border: Option<papergrid::Border<char>>,
}

impl Border {
    pub(crate) const fn new_raw(border: Option<papergrid::Border<char>>) -> Self {
        Self { border }
    }
}

impl Border {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn full(
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self::from(papergrid::Border::full(
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

    /// Using this function you deconstruct the existing borders.
    pub fn empty() -> Self {
        Self { border: None }
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [`Border::full`] with the same character set to each side.
    pub fn filled(c: char) -> Self {
        Self::full(c, c, c, c, c, c, c, c)
    }

    /// Set a top border character.
    pub fn top(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.top = Some(c);
        Self::from(b)
    }

    /// Set a bottom border character.
    pub fn bottom(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.bottom = Some(c);
        Self::from(b)
    }

    /// Set a left border character.
    pub fn left(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left = Some(c);
        Self::from(b)
    }

    /// Set a right border character.
    pub fn right(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right = Some(c);
        Self::from(b)
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left_top_corner = Some(c);
        Self::from(b)
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right_top_corner = Some(c);
        Self::from(b)
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.left_bottom_corner = Some(c);
        Self::from(b)
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(self, c: char) -> Self {
        let mut b = self.border.unwrap_or_default();
        b.right_bottom_corner = Some(c);
        Self::from(b)
    }
}

impl<R> CellOption<R> for Border
where
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let (count_rows, count_cols) = table.shape();
        let cfg = table.get_config_mut();
        for pos in entity.iter(count_rows, count_cols) {
            match &self.border {
                Some(border) => cfg.set_border(pos, border.clone()),
                None => cfg.remove_border(pos, count_cols),
            }
        }
    }
}

impl From<papergrid::Border> for Border {
    fn from(b: papergrid::Border) -> Border {
        Border { border: Some(b) }
    }
}

impl From<Border> for Option<papergrid::Border> {
    fn from(val: Border) -> Self {
        val.border
    }
}
