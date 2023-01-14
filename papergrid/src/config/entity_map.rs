use fnv::FnvHashMap;

use super::{Entity, Position};

#[derive(Debug, Clone, Default)]
pub struct EntityMap<T> {
    // we have a global type to allocate in on stack.
    // because most of the time no changes are made to the [EntityMap].
    global: T,
    columns: FnvHashMap<usize, T>,
    rows: FnvHashMap<usize, T>,
    cells: FnvHashMap<Position, T>,
}

impl<T> EntityMap<T> {
    pub fn new(global: T) -> Self {
        Self {
            global,
            rows: FnvHashMap::default(),
            columns: FnvHashMap::default(),
            cells: FnvHashMap::default(),
        }
    }

    pub fn get(&self, entity: Entity) -> &T {
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

    pub fn remove(&mut self, entity: Entity) {
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
    pub fn set(&mut self, entity: Entity, value: T) {
        // why we do it exactly?
        self.remove(entity);

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
