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
        EntityIterator {
            entity: *self,
            count_rows,
            count_cols,
            i: 0,
            j: 0,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_iter() {
        assert_eq!(
            Entity::Global.iter(10, 10).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
                (0, 9),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
                (2, 8),
                (2, 9),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (3, 8),
                (3, 9),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
                (4, 7),
                (4, 8),
                (4, 9),
                (5, 0),
                (5, 1),
                (5, 2),
                (5, 3),
                (5, 4),
                (5, 5),
                (5, 6),
                (5, 7),
                (5, 8),
                (5, 9),
                (6, 0),
                (6, 1),
                (6, 2),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
                (6, 7),
                (6, 8),
                (6, 9),
                (7, 0),
                (7, 1),
                (7, 2),
                (7, 3),
                (7, 4),
                (7, 5),
                (7, 6),
                (7, 7),
                (7, 8),
                (7, 9),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (8, 6),
                (8, 7),
                (8, 8),
                (8, 9),
                (9, 0),
                (9, 1),
                (9, 2),
                (9, 3),
                (9, 4),
                (9, 5),
                (9, 6),
                (9, 7),
                (9, 8),
                (9, 9)
            ]
        );
    }
}
