use crate::{
    grid::config::Entity, object::Object, records::ExactRecords, records::Records, CellOption,
    TableOption,
};

/// Modify structure provide an abstraction, to be able to apply
/// a set of [`CellOption`]s to the same object.
///
/// Be aware that the settings are applied all to a cell at a time.
/// So sometimes you may need to make a several calls of [`Modify`] in order to achieve the desired affect.
#[derive(Debug)]
pub struct Modify<O> {
    obj: O,
}

impl<O> Modify<O> {
    /// Creates a new [`Modify`] without any options.
    pub fn new(obj: O) -> Self {
        Self { obj }
    }

    /// It's a generic function which stores a [`CellOption`].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [`Table`].
    ///     [`Table`] will be changed only after passing [`Modify`] object to [`Table::with`].
    ///
    /// [`Table`]: crate::Table
    /// [`Table::with`]: crate::Table::with
    pub fn with<M>(self, s: M) -> ModifyList<O, M> {
        ModifyList {
            obj: self.obj,
            modifiers: s,
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
    pub fn with<M2>(self, s: M2) -> ModifyList<O, CellSettingsList<M1, M2>> {
        ModifyList {
            obj: self.obj,
            modifiers: CellSettingsList {
                s1: self.modifiers,
                s2: s,
            },
        }
    }
}

impl<O, M, R, D> TableOption<R, D> for ModifyList<O, M>
where
    O: Object<R>,
    M: CellOption<R>,
    R: Records + ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut papergrid::GridConfig, _: &mut D) {
        for entity in self.obj.cells(&records) {
            self.modifiers.change(records, cfg, entity);
        }
    }
}

/// This is a container of [`CellOption`]s.
#[derive(Debug)]
pub struct CellSettingsList<S1, S2> {
    s1: S1,
    s2: S2,
}

impl<M1, M2, R> CellOption<R> for CellSettingsList<M1, M2>
where
    M1: CellOption<R>,
    M2: CellOption<R>,
{
    fn change(&mut self, records: &mut R, cfg: &mut papergrid::GridConfig, entity: Entity) {
        self.s1.change(records, cfg, entity);
        self.s2.change(records, cfg, entity);
    }
}
