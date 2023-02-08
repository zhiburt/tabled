#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

mod records;

use std::{io::Read, path::Path};

use csv::Reader;
use tabled::{builder::Builder, table::iter::IterTable, Table};

pub mod iter {
    use super::*;
    use std::fs::File;

    use records::ErorrLogic;

    pub use records::CsvRecords;

    pub fn from_reader<R: Read>(reader: R) -> IterTable<CsvRecords<R>> {
        let reader = csv::Reader::from_reader(reader);
        IterTable::new(CsvRecords::new(reader, ErorrLogic::Ignore))
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<IterTable<CsvRecords<File>>, csv::Error> {
        let reader = csv::Reader::from_path(path)?;
        let table = IterTable::new(CsvRecords::new(reader, ErorrLogic::Ignore));

        Ok(table)
    }

    pub fn from_csv_reader<R: Read>(
        reader: Reader<R>,
    ) -> Result<IterTable<CsvRecords<R>>, csv::Error> {
        let table = IterTable::new(CsvRecords::new(reader, ErorrLogic::Ignore));

        Ok(table)
    }
}

pub fn from_reader<R: Read>(reader: R) -> Result<Table, csv::Error> {
    let reader = csv::Reader::from_reader(reader);
    read_into_table(reader)
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Table, csv::Error> {
    let reader = csv::Reader::from_path(path)?;
    read_into_table(reader)
}

pub fn from_csv_reader<R: Read>(reader: Reader<R>) -> Result<Table, csv::Error> {
    read_into_table(reader)
}

fn read_into_table<R: Read>(reader: Reader<R>) -> Result<Table, csv::Error> {
    let mut builder = Builder::default();
    for record in reader.into_records() {
        let record = record?;
        let iter = (&record).iter().map(|s| s.to_owned());
        builder.push_record(iter);
    }
    let table = builder.build();
    Ok(table)
}
