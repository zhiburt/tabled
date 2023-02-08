use std::borrow::Cow;

use papergrid::{records::IntoRecords, util::string::string_width_multiline_tab};

use crate::width::Truncate;

use super::EitherString;

#[derive(Debug)]
pub struct TruncatedRecords<'a, I> {
    records: I,
    width: Width<'a>,
    tab_size: usize,
}

impl TruncatedRecords<'_, ()> {
    pub fn new<I: IntoRecords>(
        records: I,
        width: Width<'_>,
        tab_size: usize,
    ) -> TruncatedRecords<'_, I> {
        TruncatedRecords {
            records,
            width,
            tab_size,
        }
    }
}

impl<'a, I> IntoRecords for TruncatedRecords<'a, I>
where
    I: IntoRecords,
{
    type Cell = EitherString<I::Cell>;
    type IterColumns = TruncatedRecordsColumnsIter<'a, <I::IterColumns as IntoIterator>::IntoIter>;
    type IterRows = TruncatedRecordsIter<'a, <I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        TruncatedRecordsIter {
            iter: self.records.iter_rows().into_iter(),
            width: self.width.clone(),
            tab_size: self.tab_size,
        }
    }
}

pub struct TruncatedRecordsIter<'a, I> {
    iter: I,
    width: Width<'a>,
    tab_size: usize,
}

impl<'a, I> Iterator for TruncatedRecordsIter<'a, I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = TruncatedRecordsColumnsIter<'a, <I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(TruncatedRecordsColumnsIter {
            iter: iter.into_iter(),
            current: 0,
            width: self.width.clone(),
            tab_size: self.tab_size,
        })
    }
}

pub struct TruncatedRecordsColumnsIter<'a, I> {
    iter: I,
    width: Width<'a>,
    tab_size: usize,
    current: usize,
}

impl<I> Iterator for TruncatedRecordsColumnsIter<'_, I>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    type Item = EitherString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.iter.next()?;

        let width = self.width.get(self.current);
        self.current += 1;

        let text_width = string_width_multiline_tab(s.as_ref(), self.tab_size);
        if text_width <= width {
            return Some(EitherString::Some(s));
        }

        let text = Truncate::truncate_text(s.as_ref(), width, self.tab_size);
        let text = text.into_owned();
        let text = EitherString::Owned(text);

        Some(text)
    }
}

#[derive(Debug, Clone)]
pub enum Width<'a> {
    Exact(usize),
    List(Cow<'a, [usize]>),
}

impl Width<'_> {
    pub fn get(&self, col: usize) -> usize {
        match self {
            Width::Exact(val) => *val,
            Width::List(cols) => cols[col],
        }
    }
}
