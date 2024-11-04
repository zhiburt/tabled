//! This module contains an [`ObjectIterator`].

use core::marker::PhantomData;

use crate::grid::config::Entity;
use crate::settings::object::Object;

/// A utility trait helps to modify an [`Object`],
/// by various functions.
pub trait ObjectIterator<R>: Object<R> {
    /// Skip N entities.
    fn skip(self, n: usize) -> SkipObject<Self, R>
    where
        Self: Sized,
    {
        SkipObject::new(self, n)
    }

    /// Make a step for an iteration of entities.
    fn step_by(self, n: usize) -> StepByObject<Self, R>
    where
        Self: Sized,
    {
        StepByObject::new(self, n)
    }

    /// Use a filter while iterating over entities.
    fn filter<F>(self, predicate: F) -> FilterObject<Self, F, R>
    where
        Self: Sized,
        F: Fn(Entity) -> bool,
    {
        FilterObject::new(self, predicate)
    }
}

impl<T, R> ObjectIterator<R> for T where T: Object<R> {}

/// Skip object for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SkipObject<O, R> {
    obj: O,
    n: usize,
    _records: PhantomData<R>,
}

impl<O, R> SkipObject<O, R> {
    fn new(obj: O, n: usize) -> Self {
        Self {
            obj,
            n,
            _records: PhantomData,
        }
    }
}

impl<O, R> Object<R> for SkipObject<O, R>
where
    O: Object<R>,
{
    type Iter = SkipObjectIter<O::Iter>;

    fn cells(&self, records: &R) -> Self::Iter {
        SkipObjectIter::new(self.obj.cells(records), self.n)
    }
}

/// Skip object iterator for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SkipObjectIter<I> {
    iter: I,
    n: usize,
}

impl<I> SkipObjectIter<I> {
    fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> Iterator for SkipObjectIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n > 0 {
            self.n -= 1;
            let _ = self.iter.next()?;
        }

        self.iter.next()
    }
}

/// Step object for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct StepByObject<O, R> {
    obj: O,
    n: usize,
    _records: PhantomData<R>,
}

impl<O, R> StepByObject<O, R> {
    fn new(obj: O, n: usize) -> Self {
        Self {
            obj,
            n,
            _records: PhantomData,
        }
    }
}

impl<O, R> Object<R> for StepByObject<O, R>
where
    O: Object<R>,
{
    type Iter = StepByObjectIter<O::Iter>;

    fn cells(&self, records: &R) -> Self::Iter {
        StepByObjectIter::new(self.obj.cells(records), self.n)
    }
}

/// Step object iterator for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct StepByObjectIter<I> {
    iter: I,
    step: usize,
    end: bool,
}

impl<I> StepByObjectIter<I> {
    fn new(iter: I, step: usize) -> Self {
        let end = step == 0;

        Self { iter, step, end }
    }
}

impl<I> Iterator for StepByObjectIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let item = self.iter.next();
        let _ = item.as_ref()?;

        for _ in 0..self.step - 1 {
            let next = self.iter.next();
            if next.is_none() {
                self.end = true;
                break;
            }
        }

        item
    }
}

/// Filter object for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FilterObject<O, F, R> {
    obj: O,
    f: F,
    _records: PhantomData<R>,
}

impl<O, F, R> FilterObject<O, F, R> {
    fn new(obj: O, f: F) -> Self {
        Self {
            obj,
            f,
            _records: PhantomData,
        }
    }
}

impl<O, R, F> Object<R> for FilterObject<O, F, R>
where
    O: Object<R>,
    F: Fn(Entity) -> bool + Clone,
{
    type Iter = FilterObjectIter<O::Iter, F>;

    fn cells(&self, records: &R) -> Self::Iter {
        FilterObjectIter::new(self.obj.cells(records), self.f.clone())
    }
}

/// Filter object iterator for any [`Object`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FilterObjectIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F> FilterObjectIter<I, F> {
    fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F> Iterator for FilterObjectIter<I, F>
where
    I: Iterator<Item = Entity>,
    F: Fn(Entity) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(item) => {
                    if (self.f)(item) {
                        return Some(item);
                    }
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grid::records::vec_records::VecRecords,
        settings::object::{Columns, Rows},
    };

    use super::*;

    #[test]
    fn test_skip_iterator() {
        use Entity::*;

        assert_eq!(
            cells(Rows::new(1..5).skip(1), 10, 10),
            [Row(2), Row(3), Row(4)]
        );

        assert_eq!(
            cells(Columns::new(5..).skip(1), 10, 10),
            [Column(6), Column(7), Column(8), Column(9)]
        );

        assert_eq!(cells((1, 5).skip(1), 10, 10), []);
        assert_eq!(cells((1, 5).skip(0), 10, 10), [Cell(1, 5)]);

        assert_eq!(
            cells(Rows::new(1..5).skip(0), 10, 10),
            [Row(1), Row(2), Row(3), Row(4)]
        );
    }

    #[test]
    fn test_step_by_iterator() {
        use Entity::*;

        assert_eq!(cells(Rows::new(1..5).step_by(0), 10, 10), []);
        assert_eq!(
            cells(Rows::new(1..5).step_by(1), 10, 10),
            [Row(1), Row(2), Row(3), Row(4)]
        );
        assert_eq!(cells(Rows::new(1..5).step_by(2), 10, 10), [Row(1), Row(3)]);

        assert_eq!(
            cells(Columns::new(5..).step_by(1), 10, 10),
            [Column(5), Column(6), Column(7), Column(8), Column(9)]
        );

        assert_eq!(cells((1, 5).step_by(2), 10, 10), [Cell(1, 5)]);
        assert_eq!(cells((1, 5).step_by(1), 10, 10), [Cell(1, 5)]);

        assert_eq!(cells(Rows::new(1..5).step_by(100), 10, 10), [Row(1)]);
    }

    #[test]
    fn test_filter_iterator() {
        use Entity::*;

        assert_eq!(
            cells(Rows::new(1..5).filter(|i| matches!(i, Row(3))), 10, 10),
            [Row(3)]
        );
        assert_eq!(cells(Rows::new(1..5).filter(|_| false), 10, 10), []);
        assert_eq!(
            cells(Rows::new(1..5).filter(|_| true), 10, 10),
            [Row(1), Row(2), Row(3), Row(4)]
        );
    }

    fn cells<O>(o: O, count_rows: usize, count_cols: usize) -> Vec<Entity>
    where
        O: Object<VecRecords<String>>,
    {
        let data = vec![vec![String::default(); count_cols]; count_rows];
        let records = VecRecords::new(data);
        o.cells(&records).collect::<Vec<_>>()
    }
}
