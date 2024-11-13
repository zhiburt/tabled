//! This module contains a configuration of a Border to set its color via [`BorderColor`].

use crate::{
    grid::{
        config::{Border as GridBorder, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::{CellOption, Color, TableOption},
};

/// Border represents a border color of a Cell.
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
/// # Example
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{style::{Style, BorderColor}, object::Rows, Color}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .modify(Rows::single(0), BorderColor::new().top(Color::FG_RED));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BorderColor {
    inner: GridBorder<Color>,
}

impl BorderColor {
    pub(crate) const fn from_border(inner: GridBorder<Color>) -> Self {
        BorderColor { inner }
    }

    /// Creates an empty border.
    pub const fn new() -> Self {
        Self::from_border(GridBorder::empty())
    }

    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub const fn full(
        top: Color,
        bottom: Color,
        left: Color,
        right: Color,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    ) -> Self {
        Self::from_border(GridBorder::full(
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

    /// This function constructs a cell borders with all sides's char set to a given color.
    /// It behaves like [`BorderColor::full`] with the same color set to each side.
    pub fn filled(c: Color) -> Self {
        Self::full(
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c,
        )
    }

    /// Set a top border color.
    pub fn top(mut self, c: Color) -> Self {
        self.inner.top = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a bottom border color.
    pub fn bottom(mut self, c: Color) -> Self {
        self.inner.bottom = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a left border color.
    pub fn left(mut self, c: Color) -> Self {
        self.inner.left = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Set a right border color.
    pub fn right(mut self, c: Color) -> Self {
        self.inner.right = Some(c);
        BorderColor::from_border(self.inner)
    }

    /// Converts a border into a general data structure.
    pub fn into_inner(self) -> GridBorder<Color> {
        self.inner
    }

    /// Set a top left intersection color.
    pub fn corner_top_left(mut self, c: Color) -> Self {
        self.inner.left_top_corner = Some(c);
        self
    }

    /// Set a top right intersection color.
    pub fn corner_top_right(mut self, c: Color) -> Self {
        self.inner.right_top_corner = Some(c);
        self
    }

    /// Set a bottom left intersection color.
    pub fn corner_bottom_left(mut self, c: Color) -> Self {
        self.inner.left_bottom_corner = Some(c);
        self
    }

    /// Set a bottom right intersection color.
    pub fn corner_bottom_right(mut self, c: Color) -> Self {
        self.inner.right_bottom_corner = Some(c);
        self
    }
}

impl From<BorderColor> for GridBorder<Color> {
    fn from(value: BorderColor) -> Self {
        value.inner
    }
}

impl<Data> CellOption<Data, ColoredConfig> for BorderColor
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, entity: Entity) {
        let border = self.inner.clone().convert();
        if matches!(entity, Entity::Global) && border.is_same() && !border.is_empty() {
            let color = border.top.expect("ok");
            cfg.set_border_color_default(color);
            return;
        }

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            cfg.set_border_color(pos, border.clone());
        }
    }
}

impl<Data, D> TableOption<Data, ColoredConfig, D> for BorderColor
where
    Data: Records + ExactRecords,
{
    fn change(self, records: &mut Data, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();
        let shape = (count_rows, count_columns);

        if count_rows == 0 || count_columns == 0 {
            return;
        }

        let color = self.inner.clone().convert();

        for col in 0..count_columns {
            let pos = (0, col);
            let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
            b.top = color.top.clone();
            b.right_top_corner = color.top.clone();
            cfg.set_border_color(pos, b);

            let pos = (count_rows - 1, col);
            let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
            b.bottom = color.bottom.clone();
            b.right_bottom_corner = color.bottom.clone();
            cfg.set_border_color(pos, b);
        }

        for row in 0..count_rows {
            let pos = (row, 0);
            let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
            b.left = color.left.clone();
            b.left_bottom_corner = color.left.clone();
            cfg.set_border_color(pos, b);

            let pos = (row, count_columns - 1);
            let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
            b.right = color.right.clone();
            b.right_bottom_corner = color.right.clone();
            cfg.set_border_color(pos, b);
        }

        let pos = (0, 0);
        let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
        b.left_top_corner = color.left_top_corner.clone();
        cfg.remove_border_color(pos, shape);
        cfg.set_border_color(pos, b);

        let pos = (0, count_columns - 1);
        let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
        b.right_top_corner = color.right_top_corner.clone();
        cfg.remove_border_color(pos, shape);
        cfg.set_border_color(pos, b);

        let pos = (count_rows - 1, 0);
        let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
        b.left_bottom_corner = color.left_bottom_corner.clone();
        cfg.remove_border_color(pos, shape);
        cfg.set_border_color(pos, b);

        let pos = (count_rows - 1, count_columns - 1);
        let mut b = GridBorder::cloned(&cfg.get_border_color(pos, shape));
        b.right_bottom_corner = color.right_bottom_corner.clone();
        cfg.remove_border_color(pos, shape);
        cfg.set_border_color(pos, b);
    }
}
