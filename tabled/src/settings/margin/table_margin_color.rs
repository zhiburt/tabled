//! This module contains a configuration of a color for [`Margin`].
//!
//! [`Margin`]: crate::settings::margin::Margin

use crate::{
    grid::config::{GridConfig, MarginColor as GridMarginColor},
    settings::{color::Color, TableOption},
};

/// List of colors for [`Margin`].
///
/// ```rust,no_run
/// # use tabled::{settings::{margin::{Margin, MarginColor}, color::Color}, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Margin::new(1, 1, 1, 1))
///     .with(MarginColor::new(
///         Color::BG_RED,
///         Color::default(),
///         Color::default(),
///         Color::default(),
///     ));
/// ```
///
/// [`Margin`]: crate::settings::margin::Margin
#[cfg_attr(docsrs, doc(cfg(feature = "color")))]
#[derive(Debug, Clone, Default)]
pub struct MarginColor(GridMarginColor<'static>);

impl MarginColor {
    /// Creates a new [MarginColor] with colors set for all sides.
    pub fn new(top: Color, bottom: Color, left: Color, right: Color) -> Self {
        Self(GridMarginColor {
            bottom: bottom.into(),
            left: left.into(),
            right: right.into(),
            top: top.into(),
        })
    }
}

impl<R, D> TableOption<R, D> for MarginColor {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        cfg.set_margin_color(self.0.clone());
    }
}
