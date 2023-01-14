#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

mod table;
mod records;

pub trait TableFromCsv {
    fn from_csv<R>(reader: Reader<R>) -> Table<CsvRecords<'_, R>>
    where
        R: Read;
}

impl TableFromCsv for Table {
    fn from_csv<R>(reader: Reader<R>) -> CsvTable<R>
    where
        R: Read,
    {
    }
}
