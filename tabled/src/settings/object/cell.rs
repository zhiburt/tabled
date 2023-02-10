use crate::{
    grid::config::{Entity, Position},
    settings::object::Object,
};

/// Cell denotes a particular cell on a [`Table`].
///
/// For example such table has 4 cells.
/// Which indexes are (0, 0), (0, 1), (1, 0), (1, 1).
///
/// ```text
/// ┌───┬───┐
/// │ 0 │ 1 │
/// ├───┼───┤
/// │ 1 │ 2 │
/// └───┴───┘
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell(usize, usize);

impl Cell {
    /// Create new cell structure.
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }
}

impl From<Position> for Cell {
    fn from((row, col): Position) -> Self {
        Self(row, col)
    }
}

impl<I> Object<I> for Cell {
    type Iter = EntityOnce;

    fn cells(&self, _: &I) -> Self::Iter {
        EntityOnce::new(Some(Entity::Cell(self.0, self.1)))
    }
}

impl<I> Object<I> for Position {
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
