//! The module contains [`TruncateContent`] records iterator.

use crate::{
    grid::dimension::Dimension, grid::records::into_records::either_string::EitherString,
    grid::records::IntoRecords, grid::util::string::get_text_width, settings::width::Truncate,
};

/// A records iterator which truncates all cells to a given width.
#[derive(Debug)]
pub struct TruncateContent<I, D> {
    records: I,
    dimension: D,
}

impl TruncateContent<(), ()> {
    /// Creates new [`TruncateContent`] object.
    pub fn new<I, D>(records: I, dimension: D) -> TruncateContent<I, D> {
        TruncateContent { records, dimension }
    }
}

impl<I, D> IntoRecords for TruncateContent<I, D>
where
    I: IntoRecords,
    I::Cell: AsRef<str>,
    D: Clone + Dimension,
{
    type Cell = EitherString<I::Cell>;
    type IterColumns = TruncateContentColumnsIter<<I::IterColumns as IntoIterator>::IntoIter, D>;
    type IterRows = TruncateContentIter<<I::IterRows as IntoIterator>::IntoIter, D>;

    fn iter_rows(self) -> Self::IterRows {
        TruncateContentIter {
            iter: self.records.iter_rows().into_iter(),
            dimension: self.dimension.clone(),
        }
    }
}

/// A row iterator for [`TruncateContent`].
#[derive(Debug)]
pub struct TruncateContentIter<I, D> {
    iter: I,
    dimension: D,
}

impl<I, D> Iterator for TruncateContentIter<I, D>
where
    I: Iterator,
    I::Item: IntoIterator,
    D: Clone,
{
    type Item = TruncateContentColumnsIter<<I::Item as IntoIterator>::IntoIter, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        let iter = TruncateContentColumnsIter {
            iter: iter.into_iter(),
            iter_column: 0,
            dimension: self.dimension.clone(),
        };

        Some(iter)
    }
}

/// A column iterator for [`TruncateContent`].
#[derive(Debug)]
pub struct TruncateContentColumnsIter<I, D> {
    iter: I,
    dimension: D,
    iter_column: usize,
}

impl<I, D> Iterator for TruncateContentColumnsIter<I, D>
where
    I: Iterator,
    I::Item: AsRef<str>,
    D: Dimension,
{
    type Item = EitherString<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.iter.next()?;
        let text_ref = text.as_ref();

        let width = self.dimension.get_width(self.iter_column);
        self.iter_column += 1;

        let text_width = get_text_width(text_ref);
        let is_small = text_width <= width;
        if is_small {
            Some(EitherString::Some(text))
        } else {
            let text = Truncate::truncate(text_ref, width);
            let text = text.into_owned();
            Some(EitherString::Owned(text))
        }
    }
}
