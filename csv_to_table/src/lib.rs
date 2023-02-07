#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unreachable_pub
)]
#![deny(unused_must_use)]

// mod table;
// mod records;

// pub trait TableFromCsv {
//     fn from_csv<R>(reader: Reader<R>) -> Table<CsvRecords<'_, R>>
//     where
//         R: Read;
// }

// impl TableFromCsv for Table {
//     fn from_csv<R>(reader: Reader<R>) -> CsvTable<R>
//     where
//         R: Read,
//     {
//     }
// }

use std::{io::Read, mem::transmute};

use csv::{StringRecord, StringRecordIter, StringRecordsIntoIter};
use tabled::records::IntoRecords;

struct CsvRecords<R> {
    rows: StringRecordsIntoIter<R>,
    err_logic: ErorrHandling,
}

enum ErorrHandling {
    Ignore,
    Print,
}

struct CsvStringRecordsRows<R>(StringRecordsIntoIter<R>);

impl<R> Iterator for CsvStringRecordsRows<R>
where
    R: Read,
{
    type Item = CsvStringRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.next()?;

        let record = match result {
            Ok(record) => record,
            // todo add a flag whether to ignore the errors
            Err(err) => {
                let err = err.to_string();
                let mut record = StringRecord::with_capacity(err.len(), 1);
                record.push_field(&err);
                record
            }
        };

        Some(CsvStringRecord::new(record))
    }
}

struct CsvStringRecord {
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

struct CsvStrField(&'static str);

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
        CsvStringRecordsRows(self.rows.into_iter())
    }
}

#[cfg(test)]
#[test]
fn foo() {
    let csv = csv::Reader::from_path(
        "/home/maxim/Downloads/annual-enterprise-survey-2021-financial-year-provisional-csv.csv",
    )
    .unwrap();
    let records = CsvRecords {
        rows: csv.into_records(),
        err_logic: ErorrHandling::Print,
    };

    let table = tabled::iter_table::Table::new(records);
    let table = table.build(std::io::stderr());

    println!("{:?}", table);
}

#[cfg(test)]
#[test]
fn boo2() {
    let csv = csv::Reader::from_path(
        "/home/maxim/Downloads/annual-enterprise-survey-2021-financial-year-provisional-csv.csv",
    )
    .unwrap();
    let records = CsvRecords {
        rows: csv.into_records(),
        err_logic: ErorrHandling::Print,
    };

    for row in records.iter_rows() {
        println!("///////////");

        for col in row {
            print!("{} ", col.as_ref());
        }

        println!();

        // println!("{} {}", row._record.as_slice(), row._record.len());
    }
}

// let mut t = Self { record, iter: None };

// let iter = t.record.iter();
// let static_iter =
//     unsafe { std::mem::transmute::<StringRecordIter<'_>, StringRecordIter<'static>>(iter) };

// t.iter = Some(static_iter);

// eprintln!("...........");
// eprintln!("{:?} {:?} {:?}", t.iter.as_mut().unwrap().next(), t.iter.as_mut().unwrap().next(), t.iter.as_mut().unwrap().next());

// t
