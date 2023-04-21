use crate::settings::TableOption;

#[cfg(feature = "std")]
use crate::grid::config::Entity;
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
}

impl<R, D, C, A, B> TableOption<R, D, C> for Settings<A, B>
where
    A: TableOption<R, D, C>,
    B: TableOption<R, D, C>,
{
    fn change(self, records: &mut R, cfg: &mut C, dims: &mut D) {
        self.0.change(records, cfg, dims);
        self.1.change(records, cfg, dims);
    }
}

/// A marker structure to be able to create an empty [`Settings`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmptySettings;

#[cfg(feature = "std")]
impl<R, C> CellOption<R, C> for EmptySettings {
    fn change(self, _: &mut R, _: &mut C, _: Entity) {}
}

impl<R, D, C> TableOption<R, D, C> for EmptySettings {
    fn change(self, _: &mut R, _: &mut C, _: &mut D) {}
}
