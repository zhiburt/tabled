//! A module which contains [Colors] trait and its blanket implementations.

use std::collections::{BTreeMap, HashMap};

use crate::{
    color::{AnsiColor, Color},
    config::{Entity, EntityMap, Position},
};

/// A trait which represents map of colors.
pub trait Colors {
    /// Color implementation.
    type Color: Color;

    /// Returns a color for a given position.
    fn get_color(&self, pos: Position) -> Option<&Self::Color>;
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

impl<C> Colors for HashMap<Position, C>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }
}

impl<C> Colors for BTreeMap<Position, C>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(&pos)
    }
}

impl<C> Colors for EntityMap<Option<C>>
where
    C: Color,
{
    type Color = C;

    fn get_color(&self, pos: Position) -> Option<&Self::Color> {
        self.get(Entity::Cell(pos.0, pos.1)).as_ref()
    }
}

/// The structure represents empty [`Colors`] map.
#[derive(Debug, Default, Clone)]
pub struct NoColors;

impl Colors for NoColors {
    type Color = AnsiColor<'static>;

    fn get_color(&self, _: Position) -> Option<&Self::Color> {
        None
    }
}
