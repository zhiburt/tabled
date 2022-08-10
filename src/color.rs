use std::convert::TryFrom;

use papergrid::{records::Records, AnsiColor, Entity};

use crate::{CellOption, Table, TableOption};

/// Color represents a color which can be set to things like [`Border`], [`Padding`] and [`Margin`].
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use owo_colors::OwoColorize;
/// use tabled::{style::Color, TableIteratorExt};
///
/// let data = [
///     (0u8, "Hello"),
///     (1u8, "World"),
/// ];
///
/// let table = data.table()
///     .with(Color::try_from(" ".red().to_string()).unwrap());
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::Padding
/// [`Margin`]: crate::Margin
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Color(AnsiColor);

impl Color {
    pub fn new(prefix: String, suffix: String) -> Self {
        Self(AnsiColor::new(prefix, suffix))
    }
}

impl From<Color> for AnsiColor {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<AnsiColor> for Color {
    fn from(c: AnsiColor) -> Self {
        Self(c)
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

impl TryFrom<String> for Color {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

impl<R> TableOption<R> for Color {
    fn change(&mut self, table: &mut Table<R>) {
        let color = self.0.clone();
        table.get_config_mut().set_border_color_global(color);
    }
}

impl<R> CellOption<R> for Color
where
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let border = border_color(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            table.get_config_mut().set_border_color(pos, border.clone());
        }
    }
}

impl<'b, R> CellOption<R> for &'b Color
where
    for<'a> &'a R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let border = border_color(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            table.get_config_mut().set_border_color(pos, border.clone());
        }
    }
}

fn border_color(color: &Color) -> papergrid::Border<AnsiColor> {
    papergrid::Border::new(
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
        color.0.clone(),
    )
}
