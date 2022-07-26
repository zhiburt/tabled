use fnv::FnvHashMap;

use crate::{Entity, Position};

#[derive(Debug, Clone, Default)]
pub(crate) struct EntityMap<T> {
    // we have a global type to allocate in on stack.
    // because most of the time no changes are made to the [EntityMap].
    pub(crate) global: T,
    pub(crate) columns: FnvHashMap<usize, T>,
    pub(crate) rows: FnvHashMap<usize, T>,
    pub(crate) cells: FnvHashMap<Position, T>,
}

impl<T> EntityMap<T> {
    pub(crate) fn new(global: T) -> Self {
        Self {
            global,
            rows: FnvHashMap::default(),
            columns: FnvHashMap::default(),
            cells: FnvHashMap::default(),
        }
    }

    pub(crate) fn lookup(&self, entity: Entity) -> &T {
        if self.rows.is_empty() && self.columns.is_empty() && self.cells.is_empty() {
            return &self.global;
        }

        match entity {
            Entity::Column(col) => self.columns.get(&col).unwrap_or(&self.global),
            Entity::Row(row) => self.rows.get(&row).unwrap_or(&self.global),
            Entity::Cell(row, col) => self
                .cells
                .get(&(row, col))
                .or_else(|| self.columns.get(&col))
                .or_else(|| self.rows.get(&row))
                .unwrap_or(&self.global),
            Entity::Global => &self.global,
        }
    }

    pub(crate) fn invalidate(&mut self, entity: Entity) {
        match entity {
            Entity::Global => {
                self.cells.clear();
                self.rows.clear();
                self.columns.clear();
            }
            Entity::Column(col) => self.cells.retain(|&(_, c), _| c != col),
            Entity::Row(row) => self.cells.retain(|&(r, _), _| r != row),
            Entity::Cell(_, _) => (),
        }
    }
}

impl<T: Clone> EntityMap<T> {
    pub(crate) fn set(&mut self, entity: Entity, value: T) {
        self.invalidate(entity);

        match entity {
            Entity::Column(col) => {
                for &row in self.rows.keys() {
                    self.cells.insert((row, col), value.clone());
                }

                self.columns.insert(col, value);
            }
            Entity::Row(row) => {
                for &col in self.columns.keys() {
                    self.cells.insert((row, col), value.clone());
                }

                self.rows.insert(row, value);
            }
            Entity::Cell(row, col) => {
                self.cells.insert((row, col), value);
            }
            Entity::Global => self.global = value,
        }
    }
}
