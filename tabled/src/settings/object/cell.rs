use crate::{grid::config::Entity, settings::object::Object};

/// Cell denotes a particular cell on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Cell(pub usize, pub usize);

impl<I> Object<I> for Cell {
    type Iter = EntityOnce;

    fn cells(&self, _: &I) -> Self::Iter {
        EntityOnce::new(Some(Entity::Cell(self.0, self.1)))
    }
}

/// An [`Iterator`] which returns an entity once.
#[derive(Debug)]
pub struct EntityOnce {
    entity: Option<Entity>,
}

impl EntityOnce {
    pub(crate) const fn new(entity: Option<Entity>) -> Self {
        Self { entity }
    }
}

impl Iterator for EntityOnce {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.entity.take()
    }
}
