//! A module which contains [Colors] trait and its blanket implementations.

use crate::{color::Color, config::Position};

/// A trait which represents map of colors.
pub trait Colors {
    /// Color implementation.
    type Color: Color;

    /// Returns a color for a given position.
    fn get_color(&self, pos: (usize, usize)) -> Option<&Self::Color>;
}

impl<C> Colors for &'_ C
where
    C: Colors,
{
    type Color = C::Color;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        C::get_color(self, pos)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for std::collections::HashMap<Position, C>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for std::collections::BTreeMap<Position, C>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }
}

#[cfg(feature = "std")]
impl<C> Colors for crate::config::spanned::EntityMap<Option<C>>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(pos.into()).as_ref()
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
}

/// A color which is actually has not value.
#[derive(Debug)]
pub struct EmptyColor;

impl Color for EmptyColor {
    fn fmt_prefix<W: core::fmt::Write>(&self, _: &mut W) -> core::fmt::Result {
        Ok(())
    }

    fn colorize<W: core::fmt::Write>(&self, _: &mut W, _: &str) -> core::fmt::Result {
        Ok(())
    }

    fn fmt_suffix<W: core::fmt::Write>(&self, _: &mut W) -> core::fmt::Result {
        Ok(())
    }
}
