//! A module contains [`BufRows`] and [`BufColumns`] iterators.
//!
//! Almoust always they both can be used interchangeably but [`BufRows`] is supposed to be lighter cause it
//! does not reads columns.

use crate::grid::records::IntoRecords;

use super::either_string::EitherString;

/// BufRecords inspects [`IntoRecords`] iterator and keeps read data buffered.
/// So it can be checking before hand.
#[derive(Debug)]
pub struct BufRows<I, T> {
    iter: I,
    buf: Vec<T>,
}

impl BufRows<(), ()> {
    /// Creates a new [`BufRows`] structure, filling the buffer.
    pub fn new<I: IntoRecords>(
        records: I,
        sniff: usize,
    ) -> BufRows<<I::IterRows as IntoIterator>::IntoIter, I::IterColumns> {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for _ in 0..sniff {
            match iter.next() {
                Some(row) => buf.push(row),
                None => break,
            }
        }

        BufRows { iter, buf }
    }
}

impl<I, T> BufRows<I, T> {
    /// Returns a slice of a record buffer.
    pub fn as_slice(&self) -> &[T] {
        &self.buf
    }
}

impl<I, T> From<BufRows<I, T>> for BufColumns<I>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn from(value: BufRows<I, T>) -> Self {
        let buf = value
            .buf
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.as_ref().to_string()).collect())
            .collect();

        BufColumns {
            iter: value.iter,
            buf,
        }
    }
}

impl<I, T> IntoRecords for BufRows<I, T>
where
    I: Iterator<Item = T>,
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    type Cell = T::Item;
    type IterColumns = T;
    type IterRows = BufRowIter<I, T>;

    fn iter_rows(self) -> Self::IterRows {
        BufRowIter {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

/// Buffered [`Iterator`].
#[derive(Debug)]
pub struct BufRowIter<I, T> {
    buf: std::vec::IntoIter<T>,
    iter: I,
}

impl<I, T> Iterator for BufRowIter<I, T>
where
    I: Iterator<Item = T>,
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(i) => Some(i),
            None => self.iter.next(),
        }
    }
}

/// BufRecords inspects [`IntoRecords`] iterator and keeps read data buffered.
/// So it can be checking before hand.
///
/// In contrast to [`BufRows`] it keeps records by columns.
#[derive(Debug)]
pub struct BufColumns<I> {
    iter: I,
    buf: Vec<Vec<String>>,
}

impl BufColumns<()> {
    /// Creates new [`BufColumns`] structure, filling the buffer.
    pub fn new<I: IntoRecords>(
        records: I,
        sniff: usize,
    ) -> BufColumns<<I::IterRows as IntoIterator>::IntoIter> {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for _ in 0..sniff {
            match iter.next() {
                Some(row) => {
                    let row = row
                        .into_iter()
                        .map(|cell| cell.as_ref().to_string())
                        .collect::<Vec<_>>();
                    buf.push(row)
                }
                None => break,
            }
        }

        BufColumns { iter, buf }
    }
}

impl<I> BufColumns<I> {
    /// Returns a slice of a keeping buffer.
    pub fn as_slice(&self) -> &[Vec<String>] {
        &self.buf
    }
}

impl<I> IntoRecords for BufColumns<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = EitherString<<I::Item as IntoIterator>::Item>;
    type IterColumns = EitherRowIterator<<I::Item as IntoIterator>::IntoIter>;
    type IterRows = BufColumnIter<I>;

    fn iter_rows(self) -> Self::IterRows {
        BufColumnIter {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

/// A row iterator for [`BufColumns`]
#[derive(Debug)]
pub struct BufColumnIter<I> {
    buf: std::vec::IntoIter<Vec<String>>,
    iter: I,
}

impl<I> Iterator for BufColumnIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = EitherRowIterator<<I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(i) => Some(EitherRowIterator::Owned(i.into_iter())),
            None => self
                .iter
                .next()
                .map(|i| EitherRowIterator::Some(i.into_iter())),
        }
    }
}

/// An iterator over some iterator or allocated buffer.
#[derive(Debug)]
pub enum EitherRowIterator<I> {
    /// Allocated iterator.
    Owned(std::vec::IntoIter<String>),
    /// Given iterator.
    Some(I),
}

impl<I> Iterator for EitherRowIterator<I>
where
    I: Iterator,
{
    type Item = EitherString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherRowIterator::Owned(iter) => iter.next().map(EitherString::Owned),
            EitherRowIterator::Some(iter) => iter.next().map(EitherString::Some),
        }
    }
}
