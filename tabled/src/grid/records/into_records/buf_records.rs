//! A module contains [`BufRecords`] iterator.

use crate::grid::records::IntoRecords;

/// BufRecords inspects [`IntoRecords`] iterator and keeps read data buffered.
/// So it can be checking before hand.
#[derive(Debug)]
pub struct BufRecords<I, T> {
    iter: I,
    buf: Vec<Vec<T>>,
}

impl BufRecords<(), ()> {
    /// Creates new [`BufRecords`] structure, filling the buffer.
    pub fn new<I>(
        records: I,
        sniff: usize,
    ) -> BufRecords<<I::IterRows as IntoIterator>::IntoIter, I::Cell>
    where
        I: IntoRecords,
    {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for data in iter.by_ref().take(sniff) {
            let data = data.into_iter().collect::<Vec<_>>();
            buf.push(data)
        }

        BufRecords { iter, buf }
    }
}

impl<I, T> BufRecords<I, T> {
    /// Returns a slice of a keeping buffer.
    pub fn as_slice(&self) -> &[Vec<T>] {
        &self.buf
    }
}

impl<I> IntoRecords for BufRecords<I, <I::Item as IntoIterator>::Item>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Cell = <I::Item as IntoIterator>::Item;
    type IterColumns = BufIterator<<I::Item as IntoIterator>::IntoIter, Self::Cell>;
    type IterRows = BufRecordsIter<I, Self::Cell>;

    fn iter_rows(self) -> Self::IterRows {
        BufRecordsIter {
            iter: self.iter,
            buf: self.buf.into_iter(),
        }
    }
}

/// A row iterator for [`BufRecords`]
#[derive(Debug)]
pub struct BufRecordsIter<I, T> {
    iter: I,
    buf: std::vec::IntoIter<Vec<T>>,
}

impl<I> Iterator for BufRecordsIter<I, <I::Item as IntoIterator>::Item>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = BufIterator<<I::Item as IntoIterator>::IntoIter, <I::Item as IntoIterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(i) => Some(BufIterator::Buffered(i.into_iter())),
            None => self
                .iter
                .next()
                .map(|i| BufIterator::Iterator(i.into_iter())),
        }
    }
}

/// An iterator over some iterator or allocated buffer.
#[derive(Debug)]
pub enum BufIterator<I, T> {
    /// Allocated iterator.
    Buffered(std::vec::IntoIter<T>),
    /// Given iterator.
    Iterator(I),
}

impl<I> Iterator for BufIterator<I, I::Item>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BufIterator::Buffered(iter) => iter.next(),
            BufIterator::Iterator(iter) => iter.next(),
        }
    }
}
