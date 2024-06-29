//! This module contains an [`ExtendedTable`] structure which is useful in cases where
//! a structure has a lot of fields.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Tabled, tables::ExtendedTable};
//!
//! #[derive(Tabled)]
//! struct Language {
//!     name: &'static str,
//!     designed_by: &'static str,
//!     invented_year: usize,
//! }
//!
//! let languages = vec![
//!     Language{
//!         name: "C",
//!         designed_by: "Dennis Ritchie",
//!         invented_year: 1972
//!     },
//!     Language{
//!         name: "Rust",
//!         designed_by: "Graydon Hoare",
//!         invented_year: 2010
//!     },
//!     Language{
//!         name: "Go",
//!         designed_by: "Rob Pike",
//!         invented_year: 2009
//!     },
//! ];
//!
//! let table = ExtendedTable::new(languages).to_string();
//!
//! let expected = "-[ RECORD 0 ]-+---------------\n\
//!                 name          | C\n\
//!                 designed_by   | Dennis Ritchie\n\
//!                 invented_year | 1972\n\
//!                 -[ RECORD 1 ]-+---------------\n\
//!                 name          | Rust\n\
//!                 designed_by   | Graydon Hoare\n\
//!                 invented_year | 2010\n\
//!                 -[ RECORD 2 ]-+---------------\n\
//!                 name          | Go\n\
//!                 designed_by   | Rob Pike\n\
//!                 invented_year | 2009";
//!
//! assert_eq!(table, expected);
//! ```

use std::cell::RefCell;
use std::fmt::{self, Debug, Display};
use std::rc::Rc;
use crate::grid::util::string::string_width;
use crate::Tabled;

/// `ExtendedTable` display data in a 'expanded display mode' from postgresql.
/// It may be useful for a large data sets with a lot of fields.
///
/// See 'Examples' in <https://www.postgresql.org/docs/current/app-psql.html>.
///
/// It escapes strings to resolve a multi-line ones.
/// Because of that ANSI sequences will be not be rendered too so colores will not be showed.
///
/// ```
/// use tabled::tables::ExtendedTable;
///
/// let data = vec!["Hello", "2021"];
/// let table = ExtendedTable::new(&data).to_string();
///
/// assert_eq!(
///     table,
///     concat!(
///         "-[ RECORD 0 ]-\n",
///         "&str | Hello\n",
///         "-[ RECORD 1 ]-\n",
///         "&str | 2021",
///     )
/// );
/// ```
#[derive(Clone)]
pub struct ExtendedTable {
    fields: Vec<String>,
    records: Vec<Vec<String>>,
    template: Rc<RefCell<dyn Fn(usize) -> String>>,
}

impl ExtendedTable {
    /// Creates a new instance of `ExtendedTable`
    pub fn new<T>(iter: impl IntoIterator<Item = T>) -> Self
    where
        T: Tabled,
    {
        let data = iter
            .into_iter()
            .map(|i| {
                i.fields()
                    .into_iter()
                    .map(|s| s.escape_debug().to_string())
                    .collect()
            })
            .collect();
        let header = T::headers()
            .into_iter()
            .map(|s| s.escape_debug().to_string())
            .collect();

        Self {
            records: data,
            fields: header,
            template: Rc::new(RefCell::new(record_template)),
        }
    }

    /// Truncates table to a set width value for a table.
    /// It returns a success inticator, where `false` means it's not possible to set the table width,
    /// because of the given arguments.
    ///
    /// It tries to not affect fields, but if there's no enough space all records will be deleted and fields will be cut.
    ///
    /// The minimum width is 14.
    pub fn truncate(&mut self, max: usize, suffix: &str) -> bool {
        // -[ RECORD 0 ]-
        let teplate_width = self.records.len().to_string().len() + 13;
        let min_width = teplate_width;
        if max < min_width {
            return false;
        }

        let suffix_width = string_width(suffix);
        if max < suffix_width {
            return false;
        }

        let max = max - suffix_width;

        let fields_max_width = self
            .fields
            .iter()
            .map(|s| string_width(s))
            .max()
            .unwrap_or_default();

        // 3 is a space for ' | '
        let fields_affected = max < fields_max_width + 3;
        if fields_affected {
            if max < 3 {
                return false;
            }

            let max = max - 3;

            if max < suffix_width {
                return false;
            }

            let max = max - suffix_width;

            truncate_fields(&mut self.fields, max, suffix);
            truncate_records(&mut self.records, 0, suffix);
        } else {
            let max = max - fields_max_width - 3 - suffix_width;
            truncate_records(&mut self.records, max, suffix);
        }

        true
    }

    /// Sets the template for a record.
    pub fn template<F>(mut self, template: F) -> Self
    where
        F: Fn(usize) -> String + 'static,
    {
        self.template = Rc::new(RefCell::new(template));
        self
    }
}

impl From<Vec<Vec<String>>> for ExtendedTable {
    fn from(mut data: Vec<Vec<String>>) -> Self {
        if data.is_empty() {
            return Self {
                fields: vec![],
                records: vec![],
                template: Rc::new(RefCell::new(record_template)),
            };
        }

        let fields = data.remove(0);

        Self {
            fields,
            records: data,
            template: Rc::new(RefCell::new(record_template)),
        }
    }
}

impl Display for ExtendedTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.records.is_empty() {
            return Ok(());
        }

        // It's possible that field|header can be a multiline string so
        // we escape it and trim \" chars.
        let fields = self.fields.iter().collect::<Vec<_>>();

        let max_field_width = fields
            .iter()
            .map(|s| string_width(s))
            .max()
            .unwrap_or_default();

        let max_values_length = self
            .records
            .iter()
            .map(|record| record.iter().map(|s| string_width(s)).max())
            .max()
            .unwrap_or_default()
            .unwrap_or_default();

        for (i, records) in self.records.iter().enumerate() {
            write_header_template(f, &self.template, i, max_field_width, max_values_length)?;

            for (value, field) in records.iter().zip(fields.iter()) {
                writeln!(f)?;
                write_record(f, field, value, max_field_width)?;
            }

            let is_last_record = i + 1 == self.records.len();
            if !is_last_record {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Debug for ExtendedTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtendedTable")
            .field("fields", &self.fields)
            .field("records", &self.records)
            .finish_non_exhaustive()
    }
}

fn truncate_records(records: &mut Vec<Vec<String>>, max_width: usize, suffix: &str) {
    for fields in records {
        truncate_fields(fields, max_width, suffix);
    }
}

fn truncate_fields(records: &mut Vec<String>, max_width: usize, suffix: &str) {
    for text in records {
        truncate(text, max_width, suffix);
    }
}

fn write_header_template(
    f: &mut fmt::Formatter<'_>,
    template: &Rc<RefCell<dyn Fn(usize) -> String>>,
    index: usize,
    max_field_width: usize,
    max_values_length: usize,
) -> fmt::Result {
    let record_template = template.borrow()(index);
    let mut template = format!("-{record_template}-");
    let default_template_length = template.len();

    // 3 - is responsible for ' | ' formatting
    let max_line_width = std::cmp::max(
        max_field_width + 3 + max_values_length,
        default_template_length,
    );
    let rest_to_print = max_line_width - default_template_length;
    if rest_to_print > 0 {
        // + 1 is a space after field name and we get a next pos so its +2
        if max_field_width + 2 > default_template_length {
            let part1 = (max_field_width + 1) - default_template_length;
            let part2 = rest_to_print - part1 - 1;

            template.extend(
                std::iter::repeat('-')
                    .take(part1)
                    .chain(std::iter::once('+'))
                    .chain(std::iter::repeat('-').take(part2)),
            );
        } else {
            template.extend(std::iter::repeat('-').take(rest_to_print));
        }
    }

    write!(f, "{template}")?;

    Ok(())
}

fn write_record(
    f: &mut fmt::Formatter<'_>,
    field: &str,
    value: &str,
    max_field_width: usize,
) -> fmt::Result {
    write!(f, "{field:max_field_width$} | {value}")
}

fn truncate(text: &mut String, max: usize, suffix: &str) {
    let original_len = text.len();

    if max == 0 || text.is_empty() {
        *text = String::new();
    } else {
        *text = crate::util::string::cut_str2(text, max).into_owned();
    }

    let cut_was_done = text.len() < original_len;
    if !suffix.is_empty() && cut_was_done {
        text.push_str(suffix);
    }
}

fn record_template(index: usize) -> String {
    format!("[ RECORD {index} ]")
}
