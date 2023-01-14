use core::fmt::{self, Display};
use std::{cell::Cell, io::Read};

use csv::{Reader, StringRecord, StringRecordIter, StringRecordsIntoIter};
use tabled::{papergrid::records::Records, Table};

struct CsvTable<R> {
    reader: Cell<Option<Reader<R>>>,
    window_data: Vec<Vec<String>>,
    lookup_window: usize,
}

impl<R> CsvTable<R>
where
    R: Read,
{
    pub fn new(reader: Reader<R>) -> Self {
        let mut reader = reader;
        let t = reader.into_records();
        t.next();

        Self {
            reader: Cell::new(Some(reader)),
            window_data: Vec::new(),
            lookup_window: 4,
        }
    }
}

impl<R> Display for CsvTable<R>
where
    R: Read,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reader = match self.reader.take() {
            Some(reader) => reader,
            None => return Ok(()),
        };

        let has_headers = reader.has_headers();

        Ok(())
    }
}
