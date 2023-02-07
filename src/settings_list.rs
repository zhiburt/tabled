use papergrid::{config::Entity, GridConfig};

use crate::{CellOption, TableOption};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Settings<A = EmptySettings, B = EmptySettings>(A, B);

impl Default for Settings<EmptySettings, EmptySettings> {
    fn default() -> Self {
        Self(EmptySettings, EmptySettings)
    }
}

impl Settings<(), ()> {
    pub const fn empty() -> Settings<EmptySettings, EmptySettings> {
        Settings(EmptySettings, EmptySettings)
    }
}

impl<A, B> Settings<A, B> {
    pub const fn new(settings1: A, settings2: B) -> Settings<A, B> {
        Settings(settings1, settings2)
    }

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

pub struct EmptySettings;

impl<R> CellOption<R> for EmptySettings {
    fn change(&mut self, _: &mut R, _: &mut GridConfig, _: Entity) {}
}

impl<R, D> TableOption<R, D> for EmptySettings {
    fn change(&mut self, _: &mut R, _: &mut GridConfig, _: &mut D) {}
}
