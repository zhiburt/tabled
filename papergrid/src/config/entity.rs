use super::Position;

/// Entity a structure which represent a set of cells.
///
/// For example such table:
///
/// ```text
/// ┌───┬───┐
/// │ 0 │ 1 │
/// ├───┼───┤
/// │ 1 │ 2 │
/// └───┴───┘
/// ```
///
/// - has 4 cells.
///   Which indexes are (0, 0), (0, 1), (1, 0), (1, 1).
///
/// - has 2 rows.
///   Which indexes are 0, 1.
///
/// - has 2 column.
///   Which indexes are 0, 1.
///
/// In [`Entity`] terms, all cells on the grid we call `Global`.
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
        EntityIterator::new(*self, count_rows, count_cols)
    }
}

impl From<Position> for Entity {
    fn from(pos: Position) -> Self {
        Self::Cell(pos.row(), pos.col())
    }
}

impl From<(usize, usize)> for Entity {
    fn from(pos: (usize, usize)) -> Self {
        Self::Cell(pos.0, pos.1)
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

impl EntityIterator {
    fn new(entity: Entity, count_rows: usize, count_cols: usize) -> Self {
        Self {
            entity,
            count_rows,
            count_cols,
            i: 0,
            j: 0,
        }
    }
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

                Some(Position::new(row, col))
            }
            Entity::Column(col) => {
                if self.i >= self.count_rows {
                    return None;
                }

                let i = self.i;
                self.i += 1;

                Some(Position::new(i, col))
            }
            Entity::Row(row) => {
                if self.j >= self.count_cols {
                    return None;
                }

                let j = self.j;
                self.j += 1;

                Some(Position::new(row, j))
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

                Some(Position::new(self.i, j))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_iter() {
        assert_eq!(
            Entity::Global.iter(10, 10).collect::<Vec<_>>(),
            vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
                Position::new(0, 4),
                Position::new(0, 5),
                Position::new(0, 6),
                Position::new(0, 7),
                Position::new(0, 8),
                Position::new(0, 9),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(1, 3),
                Position::new(1, 4),
                Position::new(1, 5),
                Position::new(1, 6),
                Position::new(1, 7),
                Position::new(1, 8),
                Position::new(1, 9),
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(2, 2),
                Position::new(2, 3),
                Position::new(2, 4),
                Position::new(2, 5),
                Position::new(2, 6),
                Position::new(2, 7),
                Position::new(2, 8),
                Position::new(2, 9),
                Position::new(3, 0),
                Position::new(3, 1),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 4),
                Position::new(3, 5),
                Position::new(3, 6),
                Position::new(3, 7),
                Position::new(3, 8),
                Position::new(3, 9),
                Position::new(4, 0),
                Position::new(4, 1),
                Position::new(4, 2),
                Position::new(4, 3),
                Position::new(4, 4),
                Position::new(4, 5),
                Position::new(4, 6),
                Position::new(4, 7),
                Position::new(4, 8),
                Position::new(4, 9),
                Position::new(5, 0),
                Position::new(5, 1),
                Position::new(5, 2),
                Position::new(5, 3),
                Position::new(5, 4),
                Position::new(5, 5),
                Position::new(5, 6),
                Position::new(5, 7),
                Position::new(5, 8),
                Position::new(5, 9),
                Position::new(6, 0),
                Position::new(6, 1),
                Position::new(6, 2),
                Position::new(6, 3),
                Position::new(6, 4),
                Position::new(6, 5),
                Position::new(6, 6),
                Position::new(6, 7),
                Position::new(6, 8),
                Position::new(6, 9),
                Position::new(7, 0),
                Position::new(7, 1),
                Position::new(7, 2),
                Position::new(7, 3),
                Position::new(7, 4),
                Position::new(7, 5),
                Position::new(7, 6),
                Position::new(7, 7),
                Position::new(7, 8),
                Position::new(7, 9),
                Position::new(8, 0),
                Position::new(8, 1),
                Position::new(8, 2),
                Position::new(8, 3),
                Position::new(8, 4),
                Position::new(8, 5),
                Position::new(8, 6),
                Position::new(8, 7),
                Position::new(8, 8),
                Position::new(8, 9),
                Position::new(9, 0),
                Position::new(9, 1),
                Position::new(9, 2),
                Position::new(9, 3),
                Position::new(9, 4),
                Position::new(9, 5),
                Position::new(9, 6),
                Position::new(9, 7),
                Position::new(9, 8),
                Position::new(9, 9)
            ]
        );
    }
}
