use papergrid::{Entity, Grid};

use crate::{object::Object, CellOption, TableOption};

/// Modify structure provide an abstraction, to be able to apply
/// a set of [CellOption]s to the same object.
///
/// Be aware that the settings are applied all to a cell at a time.
/// So sometimes you may need to make a several calls of [Modify] in order to achieve the desired affect.
pub struct Modify<O> {
    obj: O,
}

impl<O> Modify<O>
where
    O: Object,
{
    /// Creates a new [Modify] without any options.
    pub fn new(obj: O) -> Self {
        Self { obj }
    }

    /// It's a generic function which stores a [CellOption].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [Table].
    ///     [Table] will be changed only after passing [Modify] object to [Table::with].
    pub fn with<F>(self, s: F) -> ModifyList<O, F>
    where
        F: CellOption,
    {
        ModifyList {
            obj: self.obj,
            modifiers: s,
        }
    }
}

/// ModifyList is a container of [CellOption]s which are applied to a set [Object].
pub struct ModifyList<O, S> {
    obj: O,
    modifiers: S,
}

impl<O, S> ModifyList<O, S>
where
    O: Object,
    S: CellOption,
{
    /// With a generic function which stores a [CellOption].
    ///
    /// IMPORTANT:
    ///     The function *doesn't* changes a [Table].
    ///     [Table] will be changed only after passing [Modify] object to [Table::with].
    pub fn with<F>(self, s: F) -> ModifyList<O, CellSettingsList<S, F>>
    where
        F: CellOption,
    {
        ModifyList {
            obj: self.obj,
            modifiers: CellSettingsList {
                s1: self.modifiers,
                s2: s,
            },
        }
    }
}

impl<O, S> TableOption for ModifyList<O, S>
where
    O: Object,
    S: CellOption,
{
    fn change(&mut self, grid: &mut Grid) {
        match self.obj.as_entity(grid.count_rows(), grid.count_columns()) {
            Some(entity) => self.modifiers.change_cell(grid, entity),
            None => {
                let cells = self.obj.cells(grid.count_rows(), grid.count_columns());
                for (row, col) in cells {
                    self.modifiers.change_cell(grid, Entity::Cell(row, col));
                }
            }
        }
    }
}

/// CellSettingsList is a container of [CellOption]s.
pub struct CellSettingsList<S1, S2> {
    s1: S1,
    s2: S2,
}

impl<S1, S2> CellOption for CellSettingsList<S1, S2>
where
    S1: CellOption,
    S2: CellOption,
{
    fn change_cell(&mut self, grid: &mut Grid, entity: Entity) {
        self.s1.change_cell(grid, entity);
        self.s2.change_cell(grid, entity);
    }
}

/// An utility trait for a different interface of [Modify] creation.
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
    /// Returns a Modify container of [Object]
    fn modify(self) -> Modify<Self> {
        Modify::new(self)
    }
}

impl<O> ModifyObject for O where O: Object {}
