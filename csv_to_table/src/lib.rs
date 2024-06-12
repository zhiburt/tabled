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
//! It is useful when you have a huge csv and don't want to load it all along into memory.
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
use tabled::{builder::Builder, tables::IterTable, Table};

pub mod iter {
    //! The module contains [`CsvRecords`] which is an [`Iterator`] abstraction for [`IterTable`].

    use super::*;
    use std::fs::File;

    pub use super::records::*;

    /// Creates [`IterTable`] from a csv [`Read`]er.
    ///
    /// # Example
    ///
    /// ```
    /// use csv_to_table::iter::from_reader;
    ///
    /// let csv = r#"Name,Job Tittle,Number
    /// Maxim,Plummer,12345
    /// Alex,Sowftware Developer,45678"#;
    ///
    /// let table = from_reader(csv.as_bytes());
    ///
    /// let table = table.to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     "+-------+---------------------+--------+\n\
    ///      | Name  | Job Tittle          | Number |\n\
    ///      +-------+---------------------+--------+\n\
    ///      | Maxim | Plummer             | 12345  |\n\
    ///      +-------+---------------------+--------+\n\
    ///      | Alex  | Sowftware Developer | 45678  |\n\
    ///      +-------+---------------------+--------+",
    /// );
    /// ```
    pub fn from_reader<R>(reader: R) -> IterTable<CsvRecords<R>>
    where
        R: Read,
    {
        let rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

        IterTable::new(CsvRecords::new(rdr))
    }

    /// Creates [`IterTable`] from a [`File`] which suppose to have a csv.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use csv_to_table::iter::from_path;
    ///
    /// let table = from_path("path/to/a/file").expect("success read");
    /// let table = table.to_string();
    /// ```
    pub fn from_path<P>(path: P) -> Result<IterTable<CsvRecords<File>>, csv::Error>
    where
        P: AsRef<Path>,
    {
        let rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

        let table = IterTable::new(CsvRecords::new(rdr));

        Ok(table)
    }

    /// Creates [`IterTable`] from a [`csv::Reader`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use csv_to_table::iter::from_csv_reader;
    /// use csv::Reader;
    ///
    /// let reader = Reader::from_path("path/to/a/file").expect("failed to find a file");
    ///
    /// let table = from_csv_reader(reader).expect("failed to read from a reader");
    /// let table = table.to_string();
    /// ```
    pub fn from_csv_reader<R>(reader: Reader<R>) -> Result<IterTable<CsvRecords<R>>, csv::Error>
    where
        R: Read,
    {
        let table = IterTable::new(CsvRecords::new(reader));

        Ok(table)
    }
}

/// Creates [`Table`] from [`Read`]er.
///
/// Notice that in case of big files you might better use [`iter::CsvRecords`].
///
/// # Example
///
/// ```rust,no_run
/// use std::fs::File;
/// use csv_to_table::from_reader;
///
/// let file = File::open("/path/to/a/file").expect("failed to open a file");
///
/// let table = from_reader(file).expect("failed to read from a file");
/// let table = table.to_string();
/// ```
pub fn from_reader<R>(reader: R) -> Result<Table, csv::Error>
where
    R: Read,
{
    let rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

    read_into_table(rdr)
}

/// Creates [`Table`] from a csv [`File`].
///
/// Notice that in case of big files you might better use [`iter::CsvRecords`].
///
/// # Example
///
/// ```rust,no_run
/// use csv_to_table::from_path;
///
/// let table = from_path("path/to/a/file").expect("success read");
/// let table = table.to_string();
/// ```
///
/// [`File`]: std::fs::File
pub fn from_path<P>(path: P) -> Result<Table, csv::Error>
where
    P: AsRef<Path>,
{
    let rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

    read_into_table(rdr)
}

/// Creates [`Table`] from a [`csv::Reader`].
///
/// Notice that in case of big files you might better use [`iter::CsvRecords`].
///
/// # Example
///
/// ```rust,no_run
/// use csv_to_table::from_csv_reader;
/// use csv::Reader;
///
/// let reader = Reader::from_path("path/to/a/file").expect("failed to find a file");
///
/// let table = from_csv_reader(reader).expect("failed to read from a reader");
/// let table = table.to_string();
/// ```
pub fn from_csv_reader<R>(reader: Reader<R>) -> Result<Table, csv::Error>
where
    R: Read,
{
    read_into_table(reader)
}

fn read_into_table<R>(reader: Reader<R>) -> Result<Table, csv::Error>
where
    R: Read,
{
    let mut builder = Builder::default();

    for record in reader.into_records() {
        let record = record?;
        let iter = record.iter().map(|s| s.to_owned());
        builder.push_record(iter);
    }

    let table = builder.build();

    Ok(table)
}
