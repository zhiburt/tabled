use std::collections::HashMap;

use rtree_rs::{RTree, Rect};

use crate::config::{Entity, Position};

/// A structure to keep information for [`Entity`] as a key.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EntityMap<T> {
    // we have a global type to allocate in on stack.
    // because most of the time no changes are made to the [`EntityMap`].
    global: T,
    // todo: Maybe maybe we shall check whether tree is getting too big and switch back to a 3 hash tables?
    tree: RTree<2, usize, T>,
}

impl<T> EntityMap<T> {
    /// Creates an empty [`EntityMap`].
    pub fn new(global: T) -> Self
    where
        T: PartialEq,
    {
        Self {
            global,
            tree: RTree::new(),
        }
    }

    /// Verifies whether anything was set beside a global entry.
    pub fn is_empty(&self) -> bool
    where
        T: PartialEq,
    {
        self.tree.is_empty()
    }

    /// Get a value for an [`Entity`].
    pub fn get(&self, pos: Position) -> &T
    where
        T: PartialEq,
    {
        if self.is_empty() {
            return &self.global;
        }

        let (row, col) = pos;
        let value = self
            .tree
            .search(Rect::new([row, col], [row, col]))
            .last()
            .map(|item| item.data);

        value.unwrap_or(&self.global)
    }

    /// Removes a value for an [`Entity`].
    pub fn remove(&mut self, entity: Entity)
    where
        T: PartialEq + Clone,
    {
        let mut new = RTree::new();

        // todo: Rtree contribution to remove effectively

        match entity {
            Entity::Global => {}
            Entity::Column(col) => {
                for item in self.tree.iter() {
                    if item.rect.min[1] != col && item.rect.max[1] != col {
                        new.insert(item.rect, item.data.clone());
                    }
                }
            }
            Entity::Row(row) => {
                for item in self.tree.iter() {
                    if item.rect.min[0] != row && item.rect.max[0] != row {
                        new.insert(item.rect, item.data.clone());
                    }
                }
            }
            Entity::Cell(row, col) => {
                for item in self.tree.iter() {
                    if item.rect.min != [row, col] && item.rect.max != [row, col] {
                        new.insert(item.rect, item.data.clone());
                    }
                }
            }
        }

        self.tree = new;
    }

    /// Set a value for an [`Entity`].
    pub fn insert(&mut self, entity: Entity, value: T)
    where
        T: PartialEq,
    {
        match entity {
            Entity::Column(col) => {
                self.tree
                    .insert(Rect::new([0, col], [usize::MAX, col]), value);
            }
            Entity::Row(row) => {
                self.tree
                    .insert(Rect::new([row, 0], [row, usize::MAX]), value);
            }
            Entity::Cell(row, col) => {
                self.tree.insert(Rect::new([row, col], [row, col]), value);
            }
            Entity::Global => {
                self.tree = RTree::new();
                self.global = value
            }
        }
    }
}

impl<T> From<EntityMap<T>> for HashMap<Entity, T>
where
    T: PartialEq + Clone,
{
    fn from(em: EntityMap<T>) -> Self {
        let mut m = HashMap::new();
        m.insert(Entity::Global, em.global);

        for item in em.tree.iter() {
            let value = item.data.clone();

            let is_same_row = item.rect.min[0] == item.rect.max[0];
            let is_same_column = item.rect.min[1] == item.rect.max[1];
            let is_unbound_row = item.rect.min[0] == 0 && item.rect.max[0] == usize::MAX;
            let is_unbound_column = item.rect.min[1] == 0 && item.rect.max[1] == usize::MAX;

            let row = item.rect.min[0];
            let col = item.rect.min[1];

            if is_same_row && is_same_column {
                m.insert(Entity::Cell(row, col), value);
            } else if is_same_column && is_unbound_row {
                m.insert(Entity::Column(col), value);
            } else if is_same_row && is_unbound_column {
                m.insert(Entity::Row(row), value);
            } else {
                unreachable!("must have never happen")
            }
        }

        m
    }
}

impl<T> AsRef<T> for EntityMap<T> {
    fn as_ref(&self) -> &T {
        &self.global
    }
}