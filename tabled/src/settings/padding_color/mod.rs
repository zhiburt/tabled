//! This module contains a [`PaddingColor`] setting of a cell on a [`Table`].
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! # use tabled::{settings::{Style, Padding, object::Rows, Modify}, Table};
//! # let data: Vec<&'static str> = Vec::new();
//! let table = Table::new(&data)
//!     .with(Modify::new(Rows::single(0))
//!         .with(Padding::new(0, 0, 1, 1).fill('>', '<', '^', 'V'))
//!     );
//! ```
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
use crate::grid::{ansi::ANSIBuf, config::ColoredConfig, config::Entity};
#[cfg(feature = "std")]
use crate::settings::CellOption;

/// PaddingColor is responsible for a left/right/top/bottom inner color of a particular cell.
///
/// # Example
///
#[cfg_attr(feature = "ansi", doc = "```")]
#[cfg_attr(not(feature = "ansi"), doc = "```ignore")]
/// use tabled::{
///     Table,
///     settings::{Padding, PaddingColor, Color, Style},
/// };
///
/// let table = Table::new("2024".chars())
///     .with(Style::modern())
///     .modify((2, 0), Padding::new(2, 4, 0, 0))
///     .modify((2, 0), PaddingColor::filled(Color::FG_RED))
///     .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "┌───────┐\n",
///         "│ char  │\n",
///         "├───────┤\n",
///         "│ 2     │\n",
///         "├───────┤\n",
///         "│\u{1b}[31m  \u{1b}[39m0\u{1b}[31m    \u{1b}[39m│\n",
///         "├───────┤\n",
///         "│ 2     │\n",
///         "├───────┤\n",
///         "│ 4     │\n",
///         "└───────┘",
///     ),
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaddingColor<C> {
    colors: Sides<C>,
}

impl<C> PaddingColor<C> {
    /// Construct's an Padding object.
    pub const fn new(left: C, right: C, top: C, bottom: C) -> Self {
        Self {
            colors: Sides::new(left, right, top, bottom),
        }
    }

    /// The function, sets a color for all sides.
    pub fn filled(color: C) -> Self
    where
        C: Clone,
    {
        Self::new(color.clone(), color.clone(), color.clone(), color)
    }
}

impl PaddingColor<ANSIStr<'static>> {
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

impl<C> From<PaddingColor<C>> for Sides<C> {
    fn from(value: PaddingColor<C>) -> Self {
        value.colors
    }
}

impl<C> From<Sides<C>> for PaddingColor<C> {
    fn from(colors: Sides<C>) -> Self {
        Self { colors }
    }
}

#[cfg(feature = "std")]
impl<R, C> CellOption<R, ColoredConfig> for PaddingColor<C>
where
    C: Into<ANSIBuf> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let colors = self.colors.clone();
        let pad = Sides::new(
            Some(colors.left.into()),
            Some(colors.right.into()),
            Some(colors.top.into()),
            Some(colors.bottom.into()),
        );
        cfg.set_padding_color(entity, pad);
    }
}

#[cfg(feature = "std")]
impl<R, D, C> TableOption<R, ColoredConfig, D> for PaddingColor<C>
where
    C: Into<ANSIBuf> + Clone,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<R, ColoredConfig>>::change(self, records, cfg, Entity::Global)
    }
}

impl<R, D, C> TableOption<R, CompactConfig, D> for PaddingColor<C>
where
    C: Into<ANSIStr<'static>> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        let c = self.colors.clone();
        let colors = Sides::new(c.left.into(), c.right.into(), c.top.into(), c.bottom.into());
        *cfg = cfg.set_padding_color(colors);
    }
}

impl<R, D, C> TableOption<R, CompactMultilineConfig, D> for PaddingColor<C>
where
    C: Into<ANSIStr<'static>> + Clone,
{
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        let c = self.colors.clone();
        let colors = Sides::new(c.left.into(), c.right.into(), c.top.into(), c.bottom.into());
        cfg.set_padding_color(colors);
    }
}
