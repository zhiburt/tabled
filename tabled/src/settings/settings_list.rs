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

fn combine_entity(x1: Entity, x2: Entity) -> Entity {
    use Entity::*;
    match (x1, x2) {
        (_, Global) => Global,
        (Global, _) => Global,
        (Column(_), Row(_)) => Global,
        (Column(a), Column(_)) => Column(a),
        (Column(a), Cell(_, _)) => Column(a),
        (Row(_), Column(_)) => Global,
        (Row(a), Row(_)) => Row(a),
        (Row(a), Cell(_, _)) => Row(a),
        (Cell(_, _), Column(a)) => Column(a),
        (Cell(_, _), Row(a)) => Row(a),
        (Cell(a, b), Cell(_, _)) => Cell(a, b),
    }
}
