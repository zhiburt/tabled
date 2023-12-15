//! A module which contains [Colors] trait and its blanket implementations.

use crate::{ansi::ANSIFmt, config::Position};

/// A trait which represents map of colors.
pub trait Colors {
    /// Color implementation.
    type Color: ANSIFmt;

    /// Returns a color for a given position.
    fn get_color(&self, pos: (usize, usize)) -> Option<&Self::Color>;

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
    C: ANSIFmt,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(pos.into()).as_ref()
    }

    fn is_empty(&self) -> bool {
        crate::config::spanned::EntityMap::is_empty(self)
            && self.get(crate::config::Entity::Global).is_none()
    }
}

/// The structure represents empty [`Colors`] map.
#[derive(Debug, Default, Clone)]
pub struct NoColors;

impl Colors for NoColors {
    type Color = EmptyColor;

    fn get_color(&self, _: Position) -> Option<&Self::Color> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }
}

/// A color which is actually has not value.
#[derive(Debug)]
pub struct EmptyColor;

impl ANSIFmt for EmptyColor {
    fn fmt_ansi_prefix<W: core::fmt::Write>(&self, _: &mut W) -> core::fmt::Result {
        Ok(())
    }

    fn fmt_ansi_suffix<W: core::fmt::Write>(&self, _: &mut W) -> core::fmt::Result {
        Ok(())
    }
}
