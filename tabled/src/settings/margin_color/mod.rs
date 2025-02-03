//! This module contains a Margin settings of a [`Table`].
//!
//! [`Table`]: crate::Table

use crate::{
    grid::{
        ansi::ANSIStr,
        config::Sides,
        config::{CompactConfig, CompactMultilineConfig},
    },
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::grid::{ansi::ANSIBuf, config::ColoredConfig};

/// MarginColor is responsible for a left/right/top/bottom outer color of a grid.
///
/// # Example
///
#[cfg_attr(feature = "ansi", doc = "```")]
#[cfg_attr(not(feature = "ansi"), doc = "```ignore")]
/// use tabled::{
///     settings::{Margin, MarginColor, Style, Color},
///     Table,
/// };
///
/// let data = vec!["Hello", "World", "!"];
///
/// let mut table = Table::new(data);
/// table
///     .with(Style::markdown())
///     .with(Margin::new(3, 3, 1, 0))
///     .with(MarginColor::filled(Color::BG_RED));
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "\u{1b}[41m               \u{1b}[49m\n",
///         "\u{1b}[41m   \u{1b}[49m| &str  |\u{1b}[41m   \u{1b}[49m\n",
///         "\u{1b}[41m   \u{1b}[49m|-------|\u{1b}[41m   \u{1b}[49m\n",
///         "\u{1b}[41m   \u{1b}[49m| Hello |\u{1b}[41m   \u{1b}[49m\n",
///         "\u{1b}[41m   \u{1b}[49m| World |\u{1b}[41m   \u{1b}[49m\n",
///         "\u{1b}[41m   \u{1b}[49m| !     |\u{1b}[41m   \u{1b}[49m",
///     )
/// );
/// ```
#[derive(Debug, Clone)]
pub struct MarginColor<C> {
    colors: Sides<C>,
}

impl<C> MarginColor<C> {
    /// Construct's an Margin object.
    pub const fn new(left: C, right: C, top: C, bottom: C) -> Self {
        Self {
            colors: Sides::new(left, right, top, bottom),
        }
    }

    /// The function, sets a color for the margin on an each side.
    pub fn filled(color: C) -> Self
    where
        C: Clone,
    {
        Self::new(color.clone(), color.clone(), color.clone(), color)
    }
}

impl MarginColor<ANSIStr<'static>> {
    /// Construct's an Padding object with no color.
    pub const fn empty() -> Self {
        Self::new(
            ANSIStr::empty(),
            ANSIStr::empty(),
            ANSIStr::empty(),
            ANSIStr::empty(),
        )
    }
}

#[cfg(feature = "std")]
impl<R, D, C> TableOption<R, ColoredConfig, D> for MarginColor<C>
where
    C: Into<ANSIBuf> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let c = self.colors.clone();
        let margin = Sides::new(
            Some(c.left.into()),
            Some(c.right.into()),
            Some(c.top.into()),
            Some(c.bottom.into()),
        );

        cfg.set_margin_color(margin);
    }
}

impl<R, D, C> TableOption<R, CompactConfig, D> for MarginColor<C>
where
    C: Into<ANSIStr<'static>> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        let colors = self.colors.convert_into();
        *cfg = cfg.set_margin_color(colors);
    }
}

impl<R, D, C> TableOption<R, CompactMultilineConfig, D> for MarginColor<C>
where
    C: Into<ANSIStr<'static>> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        let colors = self.colors.convert_into();
        cfg.set_margin_color(colors);
    }
}
