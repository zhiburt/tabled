//! The module contains [`TruncateContent`] records iterator.

use std::borrow::Cow;

use crate::{
    grid::records::IntoRecords, grid::util::string::string_width_multiline,
    settings::width::Truncate,
};

use super::either_string::EitherString;

/// A records iterator which truncates all cells to a given width.
#[derive(Debug)]
pub struct TruncateContent<'a, I> {
    records: I,
    width: ExactValue<'a>,
}

impl TruncateContent<'_, ()> {
    /// Creates new [`TruncateContent`] object.
    pub fn new<I: IntoRecords>(records: I, width: ExactValue<'_>) -> TruncateContent<'_, I> {
        TruncateContent { records, width }
    }
}

impl<'a, I> IntoRecords for TruncateContent<'a, I>
where
    I: IntoRecords,
{
    type Cell = EitherString<I::Cell>;
    type IterColumns = TruncateContentColumnsIter<'a, <I::IterColumns as IntoIterator>::IntoIter>;
    type IterRows = TruncateContentIter<'a, <I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        TruncateContentIter {
            iter: self.records.iter_rows().into_iter(),
            width: self.width.clone(),
        }
    }
}

/// A row iterator for [`TruncateContent`].
#[derive(Debug)]
pub struct TruncateContentIter<'a, I> {
    iter: I,
    width: ExactValue<'a>,
}

impl<'a, I> Iterator for TruncateContentIter<'a, I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = TruncateContentColumnsIter<'a, <I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(TruncateContentColumnsIter {
            iter: iter.into_iter(),
            current: 0,
            width: self.width.clone(),
        })
    }
}

/// A column iterator for [`TruncateContent`].
#[derive(Debug)]
pub struct TruncateContentColumnsIter<'a, I> {
    iter: I,
    width: ExactValue<'a>,
    current: usize,
}

impl<I> Iterator for TruncateContentColumnsIter<'_, I>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    type Item = EitherString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.iter.next()?;

        let width = self.width.get(self.current);
        self.current += 1;

        let text_width = string_width_multiline(s.as_ref());
        if text_width <= width {
            return Some(EitherString::Some(s));
        }

        let text = Truncate::truncate_text(s.as_ref(), width);
        let text = text.into_owned();
        let text = EitherString::Owned(text);

        Some(text)
    }
}

/// A width value.
#[derive(Debug, Clone)]
pub enum ExactValue<'a> {
    /// Const width value.
    Exact(usize),
    /// A list of width values for columns.
    List(Cow<'a, [usize]>),
}

impl<'a> From<&'a [usize]> for ExactValue<'a> {
    fn from(value: &'a [usize]) -> Self {
        Self::List(value.into())
    }
}

impl From<Vec<usize>> for ExactValue<'_> {
    fn from(value: Vec<usize>) -> Self {
        Self::List(value.into())
    }
}

impl From<usize> for ExactValue<'_> {
    fn from(value: usize) -> Self {
        Self::Exact(value)
    }
}

impl ExactValue<'_> {
    /// Get a width by column.
    pub fn get(&self, col: usize) -> usize {
        match self {
            ExactValue::Exact(val) => *val,
            ExactValue::List(cols) => cols[col],
        }
    }
}
