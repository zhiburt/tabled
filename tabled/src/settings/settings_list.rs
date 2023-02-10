use crate::{
    grid::config::{Entity, GridConfig},
    settings::{CellOption, TableOption},
};

/// Settigns is a combinator of [`CellOption`] and [`TableOption`].
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

impl<R, A, B> CellOption<R> for Settings<A, B>
where
    A: CellOption<R>,
    B: CellOption<R>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        self.0.change(records, cfg, entity);
        self.1.change(records, cfg, entity);
    }
}

impl<R, D, A, B> TableOption<R, D> for Settings<A, B>
where
    A: TableOption<R, D>,
    B: TableOption<R, D>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dims: &mut D) {
        self.0.change(records, cfg, dims);
        self.1.change(records, cfg, dims);
    }
}

/// A marker structure to be able to create an empty [`Settings`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptySettings;

impl<R> CellOption<R> for EmptySettings {
    fn change(&mut self, _: &mut R, _: &mut GridConfig, _: Entity) {}
}

impl<R, D> TableOption<R, D> for EmptySettings {
    fn change(&mut self, _: &mut R, _: &mut GridConfig, _: &mut D) {}
}
