use papergrid::records::IntoRecords;

#[derive(Debug)]
pub struct ColumnLimitRecords<I> {
    records: I,
    limit: usize,
}

impl ColumnLimitRecords<()> {
    pub fn new<I: IntoRecords>(records: I, limit: usize) -> ColumnLimitRecords<I> {
        ColumnLimitRecords { records, limit }
    }
}

impl<I> IntoRecords for ColumnLimitRecords<I>
where
    I: IntoRecords,
{
    type Cell = I::Cell;
    type IterColumns = ColumnLimitRecordsColumnsIter<<I::IterColumns as IntoIterator>::IntoIter>;
    type IterRows = ColumnLimitRecordsIter<<I::IterRows as IntoIterator>::IntoIter>;

    fn iter_rows(self) -> Self::IterRows {
        ColumnLimitRecordsIter {
            iter: self.records.iter_rows().into_iter(),
            limit: self.limit,
        }
    }
}

pub struct ColumnLimitRecordsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for ColumnLimitRecordsIter<I>
where
    I: Iterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
{
    type Item = ColumnLimitRecordsColumnsIter<<I::Item as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.iter.next()?;
        Some(ColumnLimitRecordsColumnsIter {
            iter: iter.into_iter(),
            limit: self.limit,
        })
    }
}

pub struct ColumnLimitRecordsColumnsIter<I> {
    iter: I,
    limit: usize,
}

impl<I> Iterator for ColumnLimitRecordsColumnsIter<I>
where
    I: Iterator,
    I::Item: AsRef<str>,
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
