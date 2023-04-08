use fnv::FnvHashMap;

use crate::config::{Entity, Position};

/// A structure to keep information for [`Entity`] as a key.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EntityMap<T> {
    // we have a global type to allocate in on stack.
    // because most of the time no changes are made to the [`EntityMap`].
    global: T,
    columns: FnvHashMap<usize, T>,
    rows: FnvHashMap<usize, T>,
    cells: FnvHashMap<Position, T>,
}

impl<T> EntityMap<T> {
    /// Creates an empty [`EntityMap`].
    pub fn new(global: T) -> Self {
        Self {
            global,
            rows: FnvHashMap::default(),
            columns: FnvHashMap::default(),
            cells: FnvHashMap::default(),
        }
    }

    /// Get a value for an [`Entity`].
    pub fn get(&self, entity: Entity) -> &T {
        if self.rows.is_empty() && self.columns.is_empty() && self.cells.is_empty() {
            return &self.global;
        }

        match entity {
            Entity::Column(col) => self.columns.get(&col).unwrap_or(&self.global),
            Entity::Row(row) => self.rows.get(&row).unwrap_or(&self.global),
            Entity::Cell(row, col) => {
                // todo: optimize;
                //
                // Cause we can change rows/columns/cells separately we need to check them separately.
                // But we often doing this checks in `Grid::fmt` and I believe if we could optimize it it could be beneficial.
                //
                // Haven't found a solution for that yet.
                //
                // I was wondering if there is a hash function like.
                // Apparently it doesn't make sense cause we will reset columns/rows on cell insert which is not what we want.
                //
                // ```
                // hash(column, row) == hash(column) == hash(row)
                // ```
                //
                // ref: https://opendsa-server.cs.vt.edu/ODSA/Books/Everything/html/Sparse.html
                // ref: https://users.rust-lang.org/t/make-hash-return-same-value-whather-the-order-of-element-of-a-tuple/69932/13

                self.cells
                    .get(&(row, col))
                    .or_else(|| self.columns.get(&col))
                    .or_else(|| self.rows.get(&row))
                    .unwrap_or(&self.global)
            }
            Entity::Global => &self.global,
        }
    }

    /// Removes a value for an [`Entity`].
    pub fn remove(&mut self, entity: Entity) {
        match entity {
            Entity::Global => {
                self.cells.clear();
                self.rows.clear();
                self.columns.clear();
            }
            Entity::Column(col) => self.cells.retain(|&(_, c), _| c != col),
            Entity::Row(row) => self.cells.retain(|&(r, _), _| r != row),
            Entity::Cell(row, col) => {
                self.cells.remove(&(row, col));
            }
        }
    }
}

impl<T: Clone> EntityMap<T> {
    /// Set a value for an [`Entity`].
    pub fn insert(&mut self, entity: Entity, value: T) {
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
            Entity::Global => {
                self.remove(Entity::Global);
                self.global = value
            }
        }
    }
}
