#[allow(unused)]
use papergrid::Grid;
use std::{
    collections::BTreeSet,
    ops::{Bound, RangeBounds},
};

/// Object helps to locate a nessesary part of a [Grid].
pub trait Object: Sized {
    /// Cells returns a set of cordinates of cells
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)>;

    /// Combines cells.
    /// It doesn't repeat cells.
    fn and<O: Object>(self, rhs: O) -> Combination<Self, O> {
        Combination {
            lhs: self,
            rhs,
            combinator: combine_cells,
        }
    }

    /// Excludes rhs cells from this cells.
    fn not<O: Object>(self, rhs: O) -> Combination<Self, O> {
        Combination {
            lhs: self,
            rhs,
            combinator: remove_cells,
        }
    }
}

/// Head represents the row at the top of a [Table].
pub struct Head;

impl Object for Head {
    fn cells(&self, _: usize, count_columns: usize) -> Vec<(usize, usize)> {
        (0..count_columns).map(|column| (0, column)).collect()
    }
}

/// Full represents all cells on a [Grid]
pub struct Full;

impl Object for Full {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        (0..count_rows)
            .map(|row| {
                (0..count_columns)
                    .map(|column| (row, column))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

/// Row denotes a set of cells on given rows on a [Grid].
pub struct Row<R: RangeBounds<usize>>(pub R);

impl<R: RangeBounds<usize>> Object for Row<R> {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(self.0.start_bound(), self.0.end_bound(), count_rows);

        (x..y)
            .map(|row| (0..count_columns).map(|column| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

/// Column denotes a set of cells on given columns on a [Grid].
pub struct Column<R: RangeBounds<usize>>(pub R);

impl<R: RangeBounds<usize>> Object for Column<R> {
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let (x, y) = bounds_to_usize(self.0.start_bound(), self.0.end_bound(), count_columns);

        (x..y)
            .map(|column| (0..count_rows).map(|row| (row, column)).collect())
            .collect::<Vec<Vec<_>>>()
            .concat()
    }
}

/// Cell denotes a particular cell on a [Grid].
pub struct Cell(pub usize, pub usize);

impl Object for Cell {
    fn cells(&self, _: usize, _: usize) -> Vec<(usize, usize)> {
        vec![(self.0, self.1)]
    }
}

/// Combinator is a transformation function
type Combinator = fn(Vec<(usize, usize)>, Vec<(usize, usize)>) -> Vec<(usize, usize)>;

/// Combination struct used for chaning [Object]'s.
pub struct Combination<L, R> {
    lhs: L,
    rhs: R,
    combinator: Combinator,
}

impl<L, R> Object for Combination<L, R>
where
    L: Object,
    R: Object,
{
    fn cells(&self, count_rows: usize, count_columns: usize) -> Vec<(usize, usize)> {
        let l = self.lhs.cells(count_rows, count_columns);
        let r = self.rhs.cells(count_rows, count_columns);
        (self.combinator)(l, r)
    }
}

/// Combines 2 sets of cells into one.
///
/// Dublicates are removed from the output set.
fn combine_cells(lhs: Vec<(usize, usize)>, rhs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    lhs.into_iter()
        .chain(rhs.into_iter())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Removes cells from fist set which are present in a second set.
fn remove_cells(lhs: Vec<(usize, usize)>, rhs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    lhs.into_iter().filter(|l| !rhs.contains(l)).collect()
}

/// Converts a range bound to its indexes.
pub(crate) fn bounds_to_usize(
    left: Bound<&usize>,
    right: Bound<&usize>,
    count_elements: usize,
) -> (usize, usize) {
    match (left, right) {
        (Bound::Included(x), Bound::Included(y)) => (*x, y + 1),
        (Bound::Included(x), Bound::Excluded(y)) => (*x, *y),
        (Bound::Included(x), Bound::Unbounded) => (*x, count_elements),
        (Bound::Unbounded, Bound::Unbounded) => (0, count_elements),
        (Bound::Unbounded, Bound::Included(y)) => (0, y + 1),
        (Bound::Unbounded, Bound::Excluded(y)) => (0, *y),
        (Bound::Excluded(_), Bound::Unbounded)
        | (Bound::Excluded(_), Bound::Included(_))
        | (Bound::Excluded(_), Bound::Excluded(_)) => {
            unreachable!("A start bound can't be excluded")
        }
    }
}
