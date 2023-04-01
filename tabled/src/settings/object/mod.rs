//! This module contains a list of primitives that implement a [`Object`] trait.
//! They help to locate a necessary segment on a [`Table`].
//!
//! [`Table`]: crate::Table

mod cell;
mod columns;
mod frame;
mod rows;
mod segment;
pub(crate) mod util;

use std::{collections::HashSet, marker::PhantomData};

use self::segment::SectorCellsIter;

use crate::{
    grid::config::{Entity, EntityIterator},
    grid::records::{ExactRecords, Records},
};

pub use cell::{Cell, EntityOnce};
pub use columns::{Column, Columns, ColumnsIter, FirstColumn, LastColumn, LastColumnOffset};
pub use frame::{Frame, FrameIter};
pub use rows::{FirstRow, LastRow, LastRowOffset, Row, Rows, RowsIter};
pub use segment::{SectorIter, Segment, SegmentAll};

/// Object helps to locate a necessary part of a [`Table`].
///
/// [`Table`]: crate::Table
pub trait Object<R> {
    /// An [`Iterator`] which returns a list of cells.
    type Iter: Iterator<Item = Entity>;

    /// Cells returns a set of coordinates of cells.
    fn cells(&self, records: &R) -> Self::Iter;

    /// Combines cells.
    /// It doesn't repeat cells.
    fn and<O>(self, rhs: O) -> UnionCombination<Self, O, R>
    where
        Self: Sized,
    {
        UnionCombination::new(self, rhs)
    }

    /// Excludes rhs cells from this cells.
    fn not<O>(self, rhs: O) -> DiffCombination<Self, O, R>
    where
        Self: Sized,
    {
        DiffCombination::new(self, rhs)
    }

    /// Returns cells which are present in both [`Object`]s only.
    fn intersect<O>(self, rhs: O) -> IntersectionCombination<Self, O, R>
    where
        Self: Sized,
    {
        IntersectionCombination::new(self, rhs)
    }

    /// Returns cells which are not present in target [`Object`].
    fn inverse(self) -> InversionCombination<Self, R>
    where
        Self: Sized,
    {
        InversionCombination::new(self)
    }
}

/// Combination struct used for chaining [`Object`]'s.
///
/// Combines 2 sets of cells into one.
///
/// Duplicates are removed from the output set.
#[derive(Debug)]
pub struct UnionCombination<L, R, I> {
    lhs: L,
    rhs: R,
    _records: PhantomData<I>,
}

impl<L, R, I> UnionCombination<L, R, I> {
    fn new(lhs: L, rhs: R) -> Self {
        Self {
            lhs,
            rhs,
            _records: PhantomData,
        }
    }
}

impl<I, L, R> Object<I> for UnionCombination<L, R, I>
where
    L: Object<I>,
    R: Object<I>,
    I: Records + ExactRecords,
{
    type Iter = UnionIter<L::Iter, R::Iter>;

    fn cells(&self, records: &I) -> Self::Iter {
        let lhs = self.lhs.cells(records);
        let rhs = self.rhs.cells(records);

        UnionIter::new(lhs, rhs, records.count_rows(), records.count_columns())
    }
}

/// Difference struct used for chaining [`Object`]'s.
///
/// Returns cells from 1st set with removed ones from the 2nd set.
#[derive(Debug)]
pub struct DiffCombination<L, R, I> {
    lhs: L,
    rhs: R,
    _records: PhantomData<I>,
}

impl<L, R, I> DiffCombination<L, R, I> {
    fn new(lhs: L, rhs: R) -> Self {
        Self {
            lhs,
            rhs,
            _records: PhantomData,
        }
    }
}

impl<I, L, R> Object<I> for DiffCombination<L, R, I>
where
    L: Object<I>,
    R: Object<I>,
    I: Records + ExactRecords,
{
    type Iter = DiffIter<L::Iter>;

    fn cells(&self, records: &I) -> Self::Iter {
        let lhs = self.lhs.cells(records);
        let rhs = self.rhs.cells(records);

        DiffIter::new(lhs, rhs, records.count_rows(), records.count_columns())
    }
}

/// Intersection struct used for chaining [`Object`]'s.
///
/// Returns cells which are present in 2 sets.
/// But not in one of them
#[derive(Debug)]
pub struct IntersectionCombination<L, R, I> {
    lhs: L,
    rhs: R,
    _records: PhantomData<I>,
}

impl<L, R, I> IntersectionCombination<L, R, I> {
    fn new(lhs: L, rhs: R) -> Self {
        Self {
            lhs,
            rhs,
            _records: PhantomData,
        }
    }
}

impl<I, L, R> Object<I> for IntersectionCombination<L, R, I>
where
    L: Object<I>,
    R: Object<I>,
    I: Records + ExactRecords,
{
    type Iter = IntersectIter<L::Iter>;

    fn cells(&self, records: &I) -> Self::Iter {
        let lhs = self.lhs.cells(records);
        let rhs = self.rhs.cells(records);

        IntersectIter::new(lhs, rhs, records.count_rows(), records.count_columns())
    }
}

/// Inversion struct used for chaining [`Object`]'s.
///
/// Returns cells which are present in 2 sets.
/// But not in one of them
#[derive(Debug)]
pub struct InversionCombination<O, I> {
    obj: O,
    _records: PhantomData<I>,
}

impl<O, I> InversionCombination<O, I> {
    fn new(obj: O) -> Self {
        Self {
            obj,
            _records: PhantomData,
        }
    }
}

impl<I, O> Object<I> for InversionCombination<O, I>
where
    O: Object<I>,
    I: Records + ExactRecords,
{
    type Iter = InversionIter;

    fn cells(&self, records: &I) -> Self::Iter {
        let obj = self.obj.cells(records);

        InversionIter::new(obj, records.count_rows(), records.count_columns())
    }
}

/// An [`Iterator`] which goes over a combination [`Object::Iter`].
#[derive(Debug)]
pub struct UnionIter<L, R> {
    lhs: Option<L>,
    rhs: R,
    seen: HashSet<(usize, usize)>,
    current: Option<EntityIterator>,
    count_rows: usize,
    count_cols: usize,
}

impl<L, R> UnionIter<L, R>
where
    L: Iterator<Item = Entity>,
    R: Iterator<Item = Entity>,
{
    fn new(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self {
        let size = match lhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        Self {
            lhs: Some(lhs),
            rhs,
            seen: HashSet::with_capacity(size),
            current: None,
            count_rows,
            count_cols,
        }
    }
}

impl<L, R> Iterator for UnionIter<L, R>
where
    L: Iterator<Item = Entity>,
    R: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if self.lhs.is_none() && self.seen.contains(&p) {
                    continue;
                }

                let _ = self.seen.insert(p);
                return Some(Entity::Cell(p.0, p.1));
            }
        }

        if let Some(lhs) = self.lhs.as_mut() {
            for entity in lhs.by_ref() {
                let mut iter = entity.iter(self.count_rows, self.count_cols);
                if let Some(p) = iter.by_ref().next() {
                    let _ = self.seen.insert(p);
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }

            self.lhs = None;
        }

        for entity in self.rhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    let _ = self.seen.insert(p);
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes over only cells which are present in first [`Object::Iter`] but not second.
#[derive(Debug)]
pub struct DiffIter<L> {
    lhs: L,
    seen: HashSet<(usize, usize)>,
    count_rows: usize,
    count_cols: usize,
    current: Option<EntityIterator>,
}

impl<L> DiffIter<L>
where
    L: Iterator<Item = Entity>,
{
    fn new<R>(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self
    where
        R: Iterator<Item = Entity>,
    {
        let size = match rhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in rhs {
            seen.extend(entity.iter(count_rows, count_cols));
        }

        Self {
            lhs,
            seen,
            count_rows,
            count_cols,
            current: None,
        }
    }
}

impl<L> Iterator for DiffIter<L>
where
    L: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        for entity in self.lhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if !self.seen.contains(&p) {
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes goes over cells which are present in both [`Object::Iter`]ators.
#[derive(Debug)]
pub struct IntersectIter<L> {
    lhs: L,
    seen: HashSet<(usize, usize)>,
    count_rows: usize,
    count_cols: usize,
    current: Option<EntityIterator>,
}

impl<L> IntersectIter<L>
where
    L: Iterator<Item = Entity>,
{
    fn new<R>(lhs: L, rhs: R, count_rows: usize, count_cols: usize) -> Self
    where
        R: Iterator<Item = Entity>,
    {
        let size = match rhs.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in rhs {
            seen.extend(entity.iter(count_rows, count_cols));
        }

        Self {
            lhs,
            seen,
            count_rows,
            count_cols,
            current: None,
        }
    }
}

impl<L> Iterator for IntersectIter<L>
where
    L: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.current.as_mut() {
            for p in iter.by_ref() {
                if self.seen.contains(&p) {
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        for entity in self.lhs.by_ref() {
            let mut iter = entity.iter(self.count_rows, self.count_cols);

            for p in iter.by_ref() {
                if self.seen.contains(&p) {
                    self.current = Some(iter);
                    return Some(Entity::Cell(p.0, p.1));
                }
            }
        }

        None
    }
}

/// An [`Iterator`] which goes goes over cells which are not present an [`Object::Iter`]ator.
#[derive(Debug)]
pub struct InversionIter {
    all: SectorCellsIter,
    seen: HashSet<(usize, usize)>,
}

impl InversionIter {
    fn new<O>(obj: O, count_rows: usize, count_columns: usize) -> Self
    where
        O: Iterator<Item = Entity>,
    {
        let size = match obj.size_hint() {
            (s1, Some(s2)) if s1 == s2 => s1,
            _ => 0,
        };

        let mut seen = HashSet::with_capacity(size);
        for entity in obj {
            seen.extend(entity.iter(count_rows, count_columns));
        }

        let all = SectorCellsIter::new(0, count_rows, 0, count_columns);

        Self { all, seen }
    }
}

impl Iterator for InversionIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.all.by_ref() {
            if !self.seen.contains(&p) {
                return Some(Entity::Cell(p.0, p.1));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::records::vec_records::VecRecords;

    use super::*;

    #[test]
    fn cell_test() {
        assert_eq!(vec_cells((0, 0), 2, 3), [Entity::Cell(0, 0)]);
        assert_eq!(vec_cells((1, 1), 2, 3), [Entity::Cell(1, 1)]);
        assert_eq!(vec_cells((1, 1), 0, 0), [Entity::Cell(1, 1)]);
        assert_eq!(vec_cells((1, 100), 2, 3), [Entity::Cell(1, 100)]);
        assert_eq!(vec_cells((100, 1), 2, 3), [Entity::Cell(100, 1)]);
    }

    #[test]
    fn columns_test() {
        assert_eq!(
            vec_cells(Columns::new(..), 2, 3),
            [Entity::Column(0), Entity::Column(1), Entity::Column(2)]
        );
        assert_eq!(
            vec_cells(Columns::new(1..), 2, 3),
            [Entity::Column(1), Entity::Column(2)]
        );
        assert_eq!(vec_cells(Columns::new(2..), 2, 3), [Entity::Column(2)]);
        assert_eq!(vec_cells(Columns::new(3..), 2, 3), []);
        assert_eq!(vec_cells(Columns::new(3..), 0, 0), []);
        assert_eq!(vec_cells(Columns::new(0..1), 2, 3), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::new(1..2), 2, 3), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::new(2..3), 2, 3), [Entity::Column(2)]);
        assert_eq!(vec_cells(Columns::new(..), 0, 0), []);
        assert_eq!(vec_cells(Columns::new(..), 2, 0), []);
        assert_eq!(vec_cells(Columns::new(..), 0, 3), []);
    }

    #[test]
    fn first_column_test() {
        assert_eq!(vec_cells(Columns::first(), 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first(), 0, 0), []);
        assert_eq!(vec_cells(Columns::first(), 10, 0), []);
        assert_eq!(vec_cells(Columns::first(), 0, 10), []);
    }

    #[test]
    fn last_column_test() {
        assert_eq!(vec_cells(Columns::last(), 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last(), 5, 29), [Entity::Column(28)]);
        assert_eq!(vec_cells(Columns::last(), 0, 0), []);
        assert_eq!(vec_cells(Columns::last(), 10, 0), []);
        assert_eq!(vec_cells(Columns::last(), 0, 10), []);
    }

    #[test]
    fn last_column_sub_test() {
        assert_eq!(vec_cells(Columns::last(), 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last() - 0, 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::last() - 1, 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::last() - 2, 5, 2), []);
        assert_eq!(vec_cells(Columns::last() - 100, 5, 2), []);
    }

    #[test]
    fn first_column_add_test() {
        assert_eq!(vec_cells(Columns::first(), 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first() + 0, 5, 2), [Entity::Column(0)]);
        assert_eq!(vec_cells(Columns::first() + 1, 5, 2), [Entity::Column(1)]);
        assert_eq!(vec_cells(Columns::first() + 2, 5, 2), [Entity::Column(2)]);
        assert_eq!(
            vec_cells(Columns::first() + 100, 5, 2),
            [Entity::Column(100)]
        );
    }

    #[test]
    fn rows_test() {
        assert_eq!(
            vec_cells(Rows::new(..), 2, 3),
            [Entity::Row(0), Entity::Row(1)]
        );
        assert_eq!(vec_cells(Rows::new(1..), 2, 3), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::new(2..), 2, 3), []);
        assert_eq!(vec_cells(Rows::new(2..), 0, 0), []);
        assert_eq!(vec_cells(Rows::new(0..1), 2, 3), [Entity::Row(0)],);
        assert_eq!(vec_cells(Rows::new(1..2), 2, 3), [Entity::Row(1)],);
        assert_eq!(vec_cells(Rows::new(..), 0, 0), []);
        assert_eq!(vec_cells(Rows::new(..), 0, 3), []);
        assert_eq!(
            vec_cells(Rows::new(..), 2, 0),
            [Entity::Row(0), Entity::Row(1)]
        );
    }

    #[test]
    fn last_row_test() {
        assert_eq!(vec_cells(Rows::last(), 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last(), 100, 2), [Entity::Row(99)]);
        assert_eq!(vec_cells(Rows::last(), 0, 0), []);
        assert_eq!(vec_cells(Rows::last(), 5, 0), []);
        assert_eq!(vec_cells(Rows::last(), 0, 2), []);
    }

    #[test]
    fn first_row_test() {
        assert_eq!(vec_cells(Rows::first(), 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first(), 100, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first(), 0, 0), []);
        assert_eq!(vec_cells(Rows::first(), 5, 0), []);
        assert_eq!(vec_cells(Rows::first(), 0, 2), []);
    }

    #[test]
    fn last_row_sub_test() {
        assert_eq!(vec_cells(Rows::last(), 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last() - 0, 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::last() - 1, 5, 2), [Entity::Row(3)]);
        assert_eq!(vec_cells(Rows::last() - 2, 5, 2), [Entity::Row(2)]);
        assert_eq!(vec_cells(Rows::last() - 3, 5, 2), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::last() - 4, 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::last() - 5, 5, 2), []);
        assert_eq!(vec_cells(Rows::last() - 100, 5, 2), []);
        assert_eq!(vec_cells(Rows::last() - 1, 0, 0), []);
        assert_eq!(vec_cells(Rows::last() - 1, 5, 0), []);
        assert_eq!(vec_cells(Rows::last() - 1, 0, 2), []);
    }

    #[test]
    fn first_row_add_test() {
        assert_eq!(vec_cells(Rows::first(), 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first() + 0, 5, 2), [Entity::Row(0)]);
        assert_eq!(vec_cells(Rows::first() + 1, 5, 2), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::first() + 2, 5, 2), [Entity::Row(2)]);
        assert_eq!(vec_cells(Rows::first() + 3, 5, 2), [Entity::Row(3)]);
        assert_eq!(vec_cells(Rows::first() + 4, 5, 2), [Entity::Row(4)]);
        assert_eq!(vec_cells(Rows::first() + 5, 5, 2), [Entity::Row(5)]);
        assert_eq!(vec_cells(Rows::first() + 100, 5, 2), [Entity::Row(100)]);
        assert_eq!(vec_cells(Rows::first() + 1, 0, 0), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::first() + 1, 5, 0), [Entity::Row(1)]);
        assert_eq!(vec_cells(Rows::first() + 1, 0, 2), [Entity::Row(1)]);
    }

    #[test]
    fn frame_test() {
        assert_eq!(
            vec_cells(Frame, 2, 3),
            [
                Entity::Cell(0, 0),
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(vec_cells(Frame, 0, 0), []);
        assert_eq!(vec_cells(Frame, 2, 0), []);
        assert_eq!(vec_cells(Frame, 0, 2), []);
    }

    #[test]
    fn segment_test() {
        assert_eq!(
            vec_cells(Segment::new(.., ..), 2, 3),
            [
                Entity::Cell(0, 0),
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Segment::new(1.., ..), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Segment::new(2.., ..), 2, 3), []);

        assert_eq!(
            vec_cells(Segment::new(.., 1..), 2, 3),
            [
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Segment::new(.., 2..), 2, 3),
            [Entity::Cell(0, 2), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Segment::new(.., 3..), 2, 3), []);

        assert_eq!(
            vec_cells(Segment::new(1.., 1..), 2, 3),
            [Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(
            vec_cells(Segment::new(1..2, 1..2), 2, 3),
            [Entity::Cell(1, 1)]
        );

        assert_eq!(vec_cells(Segment::new(5.., 5..), 2, 3), []);
    }

    #[test]
    fn object_and_test() {
        assert_eq!(
            vec_cells(Cell::new(0, 0).and(Cell::new(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        assert_eq!(
            vec_cells(Cell::new(0, 0).and(Cell::new(1, 2)), 2, 3),
            [Entity::Cell(0, 0), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Cell::new(0, 0).and(Cell::new(1, 2)), 0, 0), []);
    }

    #[test]
    fn object_not_test() {
        assert_eq!(vec_cells(Rows::first().not(Cell::new(0, 0)), 0, 0), []);
        assert_eq!(vec_cells(Cell::new(0, 0).not(Cell::new(0, 0)), 2, 3), []);
        assert_eq!(
            vec_cells(Rows::first().not(Cell::new(0, 0)), 2, 3),
            [Entity::Cell(0, 1), Entity::Cell(0, 2)]
        );
        assert_eq!(
            vec_cells(Columns::single(1).not(Rows::single(1)), 3, 3),
            [Entity::Cell(0, 1), Entity::Cell(2, 1)]
        );
        assert_eq!(
            vec_cells(Rows::single(1).not(Columns::single(1)), 3, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 2)]
        );
    }

    #[test]
    fn object_intersect_test() {
        assert_eq!(
            vec_cells(Rows::first().intersect(Cell::new(0, 0)), 0, 0),
            []
        );
        assert_eq!(
            vec_cells(Segment::all().intersect(Rows::single(1)), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(
            vec_cells(Cell::new(0, 0).intersect(Cell::new(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        assert_eq!(
            vec_cells(Rows::first().intersect(Cell::new(0, 0)), 2, 3),
            [Entity::Cell(0, 0)]
        );
        // maybe we somehow shall not limit the rows/columns by the max count?
        assert_eq!(
            vec_cells(Rows::single(1).intersect(Columns::single(1)), 2, 1),
            []
        );
    }

    #[test]
    fn object_inverse_test() {
        assert_eq!(vec_cells(Segment::all().inverse(), 2, 3), []);
        assert_eq!(
            vec_cells(Cell::new(0, 0).inverse(), 2, 3),
            [
                Entity::Cell(0, 1),
                Entity::Cell(0, 2),
                Entity::Cell(1, 0),
                Entity::Cell(1, 1),
                Entity::Cell(1, 2)
            ]
        );
        assert_eq!(
            vec_cells(Rows::first().inverse(), 2, 3),
            [Entity::Cell(1, 0), Entity::Cell(1, 1), Entity::Cell(1, 2)]
        );
        assert_eq!(vec_cells(Rows::first().inverse(), 0, 0), []);
    }

    fn vec_cells<O: Object<VecRecords<String>>>(
        o: O,
        count_rows: usize,
        count_cols: usize,
    ) -> Vec<Entity> {
        let data = vec![vec![String::default(); count_cols]; count_rows];
        let records = VecRecords::new(data);
        o.cells(&records).collect::<Vec<_>>()
    }
}
