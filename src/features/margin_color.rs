//! This module contains a configuration of a color for [`Margin`].
//!
//! [`Margin`]: crate::Margin

use crate::{color::Color, Table, TableOption};

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
pub struct MarginColor {
    inner: papergrid::MarginColor<'static>,
}

impl MarginColor {
    /// Creates a new [MarginColor] with colors set for all sides.
    pub fn new(top: Color, bottom: Color, left: Color, right: Color) -> Self {
        Self {
            inner: papergrid::MarginColor {
                bottom: bottom.into(),
                left: left.into(),
                right: right.into(),
                top: top.into(),
            },
        }
    }
}

impl<R> TableOption<R> for MarginColor {
    fn change(&mut self, table: &mut Table<R>) {
        table.get_config_mut().set_margin_color(self.inner.clone());
    }
}
