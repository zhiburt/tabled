use std::{fmt::Debug, io::Read};

use csv::{Reader, StringRecord, StringRecordsIntoIter};
use tabled::grid::records::IntoRecords;

/// A [`IntoRecords`] implementation for a [`csv::Reader`].
///
/// By default all errors are ignored,
/// but you can return them using [`CsvRecordsIter::set_catch`].
///
/// [`CsvRecordsIter::set_catch`]: CsvRecordsIter.set_catch
pub struct CsvRecords<R> {
    rows: StringRecordsIntoIter<R>,
}

impl<R> CsvRecords<R> {
    /// Creates a new [`CsvRecords`] structure.
    pub fn new(reader: Reader<R>) -> Self
    where
        R: Read,
    {
        Self {
            rows: reader.into_records(),
        }
    }
}

impl<R> IntoRecords for CsvRecords<R>
where
    R: Read,
{
    type Cell = String;
    type IterColumns = CsvStringRecord;
    type IterRows = CsvRecordsIter<R>;

    fn iter_rows(self) -> Self::IterRows {
        CsvRecordsIter {
            iter: self.rows,
            err_logic: ErrorLogic::Ignore,
            err: None,
        }
    }
}

/// A row iterator.
pub struct CsvRecordsIter<R> {
    iter: StringRecordsIntoIter<R>,
    err_logic: ErrorLogic,
    err: Option<std::io::Error>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ErrorLogic {
    Ignore,
    Catch,
}

impl<R> CsvRecordsIter<R> {
    /// Return a status
    ///
    /// It's a cought by a catcher you can set by [`CsvRecordsIter::set_catch`].
    pub fn status(&self) -> Option<&std::io::Error> {
        self.err.as_ref()
    }

    /// Show underlying [Read] errors inside a table.
    pub fn set_catch(mut self, catch: bool) -> Self {
        self.err_logic = if catch {
            ErrorLogic::Catch
        } else {
            ErrorLogic::Ignore
        };

        self
    }
}

impl<R> Iterator for CsvRecordsIter<R>
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
                    ErrorLogic::Ignore => continue,
                    ErrorLogic::Catch => {
                        self.err = Some(std::io::Error::from(err));
                        return None;
                    }
                },
            }
        }
    }
}

/// A column iterator.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.record.get(self.i)?;
        let text = String::from(text);

        self.i += 1;

        Some(text)
    }
}
