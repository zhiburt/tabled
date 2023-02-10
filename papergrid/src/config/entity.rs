/// Entity a structure which represent a set of cells.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Entity {
    /// All cells on the grid.
    Global,
    /// All cells in a column on the grid.
    Column(usize),
    /// All cells in a row on the grid.
    Row(usize),
    /// A particular cell (row, column) on the grid.
    Cell(usize, usize),
}

impl Entity {
    /// Iterate over cells which are covered via the [`Entity`].
    pub fn iter(&self, count_rows: usize, count_cols: usize) -> EntityIterator {
        EntityIterator {
            entity: *self,
            count_rows,
            count_cols,
            i: 0,
            j: 0,
        }
    }
}

/// Position is a (row, col) position on a Grid.
pub type Position = (usize, usize);

impl From<Position> for Entity {
    fn from((row, col): Position) -> Self {
        Self::Cell(row, col)
    }
}

/// An iterator over cells.
///
/// Produced from [`Entity::iter`].
#[derive(Debug, Clone)]
pub struct EntityIterator {
    entity: Entity,
    count_rows: usize,
    count_cols: usize,
    i: usize,
    j: usize,
}

impl Iterator for EntityIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count_rows == 0 || self.count_cols == 0 {
            return None;
        }

        match self.entity {
            Entity::Cell(row, col) => {
                self.count_cols = 0;
                self.count_rows = 0;

                Some((row, col))
            }
            Entity::Column(col) => {
                if self.i >= self.count_rows {
                    return None;
                }

                let i = self.i;
                self.i += 1;

                Some((i, col))
            }
            Entity::Row(row) => {
                if self.j >= self.count_cols {
                    return None;
                }

                let j = self.j;
                self.j += 1;

                Some((row, j))
            }
            Entity::Global => {
                if self.j >= self.count_cols {
                    self.j = 0;
                    self.i += 1;

                    if self.i >= self.count_rows {
                        return None;
                    }
                }

                let j = self.j;
                self.j += 1;

                Some((self.i, j))
            }
        }
    }
}
