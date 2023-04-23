use crate::{
    grid::records::{ExactRecords, Records},
    settings::{object::Object, CellOption, Settings, TableOption},
};

/// Modify structure provide an abstraction, to be able to apply
/// a set of [`CellOption`]s to the same object.
///
/// Be aware that the settings are applied all to a cell at a time.
/// So sometimes you may need to make a several calls of [`Modify`] in order to achieve the desired affect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Modify<O> {
    obj: O,
}

impl<O> Modify<O> {
    /// Creates a new [`Modify`] without any options.
    pub const fn new(obj: O) -> Self {
        Self { obj }
    }

    /// A function which combines together [`Modify::new`] and [`Modify::with`] calls.
    pub const fn list<M>(obj: O, next: M) -> ModifyList<O, M> {
        ModifyList {
            obj,
            modifiers: next,
        }
    }

    /// It's a generic function which stores a [`CellOption`].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [`Table`].
    ///     [`Table`] will be changed only after passing [`Modify`] object to [`Table::with`].
    ///
    /// [`Table`]: crate::Table
    /// [`Table::with`]: crate::Table::with
    pub fn with<M>(self, next: M) -> ModifyList<O, M> {
        ModifyList {
            obj: self.obj,
            modifiers: next,
        }
    }
}

/// This is a container of [`CellOption`]s which are applied to a set [`Object`].
#[derive(Debug)]
pub struct ModifyList<O, S> {
    obj: O,
    modifiers: S,
}

impl<O, M1> ModifyList<O, M1> {
    /// With a generic function which stores a [`CellOption`].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [`Table`].
    ///     [`Table`] will be changed only after passing [`Modify`] object to [`Table::with`].
    ///
    /// [`Table`]: crate::Table
    /// [`Table::with`]: crate::Table::with
    pub fn with<M2>(self, next: M2) -> ModifyList<O, Settings<M1, M2>> {
        ModifyList {
            obj: self.obj,
            modifiers: Settings::new(self.modifiers, next),
        }
    }
}

impl<O, M, R, D, C> TableOption<R, D, C> for ModifyList<O, M>
where
    O: Object<R>,
    M: CellOption<R, C> + Clone,
    R: Records + ExactRecords,
{
    fn change(self, records: &mut R, cfg: &mut C, _: &mut D) {
        for entity in self.obj.cells(records) {
            self.modifiers.clone().change(records, cfg, entity);
        }
    }
}
