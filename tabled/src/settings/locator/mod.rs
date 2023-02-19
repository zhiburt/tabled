//! The module contains a [`Locator`] trait and implementations for it.

use std::{
    iter::{self, Once},
    ops::{Range, RangeBounds},
};

use crate::{
    grid::config::Entity,
    records::{ExactRecords, Records},
    settings::object::{
        util::bounds_to_usize, Column, Columns, FirstColumn, FirstRow, LastColumn, LastRow, Object,
        Row, Rows,
    },
};

/// Locator is an interface which searches for a particular thing in the [`Records`],
/// and returns coordinate of the foundings if any.
pub trait Locator<Records> {
    /// A coordinate of the finding.
    type Coordinate;
    /// An iterator of the coordinates.
    /// If it's empty it's considered that nothing is found.
    type IntoIter: IntoIterator<Item = Self::Coordinate>;

    /// Search for the thing in [`Records`], returning a list of coordinates.
    fn locate(&mut self, records: Records) -> Self::IntoIter;
}

impl<B, R> Locator<R> for Columns<B>
where
    B: RangeBounds<usize>,
    R: Records,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        let range = self.get_range();
        let max = records.count_columns();
        let (from, to) = bounds_to_usize(range.start_bound(), range.end_bound(), max);

        from..to
    }
}

impl<R> Locator<R> for Column {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once((*self).into())
    }
}

impl<R> Locator<R> for FirstColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once(0)
    }
}

impl<R> Locator<R> for LastColumn
where
    R: Records,
{
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        if records.count_columns() > 0 {
            iter::once(records.count_columns() - 1)
        } else {
            iter::once(0)
        }
    }
}

impl<B, R> Locator<R> for Rows<B>
where
    R: Records,
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl<R> Locator<R> for Row {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once((*self).into())
    }
}

impl<R> Locator<R> for FirstRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, _: R) -> Self::IntoIter {
        iter::once(0)
    }
}

impl<R> Locator<R> for LastRow
where
    R: ExactRecords,
{
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        if records.count_rows() > 0 {
            iter::once(records.count_rows() - 1)
        } else {
            iter::once(0)
        }
    }
}

/// The structure is an implementation of [`Locator`] to search for a column by it's name.
/// A name is considered be a value in a first row.
///
/// So even if in reality there's no header, the first row will be considered to be one.
#[derive(Debug, Clone, Copy)]
pub struct ByColumnName<S>(S);

impl<S> ByColumnName<S> {
    /// Constructs a new object of the structure.
    pub fn new(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(text)
    }
}

impl<R, S> Locator<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords,
{
    type Coordinate = usize;
    type IntoIter = Vec<usize>;

    fn locate(&mut self, records: R) -> Self::IntoIter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_cell((0, *col)).as_ref() == self.0.as_ref())
            .collect::<Vec<_>>()
    }
}

impl<S, R> Object<R> for ByColumnName<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells(&self, records: &R) -> Self::Iter {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_cell((0, *col)).as_ref() == self.0.as_ref())
            .map(Entity::Column)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
