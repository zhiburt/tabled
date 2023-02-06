//! This module contains a configuration of a color for [`Margin`].
//!
//! [`Margin`]: crate::Margin

use crate::{
    color::Color,
    grid::config::{GridConfig, MarginColor as GridMarginColor},
    TableOption,
};

/// List of colors for [`Margin`].
///
/// ```rust,no_run
/// # use tabled::{Margin, margin_color::MarginColor, color::Color, Table};
/// # use owo_colors::OwoColorize;
/// # use std::convert::TryFrom;
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Margin::new(1, 1, 1, 1))
///     .with(MarginColor::new(
///         Color::try_from(" ".on_blue().red().bold().to_string()).unwrap(),
///         Color::default(),
///         Color::default(),
///         Color::default(),
///     ));
/// ```
///
/// [`Margin`]: crate::Margin
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
