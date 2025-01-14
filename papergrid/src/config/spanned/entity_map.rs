use std::collections::HashMap;

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

    /// Verifies whether anything was set beside a global entry.
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty() && self.rows.is_empty() && self.cells.is_empty()
    }

    /// Get a value for an [`Entity`].
    pub fn get(&self, pos: Position) -> &T {
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
            .get(&pos)
            .or_else(|| self.columns.get(&pos.col()))
            .or_else(|| self.rows.get(&pos.row()))
            .unwrap_or(&self.global)
    }

    /// Removes a value for an [`Entity`].
    pub fn remove(&mut self, entity: Entity) {
        match entity {
            Entity::Global => {
                self.cells.clear();
                self.rows.clear();
                self.columns.clear();
            }
            Entity::Column(col) => self.cells.retain(|pos, _| pos.col() != col),
            Entity::Row(row) => self.cells.retain(|pos, _| pos.row() != row),
            Entity::Cell(row, col) => {
                self.cells.remove(&Position::new(row, col));
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
                    self.cells.insert(Position::new(row, col), value.clone());
                }

                self.columns.insert(col, value);
            }
            Entity::Row(row) => {
                for &col in self.columns.keys() {
                    self.cells.insert(Position::new(row, col), value.clone());
                }

                self.rows.insert(row, value);
            }
            Entity::Cell(row, col) => {
                self.cells.insert(Position::new(row, col), value);
            }
            Entity::Global => {
                self.remove(Entity::Global);
                self.global = value
            }
        }
    }
}

impl<T> From<EntityMap<T>> for HashMap<Entity, T> {
    fn from(value: EntityMap<T>) -> Self {
        let mut m = HashMap::new();
        m.insert(Entity::Global, value.global);

        for (pos, value) in value.cells {
            m.insert(Entity::from(pos), value);
        }

        for (row, value) in value.rows {
            m.insert(Entity::Row(row), value);
        }

        for (col, value) in value.columns {
            m.insert(Entity::Column(col), value);
        }

        m
    }
}

impl<T> AsRef<T> for EntityMap<T> {
    fn as_ref(&self) -> &T {
        &self.global
    }
}
