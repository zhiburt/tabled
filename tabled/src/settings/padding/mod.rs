//! This module contains a [`Padding`] setting of a cell on a [`Table`].
//!
//! # Example
//!
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! # use tabled::{settings::{Style, Padding, object::Rows, Modify}, Table};
//! # let data: Vec<&'static str> = Vec::new();
//! let table = Table::new(&data)
//!     .with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).fill('>', '<', '^', 'V')));
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
use crate::{
    grid::{config::ColoredConfig, config::Entity},
    settings::padding_expand::PaddingExpand,
    settings::CellOption,
};

/// Padding is responsible for a left/right/top/bottom inner indent of a particular cell.
///
/// # Example
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// use tabled::{
///     Table,
///     settings::{Padding, Style, Modify, object::Cell},
/// };
///
/// let table = Table::new("2022".chars())
///     .with(Style::modern())
///     .with(Modify::new((2, 0)).with(Padding::new(1, 1, 2, 2)))
///     .to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "┌──────┐\n",
///         "│ char │\n",
///         "├──────┤\n",
///         "│ 2    │\n",
///         "├──────┤\n",
///         "│      │\n",
///         "│      │\n",
///         "│ 0    │\n",
///         "│      │\n",
///         "│      │\n",
///         "├──────┤\n",
///         "│ 2    │\n",
///         "├──────┤\n",
///         "│ 2    │\n",
///         "└──────┘",
///     ),
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    indent: Sides<Indent>,
}

impl Padding {
    /// Construct's an Padding object.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Padding::fill`] function.
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

    /// Construct's an Padding object with all sides set to 0.
    ///
    /// It uses space(' ') as a default fill character.
    /// To set a custom character you can use [`Padding::fill`] function.
    pub const fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// The function, sets a characters for the padding on an each side.
    pub const fn fill(mut self, left: char, right: char, top: char, bottom: char) -> Self {
        self.indent.left.fill = left;
        self.indent.right.fill = right;
        self.indent.top.fill = top;
        self.indent.bottom.fill = bottom;
        self
    }

    #[cfg(feature = "std")]
    /// Construct's an PaddingExpand object.
    pub const fn expand(horizontal: bool) -> PaddingExpand {
        if horizontal {
            PaddingExpand::Horizontal
        } else {
            PaddingExpand::Vertical
        }
    }
}

#[cfg(feature = "std")]
impl<R> CellOption<R, ColoredConfig> for Padding {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let indent = self.indent;
        let pad = Sides::new(indent.left, indent.right, indent.top, indent.bottom);
        cfg.set_padding(entity, pad);
    }
}

#[cfg(feature = "std")]
impl<R, D> TableOption<R, ColoredConfig, D> for Padding {
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<R, ColoredConfig>>::change(self, records, cfg, Entity::Global)
    }
}

impl<R, D> TableOption<R, CompactConfig, D> for Padding {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_padding(self.indent);
    }
}

impl<R, D> TableOption<R, CompactMultilineConfig, D> for Padding {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_padding(self.indent);
    }
}
