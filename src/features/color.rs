//! This module contains a configuration of a [`Border`] or a [`Table`] to set its borders color via [`Color`].
//!
//! [`Border`]: crate::Border

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
/// use tabled::{color::Color, TableIteratorExt};
///
/// let data = [
///     (0u8, "Hello"),
///     (1u8, "World"),
/// ];
///
/// let table = data.table()
///     .with(Color::try_from(" ".red().to_string()).unwrap())
///     .to_string();
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::Padding
/// [`Margin`]: crate::Margin
/// [`Border`]: crate::Border
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Color(AnsiColor<'static>);

impl Color {
    /// Creates a new [`Color`]` instance, with ANSI prefix and ANSI suffix.
    /// You can use [`TryFrom`] to construct it from [`String`].
    pub fn new(prefix: String, suffix: String) -> Self {
        Self(AnsiColor::new(prefix, suffix))
    }
}

impl From<Color> for AnsiColor<'static> {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<AnsiColor<'static>> for Color {
    fn from(c: AnsiColor<'static>) -> Self {
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
    R: Records,
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
    R: Records,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let border = border_color(self);

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            table.get_config_mut().set_border_color(pos, border.clone());
        }
    }
}

fn border_color(color: &Color) -> papergrid::Border<AnsiColor<'static>> {
    papergrid::Border::full(
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
