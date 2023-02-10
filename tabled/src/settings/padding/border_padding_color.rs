//! This module contains a [`PaddingColor`] setting of a cell on a [`Table`].
//!
//! [`Table`]: crate::Table

use crate::{
    grid::config::{Entity, GridConfig, PaddingColor as GridPaddingColor},
    settings::{color::Color, CellOption},
};

/// Color settings for a [`Padding`].
///
/// ```rust,no_run
/// # use tabled::{Table, settings::{padding::PaddingColor, color::Color, Modify, object::Segment}};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Modify::new(Segment::all()).with(
///         PaddingColor::new(Color::FG_RED, Color::FG_BLUE, Color::FG_BLUE, Color::FG_RED)
///     ));
/// ```
///
/// [`Padding`]: crate::settings::padding::Padding
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default)]
pub struct PaddingColor(GridPaddingColor<'static>);

impl PaddingColor {
    /// Creates a new [PaddingColor] with colors set for all sides.
    pub fn new(top: Color, bottom: Color, left: Color, right: Color) -> Self {
        Self(GridPaddingColor {
            bottom: bottom.into(),
            left: left.into(),
            right: right.into(),
            top: top.into(),
        })
    }
}

impl<R> CellOption<R> for PaddingColor {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, entity: Entity) {
        cfg.set_padding_color(entity, self.0.clone());
    }
}
