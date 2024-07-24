use crate::{grid::config::Entity, settings::TableOption};

#[cfg(feature = "std")]
use crate::settings::CellOption;

/// Settings is a combinator of [`TableOption`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Settings<A = EmptySettings, B = EmptySettings>(A, B);

impl Default for Settings<EmptySettings, EmptySettings> {
    fn default() -> Self {
        Self(EmptySettings, EmptySettings)
    }
}

impl Settings<(), ()> {
    /// Creates an empty list.
    pub const fn empty() -> Settings<EmptySettings, EmptySettings> {
        Settings(EmptySettings, EmptySettings)
    }
}

impl<A, B> Settings<A, B> {
    /// Creates a new combinator.
    pub const fn new(settings1: A, settings2: B) -> Settings<A, B> {
        Settings(settings1, settings2)
    }

    /// Add an option to a combinator.
    pub const fn with<C>(self, settings: C) -> Settings<Self, C> {
        Settings(self, settings)
    }
}

#[cfg(feature = "std")]
impl<R, C, A, B> CellOption<R, C> for Settings<A, B>
where
    A: CellOption<R, C>,
    B: CellOption<R, C>,
{
    fn change(self, records: &mut R, cfg: &mut C, entity: Entity) {
        self.0.change(records, cfg, entity);
        self.1.change(records, cfg, entity);
    }

    fn hint_change(&self) -> Option<Entity> {
        match (self.0.hint_change(), self.1.hint_change()) {
            (None, None) => None,
            (Some(a), Some(b)) => Some(combine_entity(a, b)),
            (None, value) => value,
            (value, None) => value,
        }
    }
}

impl<R, D, C, A, B> TableOption<R, C, D> for Settings<A, B>
where
    A: TableOption<R, C, D>,
    B: TableOption<R, C, D>,
{
    fn change(self, records: &mut R, cfg: &mut C, dims: &mut D) {
        self.0.change(records, cfg, dims);
        self.1.change(records, cfg, dims);
    }

    fn hint_change(&self) -> Option<Entity> {
        match (self.0.hint_change(), self.1.hint_change()) {
            (None, None) => None,
            (Some(a), Some(b)) => Some(combine_entity(a, b)),
            (None, value) => value,
            (value, None) => value,
        }
    }
}

/// A marker structure to be able to create an empty [`Settings`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptySettings;

#[cfg(feature = "std")]
impl<R, C> CellOption<R, C> for EmptySettings {
    fn change(self, _: &mut R, _: &mut C, _: Entity) {}
}

impl<R, D, C> TableOption<R, C, D> for EmptySettings {
    fn change(self, _: &mut R, _: &mut C, _: &mut D) {}
}

pub(crate) fn combine_entity(x1: Entity, x2: Entity) -> Entity {
    use Entity::*;

    match (x1, x2) {
        (Column(a), Column(b)) if a == b => Column(a),
        (Column(a), Cell(_, b)) if a == b => Column(a),
        (Row(a), Row(b)) if a == b => Row(a),
        (Row(a), Cell(b, _)) if a == b => Row(a),
        (Cell(_, a), Column(b)) if a == b => Column(a),
        (Cell(a, _), Row(b)) if a == b => Row(a),
        (Cell(a, b), Cell(a1, b1)) if a == a1 && b == b1 => Cell(a, b),
        _ => Global,
    }
}
