use std::{io::Read, mem::transmute};

use csv::{Reader, StringRecord, StringRecordsIntoIter};
use tabled::records::IntoRecords;

pub struct CsvRecords<R> {
    rows: StringRecordsIntoIter<R>,
    err_logic: ErorrLogic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErorrLogic {
    Ignore,
    Print,
}

impl Default for ErorrLogic {
    fn default() -> Self {
        Self::Ignore
    }
}

impl<R> CsvRecords<R> {
    pub fn new(reader: Reader<R>, err_logic: ErorrLogic) -> Self
    where
        R: Read,
    {
        Self {
            rows: reader.into_records(),
            err_logic,
        }
    }
}

pub struct CsvStringRecordsRows<R>(StringRecordsIntoIter<R>, ErorrLogic);

impl<R> Iterator for CsvStringRecordsRows<R>
where
    R: Read,
{
    type Item = CsvStringRecord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.0.next()?;

            match result {
                Ok(record) => return Some(CsvStringRecord::new(record)),
                Err(err) => match self.1 {
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

pub struct CsvStrField(&'static str);

impl AsRef<str> for CsvStrField {
    fn as_ref(&self) -> &str {
        &self.0
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
        CsvStringRecordsRows(self.rows.into_iter(), self.err_logic)
    }
}
