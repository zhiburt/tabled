// use core::cell::UnsafeCell;
// use std::collections::HashMap;

// use rtree_rs::{RTree, Rect};

// use crate::config::{Entity, Position};

// /// A structure to keep information for [`Entity`] as a key.
// #[derive(Debug, Default, Clone, PartialEq, Eq)]
// pub struct EntityMap<T> {
//     // we have a global type to allocate in on stack.
//     // because most of the time no changes are made to the [`EntityMap`].
//     global: T,
//     // todo: Maybe maybe we shall check whether tree is getting too big and switch back to a 3 hash tables?
//     tree: RTree<2, usize, T>,
// }

// impl<T> EntityMap<T> {
//     /// Creates an empty [`EntityMap`].
//     pub fn new(global: T) -> Self
//     where
//         T: PartialEq,
//     {
//         Self {
//             global,
//             tree: RTree::new(),
//         }
//     }

//     /// Verifies whether anything was set beside a global entry.
//     pub fn is_empty(&self) -> bool
//     where
//         T: PartialEq,
//     {
//         self.tree.is_empty()
//     }

//     /// Get a value for an [`Entity`].
//     pub fn get(&self, pos: Position) -> &T
//     where
//         T: PartialEq,
//     {
//         if self.is_empty() {
//             return &self.global;
//         }

//         let value = tree_search(&self.tree, pos, pos);

//         value.unwrap_or(&self.global)
//     }

//     /// Removes a value for an [`Entity`].
//     pub fn remove(&mut self, entity: Entity)
//     where
//         T: PartialEq + Clone,
//     {
//         let mut new = RTree::new();

//         // todo: Rtree contribution to remove effectively

//         match entity {
//             Entity::Global => {}
//             Entity::Column(col) => {
//                 for item in self.tree.iter() {
//                     if item.rect.min[1] != col && item.rect.max[1] != col {
//                         new.insert(item.rect, item.data.clone());
//                     }
//                 }
//             }
//             Entity::Row(row) => {
//                 for item in self.tree.iter() {
//                     if item.rect.min[0] != row && item.rect.max[0] != row {
//                         new.insert(item.rect, item.data.clone());
//                     }
//                 }
//             }
//             Entity::Cell(row, col) => {
//                 for item in self.tree.iter() {
//                     if item.rect.min != [row, col] && item.rect.max != [row, col] {
//                         new.insert(item.rect, item.data.clone());
//                     }
//                 }
//             }
//         }

//         self.tree = new;
//     }

//     /// Set a value for an [`Entity`].
//     pub fn insert(&mut self, entity: Entity, value: T)
//     where
//         T: PartialEq,
//     {
//         match entity {
//             Entity::Column(col) => {
//                 self.tree
//                     .insert(Rect::new([0, col], [usize::MAX, col]), value);
//             }
//             Entity::Row(row) => {
//                 self.tree
//                     .insert(Rect::new([row, 0], [row, usize::MAX]), value);
//             }
//             Entity::Cell(row, col) => {
//                 self.tree.insert(Rect::new([row, col], [row, col]), value);
//             }
//             Entity::Global => {
//                 self.tree = RTree::new();
//                 self.global = value
//             }
//         }
//     }

//     /// Get a value for an [`Entity`].
//     pub fn modify<F>(&mut self, target: Entity, mut f: F)
//     where
//         F: FnMut(&mut T),
//         T: PartialEq,
//     {
//         match target {
//             Entity::Column(col) => {
//                 let value = tree_search(&self.tree, (0, col), (usize::MAX, col));
//                 if let Some(value) = value {
//                     // SAFETY: Must be safe - it's just that rs_tree does not provide mut operations
//                     // NOTE: We could create a copy of the last object... but it would be a bit expensive
//                     #[allow(mutable_transmutes)]
//                     let value = unsafe { std::mem::transmute::<&T, &mut T>(value) };

//                     (f)(value);
//                 }
//             }
//             Entity::Row(row) => {
//                 let value = tree_search(&self.tree, (row, 0), (row, usize::MAX));
//                 if let Some(value) = value {
//                     // SAFETY: Must be safe - it's just that rs_tree does not provide mut operations
//                     // NOTE: We could create a copy of the last object... but it would be a bit expensive
//                     #[allow(mutable_transmutes)]
//                     let value = unsafe { std::mem::transmute::<&T, &mut T>(value) };

//                     (f)(value);
//                 }
//             }
//             Entity::Cell(row, col) => {
//                 let value = tree_search(&self.tree, (row, col), (row, col));
//                 if let Some(value) = value {
//                     // SAFETY: Must be safe - it's just that rs_tree does not provide mut operations
//                     // NOTE: We could create a copy of the last object... but it would be a bit expensive
//                     #[allow(mutable_transmutes)]
//                     let value = unsafe { std::mem::transmute::<&T, &mut T>(value) };

//                     (f)(value);
//                 }
//             }
//             Entity::Global => {
//                 (f)(&mut self.global);
//             }
//         }
//     }

//     /// Get a value for an [`Entity`].
//     pub fn has(&self, target: Entity) -> bool
//     where
//         T: PartialEq,
//     {
//         match target {
//             Entity::Column(col) => tree_contains(&self.tree, (0, col), (usize::MAX, col)),
//             Entity::Row(row) => tree_contains(&self.tree, (row, 0), (row, usize::MAX)),
//             Entity::Cell(row, col) => tree_contains(&self.tree, (row, col), (row, col)),
//             Entity::Global => true,
//         }
//     }
// }

// impl<T> From<EntityMap<T>> for HashMap<Entity, T>
// where
//     T: PartialEq + Clone,
// {
//     fn from(em: EntityMap<T>) -> Self {
//         let mut m = HashMap::new();
//         m.insert(Entity::Global, em.global);

//         for item in em.tree.iter() {
//             let value = item.data.clone();

//             let is_same_row = item.rect.min[0] == item.rect.max[0];
//             let is_same_column = item.rect.min[1] == item.rect.max[1];
//             let is_unbound_row = item.rect.min[0] == 0 && item.rect.max[0] == usize::MAX;
//             let is_unbound_column = item.rect.min[1] == 0 && item.rect.max[1] == usize::MAX;

//             let row = item.rect.min[0];
//             let col = item.rect.min[1];

//             if is_same_row && is_same_column {
//                 m.insert(Entity::Cell(row, col), value);
//             } else if is_same_column && is_unbound_row {
//                 m.insert(Entity::Column(col), value);
//             } else if is_same_row && is_unbound_column {
//                 m.insert(Entity::Row(row), value);
//             } else {
//                 unreachable!("must have never happen")
//             }
//         }

//         m
//     }
// }

// impl<T> AsRef<T> for EntityMap<T> {
//     fn as_ref(&self) -> &T {
//         &self.global
//     }
// }

// fn tree_search<T>(tree: &RTree<2, usize, T>, x: Position, y: Position) -> Option<&T>
// where
//     T: PartialEq,
// {
//     tree.search(Rect::new([x.0, x.1], [y.0, y.1]))
//         .last()
//         .map(|item| item.data)
// }

// fn tree_contains<T>(tree: &RTree<2, usize, T>, x: Position, y: Position) -> bool
// where
//     T: PartialEq,
// {
//     tree.search(Rect::new([x.0, x.1], [y.0, y.1]))
//         .next()
//         .is_some()
// }

// **********************

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
    pub fn get(&self, (row, col): Position) -> &T {
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

    /// Get a value for an [`Entity`].
    pub fn modify<F>(&mut self, target: Entity, mut f: F)
    where
        F: FnMut(&mut T),
        T: Clone,
    {
        match target {
            Entity::Column(col) => {
                for &row in self.rows.keys() {
                    let value = self.cells.entry((row, col)).or_insert(self.global.clone());
                    (f)(value);
                }

                let value = self.columns.entry(col).or_insert(self.global.clone());
                (f)(value);
            }
            Entity::Row(row) => {
                for &col in self.columns.keys() {
                    let value = self.cells.entry((row, col)).or_insert(self.global.clone());
                    (f)(value);
                }

                let value = self.rows.entry(row).or_insert(self.global.clone());
                (f)(value);
            }
            Entity::Cell(row, col) => {
                let value = self.cells.entry((row, col)).or_insert(self.global.clone());
                (f)(value);
            }
            Entity::Global => {
                (f)(&mut self.global);
            }
        }
    }

    /// Get a value for an [`Entity`].
    pub fn has(&self, target: Entity) -> bool {
        match target {
            Entity::Column(col) => self.columns.contains_key(&col),
            Entity::Row(row) => self.rows.contains_key(&row),
            Entity::Cell(row, col) => self.cells.contains_key(&(row, col)),
            Entity::Global => true,
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

    /// Set a value for an [`Entity`].
    pub fn insert(&mut self, entity: Entity, value: T)
    where
        T: Clone,
    {
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

impl<T> From<EntityMap<T>> for HashMap<Entity, T> {
    fn from(value: EntityMap<T>) -> Self {
        let mut m = HashMap::new();
        m.insert(Entity::Global, value.global);

        for ((row, col), value) in value.cells {
            m.insert(Entity::Cell(row, col), value);
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
