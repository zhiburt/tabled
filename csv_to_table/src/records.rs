use std::io::Read;

use csv::{StringRecord, StringRecordsIntoIter, Reader};

pub struct CsvRecords<R> {
    size: (usize, usize),
    records: StringRecordsIntoIter<R>,  
    first_record: Option<Result<StringRecord>>,
}

impl<R> CsvRecords<R> {
    pub fn new(records: Reader<R>) -> Self
    where
        R: Read + Clone,
    {
        let mut count_rows = 0;
        let mut count_column = 0;

        loop {
            match records.next() {
                Some(Ok(record)) => {
                    next = Some(record);
                    count_columns = record.len();
                    break;
                }
                Some(Err(..)) => {}
                None => break,
            }
        }

        let count_rows = records.reader().

        Self {
            size: (count_rows, count_columns),
            first_record: next,
            records,
        }
    }
}

impl Records for CsvRecords<'_> {
    fn count_rows(&self) -> usize {
        todo!()
    }

    fn count_columns(&self) -> usize {
        todo!()
    }

    fn get_text(&self, pos: tabled::papergrid::Position) -> &str {
        todo!()
    }

    fn get_line(&self, pos: tabled::papergrid::Position, i: usize) -> &str {
        todo!()
    }

    fn count_lines(&self, pos: tabled::papergrid::Position) -> usize {
        todo!()
    }

    fn get_width<W>(&self, pos: tabled::papergrid::Position, width_ctrl: W) -> usize
    where
        W: tabled::papergrid::width::WidthFunc,
    {
        todo!()
    }

    fn get_line_width<W>(&self, pos: tabled::papergrid::Position, i: usize, width_ctrl: W) -> usize
    where
        W: tabled::papergrid::width::WidthFunc,
    {
        todo!()
    }

    fn fmt_text_prefix(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        pos: tabled::papergrid::Position,
    ) -> std::fmt::Result {
        todo!()
    }

    fn fmt_text_suffix(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        pos: tabled::papergrid::Position,
    ) -> std::fmt::Result {
        todo!()
    }
}
