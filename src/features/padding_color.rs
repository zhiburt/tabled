//! This module contains a [`PaddingColor`] setting of a cell on a [`Table`].

use papergrid::Entity;

use crate::{color::Color, CellOption, Table};

/// Color settings for a [Padding].
///
/// ```rust,no_run
/// # use tabled::{padding_color::PaddingColor, color::Color, Table, Modify, object::Segment};
/// # use owo_colors::OwoColorize;
/// # use std::convert::TryFrom;
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Modify::new(Segment::all()).with(PaddingColor::new(
///         Color::try_from(" ".on_blue().red().to_string()).unwrap(),
///         Color::try_from(" ".red().to_string()).unwrap(),
///         Color::try_from(" ".on_blue().red().to_string()).unwrap(),
///         Color::try_from(" ".on_green().to_string()).unwrap(),
///     )));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default)]
pub struct PaddingColor {
    inner: papergrid::PaddingColor,
}

impl PaddingColor {
    /// Creates a new [PaddingColor] with colors set for all sides.
    pub fn new(top: Color, bottom: Color, left: Color, right: Color) -> Self {
        Self {
            inner: papergrid::PaddingColor {
                bottom: bottom.into(),
                left: left.into(),
                right: right.into(),
                top: top.into(),
            },
        }
    }
}

impl<R> CellOption<R> for PaddingColor {
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        table
            .get_config_mut()
            .set_padding_color(entity, self.inner.clone());
    }
}
