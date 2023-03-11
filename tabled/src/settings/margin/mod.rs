//! This module contains a Margin settings of a [`Table`].
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! use tabled::{settings::{Margin, Style}, Table};
//!
//! let data = vec!["Hello", "World", "!"];
//!
//! let mut table = Table::new(data);
//! table.with(Style::markdown()).with(Margin::new(3, 3, 1, 0));
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "               \n",
//!         "   | &str  |   \n",
//!         "   |-------|   \n",
//!         "   | Hello |   \n",
//!         "   | World |   \n",
//!         "   | !     |   ",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table

use crate::{
    grid::{
        color::StaticColor,
        compact::CompactConfig,
        config::{Indent, Sides},
    },
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::{
    grid::{
        color::AnsiColor,
        spanned::config::{ColoredMarginIndent, Offset},
    },
    tables::table::ColoredConfig,
};

/// Margin is responsible for a left/right/top/bottom outer indent of a grid.
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// # use tabled::{settings::Margin, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let table = Table::new(&data)
///     .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^'));
/// ```
#[derive(Debug, Clone)]
pub struct Margin<C = StaticColor> {
    indent: Sides<Indent>,
    colors: Option<Sides<C>>,
}

impl Margin {
    /// Construct's an Margin object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Margin::fill`] function.
    pub const fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self {
            indent: Sides::new(
                Indent::spaced(left),
                Indent::spaced(right),
                Indent::spaced(top),
                Indent::spaced(bottom),
            ),
            colors: None,
        }
    }
}

impl<Color> Margin<Color> {
    /// The function, sets a characters for the margin on an each side.
    pub const fn fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.indent.left.fill = left;
        self.indent.right.fill = right;
        self.indent.top.fill = top;
        self.indent.bottom.fill = bottom;
        self
    }

    /// The function, sets a characters for the margin on an each side.
    pub fn colorize<C>(self, left: C, right: C, top: C, bottom: C) -> Margin<C> {
        Margin {
            indent: self.indent,
            colors: Some(Sides::new(left, right, top, bottom)),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<R, D, C> TableOption<R, D, ColoredConfig> for Margin<C>
where
    C: Into<AnsiColor<'static>> + Clone,
{
    fn change(&mut self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let margin = cfg.get_margin_mut();
        margin.left = ColoredMarginIndent::new(self.indent.left, Offset::Begin(0), None);
        margin.right = ColoredMarginIndent::new(self.indent.right, Offset::Begin(0), None);
        margin.top = ColoredMarginIndent::new(self.indent.top, Offset::Begin(0), None);
        margin.bottom = ColoredMarginIndent::new(self.indent.bottom, Offset::Begin(0), None);

        if let Some(colors) = self.colors.clone() {
            margin.left.color = Some(colors.left.into());
            margin.right.color = Some(colors.right.into());
            margin.top.color = Some(colors.top.into());
            margin.bottom.color = Some(colors.bottom.into());
        }
    }
}

impl<R, D, C> TableOption<R, D, CompactConfig> for Margin<C>
where
    C: Into<StaticColor> + Clone,
{
    fn change(&mut self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_margin(self.indent);

        if let Some(c) = self.colors.clone() {
            // todo: make a new method (BECAUSE INTO doesn't work) try_into();
            let colors = Sides::new(c.left.into(), c.right.into(), c.top.into(), c.bottom.into());
            *cfg = cfg.set_margin_color(colors);
        }
    }
}
