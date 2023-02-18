#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]
#![deny(unused_must_use)]

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zhiburt/tabled/86ac146e532ce9f7626608d7fd05072123603a2e/assets/tabled-gear.svg"
)]

//! A library which provides a CSV input abstraction for [`tabled`].
//! It relies on [`csv`] library.
//!
//! # Example
//!
//! ```
//! let csv = "name,designed_by,invented_year\n\
//!            C,Dennis Ritchie,1972\n\
//!            Rust,Graydon Hoare,2010\n\
//!            Go,Rob Pike,2009";
//!
//! let table = csv_to_table::from_reader(csv.as_bytes()).unwrap().to_string();
//!
//! let expected = "+------+----------------+---------------+\n\
//!                 | name | designed_by    | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 | C    | Dennis Ritchie | 1972          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  | 2010          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Go   | Rob Pike       | 2009          |\n\
//!                 +------+----------------+---------------+";
//!
//! assert_eq!(table, expected);
//! ```
//!
//! You can also use [`iter`] to build an table from [`Iterator`].
//! It is usefull when you have a huge csv and don't want to load it all along into memory.
//! But it's interface might be a little bit less feature full cause of its limitations.
//!
//! ```
//! let csv = "name,designed_by,invented_year\n\
//!            C,Dennis Ritchie,1972\n\
//!            Rust,Graydon Hoare,2010\n\
//!            Go,Rob Pike,2009";
//!
//! let table = csv_to_table::iter::from_reader(csv.as_bytes()).to_string();
//!
//! let expected = "+------+----------------+---------------+\n\
//!                 | name | designed_by    | invented_year |\n\
//!                 +------+----------------+---------------+\n\
//!                 | C    | Dennis Ritchie | 1972          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Rust | Graydon Hoare  | 2010          |\n\
//!                 +------+----------------+---------------+\n\
//!                 | Go   | Rob Pike       | 2009          |\n\
//!                 +------+----------------+---------------+";
//!
//! assert_eq!(table, expected);
//! ```

mod records;

use std::{io::Read, path::Path};

use csv::{Reader, ReaderBuilder};
use tabled::{builder::Builder, tables::iter::IterTable, Table};

pub mod iter {
    //! The module contains [`CsvRecords`] which is an [`Iterator`] abstraction for [`IterTable`].

    use super::*;
    use std::fs::File;

    pub use super::records::CsvRecords;

    pub mod records {
        //! A module which contains [`CsvRecords`].

        pub use crate::records::*;
    }

    /// Creates [`IterTable`] from a csv [`Read`]er.
    pub fn from_reader<R: Read>(reader: R) -> IterTable<CsvRecords<R>> {
        let rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

        IterTable::new(CsvRecords::new(rdr))
    }

    /// Creates [`IterTable`] from a [`File`] which suppose to have a csv.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<IterTable<CsvRecords<File>>, csv::Error> {
        let rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

        let table = IterTable::new(CsvRecords::new(rdr));

        Ok(table)
    }

    /// Creates [`IterTable`] from a [`csv::Reader`].
    pub fn from_csv_reader<R: Read>(
        reader: Reader<R>,
    ) -> Result<IterTable<CsvRecords<R>>, csv::Error> {
        let table = IterTable::new(CsvRecords::new(reader));

        Ok(table)
    }
}

/// Creates [`Table`] from [`Read`]er.
///
/// Notice that in case of big files you might better you [`iter::CsvRecords`].
pub fn from_reader<R: Read>(reader: R) -> Result<Table, csv::Error> {
    let rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

    read_into_table(rdr)
}

/// Creates [`Table`] from a csv [`File`].
///
/// Notice that in case of big files you might better you [`iter::CsvRecords`].
///
/// [`File`]: std::fs::File
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Table, csv::Error> {
    let rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

    read_into_table(rdr)
}

/// Creates [`Table`] from a [`csv::Reader`].
///
/// Notice that in case of big files you might better you [`iter::CsvRecords`].
pub fn from_csv_reader<R: Read>(reader: Reader<R>) -> Result<Table, csv::Error> {
    read_into_table(reader)
}

fn read_into_table<R: Read>(reader: Reader<R>) -> Result<Table, csv::Error> {
    let mut builder = Builder::default();

    for record in reader.into_records() {
        let record = record?;
        let iter = record.iter().map(|s| s.to_owned());
        builder.push_record(iter);
    }

    let table = builder.build();

    Ok(table)
}
