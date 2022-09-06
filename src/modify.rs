use papergrid::{records::Records, Entity};

use crate::{object::Object, CellOption, Table, TableOption};

/// Modify structure provide an abstraction, to be able to apply
/// a set of [`CellOption`]s to the same object.
///
/// Be aware that the settings are applied all to a cell at a time.
/// So sometimes you may need to make a several calls of [`Modify`] in order to achieve the desired affect.
#[derive(Debug)]
pub struct Modify<O> {
    obj: O,
}

impl<O> Modify<O>
where
    O: Object,
{
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

impl<O, M1> ModifyList<O, M1>
where
    O: Object,
{
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

impl<O, M, R> TableOption<R> for ModifyList<O, M>
where
    O: Object,
    M: CellOption<R>,
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        for entity in self.obj.cells(table) {
            self.modifiers.change_cell(table, entity);
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
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        self.s1.change_cell(table, entity);
        self.s2.change_cell(table, entity);
    }
}

/// An utility trait for a different interface of [`Modify`] creation.
///
/// # Example
///
/// ```
/// # use tabled::{object::Cell, Modify, ModifyObject};
/// // 1st way to create modification container
/// let m = Modify::new(Cell(1, 1));
/// // 2nd way to create modification container
/// let m = Cell(1, 1).modify();
/// ```
pub trait ModifyObject: Object {
    /// Returns a Modify container of [`Object`]
    fn modify(self) -> Modify<Self> {
        Modify::new(self)
    }
}

impl<O> ModifyObject for O where O: Object {}
