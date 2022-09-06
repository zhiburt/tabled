//! The module contains a [`Locator`] trait and implementations for it.

use std::{
    iter::Once,
    ops::{Range, RangeBounds},
};

use papergrid::{records::Records, Entity};

use crate::{
    object::{
        bounds_to_usize, Column, Columns, FirstColumn, FirstRow, LastColumn, LastRow, Object, Row,
        Rows,
    },
    Table,
};

/// Locator is an interface which searches for a particular thing in the [`Records`],
/// and returns coordinate of the foundings if any.
pub trait Locator {
    /// A coordinate of the finding.
    type Coordinate;
    /// An iterator of the coordinates.
    /// If it's empty it's consideret that nothing is found.
    type IntoIter: IntoIterator<Item = Self::Coordinate>;

    /// Search for the thing in [`Records`], returning a list of coordinates.
    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records;
}

impl<B> Locator for Columns<B>
where
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl Locator for Column {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once((*self).into())
    }
}

impl Locator for FirstColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once(0)
    }
}

impl Locator for LastColumn {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        if records.count_columns() > 0 {
            std::iter::once(records.count_columns() - 1)
        } else {
            std::iter::once(0)
        }
    }
}

impl<B> Locator for Rows<B>
where
    B: RangeBounds<usize>,
{
    type Coordinate = usize;
    type IntoIter = Range<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        let (from, to) = bounds_to_usize(
            self.get_range().start_bound(),
            self.get_range().end_bound(),
            records.count_columns(),
        );

        from..to
    }
}

impl Locator for Row {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once((*self).into())
    }
}

impl Locator for FirstRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, _: R) -> Self::IntoIter
    where
        R: Records,
    {
        std::iter::once(0)
    }
}

impl Locator for LastRow {
    type Coordinate = usize;
    type IntoIter = Once<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        if records.count_rows() > 0 {
            std::iter::once(records.count_rows() - 1)
        } else {
            std::iter::once(0)
        }
    }
}

/// The structure is an implementaion of [`Locator`] to search for a column by it's name.
/// A name is considerent be a value in a first row.
///
/// So even if in reality there's no header, first row will be consideret the one.
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

impl<S> Locator for ByColumnName<S>
where
    S: AsRef<str>,
{
    type Coordinate = usize;
    type IntoIter = Vec<usize>;

    fn locate<R>(&mut self, records: R) -> Self::IntoIter
    where
        R: Records,
    {
        // todo: can be optimized by creating Iterator
        (0..records.count_columns())
            .filter(|col| records.get_text((0, *col)) == self.0.as_ref())
            .collect::<Vec<_>>()
    }
}

impl<S> Object for ByColumnName<S>
where
    S: AsRef<str>,
{
    type Iter = std::vec::IntoIter<Entity>;

    fn cells<R>(&self, table: &Table<R>) -> Self::Iter
    where
        R: Records,
    {
        // todo: can be optimized by creating Iterator
        (0..table.count_columns())
            .filter(|col| table.get_records().get_text((0, *col)) == self.0.as_ref())
            .map(Entity::Column)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
