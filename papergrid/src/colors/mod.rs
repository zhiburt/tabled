//! A module which contains [Colors] trait and its blanket implementations.

mod nocolors;

use crate::{ansi::ANSIFmt, config::Position};

pub use nocolors::*;

/// A trait which represents map of colors.
pub trait Colors {
    /// Color implementation.
    type Color: ANSIFmt;

    /// Returns a color for a given position.
    fn get_color(&self, pos: Position) -> Option<&Self::Color>;

    /// Verifies whether a map is empty or not.
    fn is_empty(&self) -> bool;
}

impl<C> Colors for &'_ C
where
    C: Colors,
{
    type Color = C::Color;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        C::get_color(self, pos)
    }

    fn is_empty(&self) -> bool {
        C::is_empty(self)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for std::collections::HashMap<Position, C>
where
    C: ANSIFmt,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }

    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for std::collections::BTreeMap<Position, C>
where
    C: ANSIFmt,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }

    fn is_empty(&self) -> bool {
        std::collections::BTreeMap::is_empty(self)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for crate::config::spanned::EntityMap<Option<C>>
where
    C: ANSIFmt + PartialEq,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(pos).as_ref()
    }

    fn is_empty(&self) -> bool {
        self.is_empty() && self.as_ref().is_none()
    }
}
