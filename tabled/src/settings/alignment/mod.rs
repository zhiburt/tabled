//! This module contains an [`Alignment`] setting for cells on the [`Table`].
//!
//! # Example
//!
#![cfg_attr(feature = "std", doc = "```")]
#![cfg_attr(not(feature = "std"), doc = "```ignore")]
//! # use tabled::{Table, settings::{Alignment, Modify, object::Rows}};
//! # let data: Vec<&'static str> = Vec::new();
//! let mut table = Table::new(&data);
//! table.with(Modify::new(Rows::single(0)).with(Alignment::center()));
//! ```
//!
//! [`Table`]: crate::Table

use crate::{
    grid::config::CompactConfig,
    grid::config::{AlignmentHorizontal, AlignmentVertical, CompactMultilineConfig},
    settings::TableOption,
};

use AlignmentInner::*;

#[cfg(feature = "std")]
use crate::grid::config::{ColoredConfig, Entity};

/// Alignment represent a horizontal and vertical alignment setting for any cell on a [`Table`].
///
/// An alignment strategy can be set by [`AlignmentStrategy`].
///
/// # Example
///
#[cfg_attr(feature = "std", doc = "```")]
#[cfg_attr(not(feature = "std"), doc = "```ignore")]
/// use tabled::{
///     Table,
///     settings::{
///         formatting::AlignmentStrategy,
///         object::Segment, Alignment, Modify, Style,
///     }
/// };
///
/// let data = [
///     ["1", "2", "3"],
///     ["Some\nMulti\nLine\nText", "and a line", "here"],
///     ["4", "5", "6"],
/// ];
///
/// let mut table = Table::new(&data);
/// table
///     .with(Style::modern())
///     .with(
///         Modify::new(Segment::all())
///             .with(Alignment::right())
///             .with(Alignment::center())
///             .with(AlignmentStrategy::PerCell)
///     );
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "┌───────┬────────────┬──────┐\n",
///         "│   0   │     1      │  2   │\n",
///         "├───────┼────────────┼──────┤\n",
///         "│   1   │     2      │  3   │\n",
///         "├───────┼────────────┼──────┤\n",
///         "│ Some  │ and a line │ here │\n",
///         "│ Multi │            │      │\n",
///         "│ Line  │            │      │\n",
///         "│ Text  │            │      │\n",
///         "├───────┼────────────┼──────┤\n",
///         "│   4   │     5      │  6   │\n",
///         "└───────┴────────────┴──────┘",
///     ),
/// )
/// ```
///
/// [`Table`]: crate::Table
/// [`AlignmentStrategy`]: crate::settings::formatting::AlignmentStrategy
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alignment {
    inner: AlignmentInner,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum AlignmentInner {
    /// A horizontal alignment.
    Horizontal(AlignmentHorizontal),
    /// A vertical alignment.
    Vertical(AlignmentVertical),
}

impl Alignment {
    /// Left constructs a horizontal alignment to [`AlignmentHorizontal::Left`]
    pub fn left() -> Self {
        Self::horizontal(AlignmentHorizontal::Left)
    }

    /// Right constructs a horizontal alignment to [`AlignmentHorizontal::Right`]
    ///
    /// ## Notice
    ///
    /// When you use [`MinWidth`] the alignment might not work as you expected.
    /// You could try to apply [`TrimStrategy`] which may help.
    ///
    /// [`MinWidth`]: crate::settings::width::MinWidth
    /// [`TrimStrategy`]: crate::settings::formatting::TrimStrategy
    pub fn right() -> Self {
        Self::horizontal(AlignmentHorizontal::Right)
    }

    /// Center constructs a horizontal alignment to [`AlignmentHorizontal::Center`]
    ///
    /// ## Notice
    ///
    /// When you use [`MinWidth`] the alignment might not work as you expected.
    /// You could try to apply [`TrimStrategy`] which may help.
    ///
    /// [`MinWidth`]: crate::settings::width::MinWidth
    /// [`TrimStrategy`]: crate::settings::formatting::TrimStrategy
    pub const fn center() -> Self {
        Self::horizontal(AlignmentHorizontal::Center)
    }

    /// Top constructs a vertical alignment to [`AlignmentVertical::Top`]
    pub const fn top() -> Self {
        Self::vertical(AlignmentVertical::Top)
    }

    /// Bottom constructs a vertical alignment to [`AlignmentVertical::Bottom`]
    pub const fn bottom() -> Self {
        Self::vertical(AlignmentVertical::Bottom)
    }

    /// `Center_vertical` constructs a vertical alignment to [`AlignmentVertical::Center`]
    pub const fn center_vertical() -> Self {
        Self::vertical(AlignmentVertical::Center)
    }

    /// Returns an alignment with the given horizontal alignment.
    const fn horizontal(alignment: AlignmentHorizontal) -> Self {
        Self::new(Horizontal(alignment))
    }

    /// Returns an alignment with the given vertical alignment.
    const fn vertical(alignment: AlignmentVertical) -> Self {
        Self::new(Vertical(alignment))
    }

    const fn new(inner: AlignmentInner) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "std")]
impl<R> crate::settings::CellOption<R, ColoredConfig> for Alignment {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        match self.inner {
            Horizontal(a) => cfg.set_alignment_horizontal(entity, a),
            Vertical(a) => cfg.set_alignment_vertical(entity, a),
        }
    }
}

#[cfg(feature = "std")]
impl<R, D> TableOption<R, D, ColoredConfig> for Alignment {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        match self.inner {
            Horizontal(a) => cfg.set_alignment_horizontal(Entity::Global, a),
            Vertical(a) => cfg.set_alignment_vertical(Entity::Global, a),
        }
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for Alignment {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        if let Horizontal(a) = self.inner {
            *cfg = cfg.set_alignment_horizontal(a)
        }
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for Alignment {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        match self.inner {
            Horizontal(a) => *cfg = cfg.set_alignment_horizontal(a),
            Vertical(a) => *cfg = cfg.set_alignment_vertical(a),
        }
    }
}
