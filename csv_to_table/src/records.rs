use std::{fmt::Debug, io::Read, mem::transmute};

use csv::{Reader, StringRecord, StringRecordsIntoIter};
use tabled::records::IntoRecords;

/// A [`IntoRecords`] implementation for a [`csv::Reader`].
///
/// By default all errors are ignored, but you can print them using [`CsvRecords::print_erorrs`].
pub struct CsvRecords<R> {
    rows: StringRecordsIntoIter<R>,
    err_logic: ErorrLogic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ErorrLogic {
    Ignore,
    Print,
}

impl<R> CsvRecords<R> {
    /// Creates a new [`CsvRecords`] structure.
    pub fn new(reader: Reader<R>) -> Self
    where
        R: Read,
    {
        Self {
            rows: reader.into_records(),
            err_logic: ErorrLogic::Ignore,
        }
    }

    /// Show underlying [Read] errors inside a table.
    pub fn print_errors(mut self) -> Self {
        self.err_logic = ErorrLogic::Print;
        self
    }
}

/// A row iterator.
pub struct CsvStringRecordsRows<R> {
    iter: StringRecordsIntoIter<R>,
    err_logic: ErorrLogic,
}

impl<R> Iterator for CsvStringRecordsRows<R>
where
    R: Read,
{
    type Item = CsvStringRecord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.iter.next()?;

            match result {
                Ok(record) => return Some(CsvStringRecord::new(record)),
                Err(err) => match self.err_logic {
                    ErorrLogic::Ignore => continue,
                    ErorrLogic::Print => {
                        let err = err.to_string();
                        let mut record = StringRecord::with_capacity(err.len(), 1);
                        record.push_field(&err);

                        return Some(CsvStringRecord::new(record));
                    }
                },
            }
        }
    }
}

/// A column iterator.
#[derive(Debug)]
pub struct CsvStringRecord {
    record: StringRecord,
    i: usize,
}

impl CsvStringRecord {
    fn new(record: StringRecord) -> Self {
        Self { record, i: 0 }
    }
}

impl Iterator for CsvStringRecord {
    type Item = CsvStrField;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.record.get(self.i)?;
        let text = unsafe { transmute::<&str, &'static str>(text) };

        self.i += 1;
        Some(CsvStrField(text))
    }
}

/// A cell struct.
///
/// # SAFETY
///
/// NOTICE that it has a &'static lifetime which is not true.
/// It's made so only cause of trait limitations.
///
/// It's unsafe to keep the reference around.
///
/// It's OK for [`CsvRecords`] cause we do not keep it internally.
#[derive(Debug)]
pub struct CsvStrField(&'static str);

impl AsRef<str> for CsvStrField {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<R> IntoRecords for CsvRecords<R>
where
    R: Read,
{
    type Cell = CsvStrField;
    type IterColumns = CsvStringRecord;
    type IterRows = CsvStringRecordsRows<R>;

    fn iter_rows(self) -> Self::IterRows {
        CsvStringRecordsRows {
            iter: self.rows,
            err_logic: self.err_logic,
        }
    }
}
