use crate::records::IntoRecords;

use super::either_string::EitherString;

#[derive(Debug)]
pub struct BufRecords<I, T> {
    iter: I,
    buf: Vec<T>,
}

impl BufRecords<(), ()> {
    pub fn new<I: IntoRecords>(
        records: I,
        sniff: usize,
    ) -> BufRecords<<I::IterRows as IntoIterator>::IntoIter, I::IterColumns> {
        let mut buf = vec![];

        let mut iter = records.iter_rows().into_iter();
        for _ in 0..sniff {
            match iter.next() {
                Some(row) => buf.push(row),
                None => break,
            }
        }

        BufRecords { iter, buf }
    }
}

impl<I, T> BufRecords<I, T> {
    pub fn as_slice(&self) -> &[T] {
        &self.buf
    }
}

impl<I, T> From<BufRecords<I, T>> for BufRecords2<I>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn from(value: BufRecords<I, T>) -> Self {
        let buf = value
            .buf
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.as_ref().to_string()).collect())
            .collect();

        BufRecords2 {
            iter: value.iter,
            buf,
        }
    }
}

impl<I, T> IntoRecords for BufRecords<I, T>
where
    I: Iterator<Item = T>,
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    type Cell = T::Item;
    type IterColumns = T;
    type IterRows = BufRecordsIter<I, T>;

    fn iter_rows(self) -> Self::IterRows {
        BufRecordsIter {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

pub struct BufRecordsIter<I, T> {
    buf: std::vec::IntoIter<T>,
    iter: I,
}

impl<I, T> Iterator for BufRecordsIter<I, T>
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

#[derive(Debug)]
pub struct BufRecords2<I> {
    iter: I,
    buf: Vec<Vec<String>>,
}

impl BufRecords2<()> {
    pub fn new<I: IntoRecords>(
        records: I,
        sniff: usize,
    ) -> BufRecords2<<I::IterRows as IntoIterator>::IntoIter> {
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

        BufRecords2 { iter, buf }
    }
}

impl<I> BufRecords2<I> {
    pub fn as_slice(&self) -> &[Vec<String>] {
        &self.buf
    }
}

impl<I> IntoRecords for BufRecords2<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Cell = EitherString<<I::Item as IntoIterator>::Item>;
    type IterColumns = EitherRowIterator<<I::Item as IntoIterator>::IntoIter>;
    type IterRows = BufRecordsIter2<I>;

    fn iter_rows(self) -> Self::IterRows {
        BufRecordsIter2 {
            buf: self.buf.into_iter(),
            iter: self.iter,
        }
    }
}

pub struct BufRecordsIter2<I> {
    buf: std::vec::IntoIter<Vec<String>>,
    iter: I,
}

impl<I> Iterator for BufRecordsIter2<I>
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

pub enum EitherRowIterator<I> {
    Owned(std::vec::IntoIter<String>),
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
