//! This module contains a configuration of a Border to set its color via [`BorderColor`].

use crate::{
    grid::{
        color::AnsiColor,
        config::{Border, ColoredConfig, Entity},
        records::{ExactRecords, Records},
    },
    settings::{color::Color, CellOption, TableOption},
};

/// BorderColored represents a colored border of a Cell.
///
/// ```rust,no_run
/// # use tabled::{settings::{style::BorderColor, Style, Color, object::Rows, Modify}, Table};
/// #
/// # let data: Vec<&'static str> = Vec::new();
/// #
/// let table = Table::new(&data)
///     .with(Style::ascii())
///     .with(Modify::new(Rows::single(0)).with(BorderColor::default().top(Color::FG_RED)));
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct BorderColor(Border<AnsiColor<'static>>);

impl BorderColor {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn full(
        top: Color,
        bottom: Color,
        left: Color,
        right: Color,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    ) -> Self {
        Self(Border::full(
            top.into(),
            bottom.into(),
            left.into(),
            right.into(),
            top_left.into(),
            top_right.into(),
            bottom_left.into(),
            bottom_right.into(),
        ))
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaves like [`Border::full`] with the same character set to each side.
    pub fn filled(c: Color) -> Self {
        let c: AnsiColor<'_> = c.into();

        Self(Border {
            top: Some(c.clone()),
            bottom: Some(c.clone()),
            left: Some(c.clone()),
            right: Some(c.clone()),
            left_bottom_corner: Some(c.clone()),
            left_top_corner: Some(c.clone()),
            right_bottom_corner: Some(c.clone()),
            right_top_corner: Some(c),
        })
    }

    /// Set a top border character.
    pub fn top(mut self, c: Color) -> Self {
        self.0.top = Some(c.into());
        self
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: Color) -> Self {
        self.0.bottom = Some(c.into());
        self
    }

    /// Set a left border character.
    pub fn left(mut self, c: Color) -> Self {
        self.0.left = Some(c.into());
        self
    }

    /// Set a right border character.
    pub fn right(mut self, c: Color) -> Self {
        self.0.right = Some(c.into());
        self
    }

    /// Set a top left intersection character.
    pub fn corner_top_left(mut self, c: Color) -> Self {
        self.0.left_top_corner = Some(c.into());
        self
    }

    /// Set a top right intersection character.
    pub fn corner_top_right(mut self, c: Color) -> Self {
        self.0.right_top_corner = Some(c.into());
        self
    }

    /// Set a bottom left intersection character.
    pub fn corner_bottom_left(mut self, c: Color) -> Self {
        self.0.left_bottom_corner = Some(c.into());
        self
    }

    /// Set a bottom right intersection character.
    pub fn corner_bottom_right(mut self, c: Color) -> Self {
        self.0.right_bottom_corner = Some(c.into());
        self
    }
}

impl<R> CellOption<R, ColoredConfig> for BorderColor
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = &self.0;

        for pos in entity.iter(count_rows, count_columns) {
            cfg.set_border_color(pos, border_color.clone());
        }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for BorderColor
where
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let border_color = &self.0;

        for row in 0..count_rows {
            for col in 0..count_columns {
                cfg.set_border_color((row, col), border_color.clone());
            }
        }
    }
}

impl From<BorderColor> for Border<AnsiColor<'static>> {
    fn from(val: BorderColor) -> Self {
        val.0
    }
}
