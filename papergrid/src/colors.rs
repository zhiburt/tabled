use std::collections::{BTreeMap, HashMap};

use crate::{
    color::Color,
    config::{Entity, EntityMap, Position},
};

pub trait Colors {
    type Color: Color;

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
