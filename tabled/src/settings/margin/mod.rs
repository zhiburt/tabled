//! This module contains a Margin settings of a [`Table`].
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! # use tabled::{settings::Margin, Table};
//! # let data: Vec<&'static str> = Vec::new();
//! let table = Table::new(&data)
//!     .with(Margin::new(1, 1, 1, 1).fill('>', '<', 'V', '^'));
//! ```
//!
//! [`Table`]: crate::Table

use crate::{
    grid::{
        config::{CompactConfig, CompactMultilineConfig},
        config::{Indent, Sides},
    },
    settings::TableOption,
};

#[cfg(feature = "std")]
use crate::grid::config::ColoredConfig;

/// Margin is responsible for a left/right/top/bottom outer indent of a grid.
///
/// # Example
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// use tabled::{settings::{Margin, Style}, Table};
///
/// let data = vec!["Hello", "World", "!"];
///
/// let mut table = Table::new(data);
/// table
///     .with(Style::markdown())
///     .with(Margin::new(3, 3, 1, 0));
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "               \n",
///         "   | &str  |   \n",
///         "   |-------|   \n",
///         "   | Hello |   \n",
///         "   | World |   \n",
///         "   | !     |   ",
///     )
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Margin {
    indent: Sides<Indent>,
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
        }
    }

    /// The function, sets a characters for the margin on an each side.
    pub const fn fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.indent.left.fill = left;
        self.indent.right.fill = right;
        self.indent.top.fill = top;
        self.indent.bottom.fill = bottom;
        self
    }
}

impl From<Margin> for Sides<Indent> {
    fn from(value: Margin) -> Self {
        value.indent
    }
}

impl From<Sides<Indent>> for Margin {
    fn from(indent: Sides<Indent>) -> Self {
        Self { indent }
    }
}

#[cfg(feature = "std")]
impl<R, D> TableOption<R, ColoredConfig, D> for Margin {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let indent = self.indent;
        let margin = Sides::new(indent.left, indent.right, indent.top, indent.bottom);
        cfg.set_margin(margin);
    }
}

impl<R, D> TableOption<R, CompactConfig, D> for Margin {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_margin(self.indent);
    }
}

impl<R, D> TableOption<R, CompactMultilineConfig, D> for Margin {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_margin(self.indent);
    }
}
