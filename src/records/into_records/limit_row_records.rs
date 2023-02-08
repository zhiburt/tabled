use crate::records::IntoRecords;

#[derive(Debug)]
pub struct RowLimitRecords<I> {
    records: I,
    limit: usize,
}

impl RowLimitRecords<()> {
    pub fn new<I: IntoRecords>(records: I, limit: usize) -> RowLimitRecords<I> {
        RowLimitRecords { records, limit }
    }
}

impl<I> IntoRecords for RowLimitRecords<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = I::IterColumns;
    type IterRows = RowLimitRecordsIter<<I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        RowLimitRecordsIter {
            iter: self.records.iter_rows().into_iter(),
            limit: self.limit,
        }
    }
}

pub struct RowLimitRecordsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for RowLimitRecordsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }

        self.limit -= 1;

        self.iter.next()
    }
}
