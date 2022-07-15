//! This module contains an [`ExpandedDisplay`] structure which is useful in cases where
//! a structure has a lot of fields.
//!
#![cfg_attr(feature = "derive", doc = "```")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! use tabled::{Tabled, display::ExpandedDisplay};
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
//! let table = ExpandedDisplay::new(languages).to_string();
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

use std::fmt;

use papergrid::{cut_str, string_width_multiline};

use crate::{width::wrap_text, Tabled};

/// `ExpandedDisplay` display data in a 'expanded display mode' from postgresql.
/// It may be useful for a large data sets with a lot of fields.
///
/// See 'Examples' in <https://www.postgresql.org/docs/current/app-psql.html.>.
///
/// It escapes strings to resolve a multi-line ones.
/// Because of that `colors` may not be rendered.
///
/// ```
/// use tabled::{display::ExpandedDisplay};
///
/// let data = vec!["Hello", "2021"];
/// let table = ExpandedDisplay::new(&data);
///
/// assert_eq!(
///     table.to_string(),
///     concat!(
///         "-[ RECORD 0 ]-\n",
///         "&str | Hello\n",
///         "-[ RECORD 1 ]-\n",
///         "&str | 2021",
///     )
/// );
/// ```
pub struct ExpandedDisplay {
    format_record_splitter: Option<fn(usize) -> String>,
    format_value: Option<Box<dyn Fn(&str) -> String>>,
    fields: Vec<String>,
    records: Vec<Vec<String>>,
}

impl ExpandedDisplay {
    /// Creates a new instance of `ExpandedDisplay`
    pub fn new<T: Tabled>(iter: impl IntoIterator<Item = T>) -> Self {
        let data = iter.into_iter().map(|i| i.fields()).collect();
        let header = T::headers();

        Self {
            records: data,
            fields: header,
            format_record_splitter: None,
            format_value: None,
        }
    }

    /// Sets a line format which will be used to split records.
    ///
    /// Default formatting is "-[ RECORD {} ]-".
    ///
    /// At least one '\n' char will be printed at the end regardless if you set it or not.
    pub fn header_template(&mut self, f: fn(usize) -> String) -> &mut Self {
        self.format_record_splitter = Some(f);
        self
    }

    /// Sets a value formatter.
    ///
    /// This method overrides others formatters like [`ExpandedDisplay::truncate`] and [`ExpandedDisplay::wrap`].
    pub fn formatter(&mut self, f: impl Fn(&str) -> String + 'static) -> &mut Self {
        self.format_value = Some(Box::new(f));
        self
    }

    /// Sets max width of value.
    /// The rest will be trunceted.
    pub fn truncate(&mut self, max: usize, tail: impl AsRef<str>) -> &mut Self {
        let tail = tail.as_ref().to_string();
        self.format_value = Some(Box::new(move |s| {
            let mut trucated = truncate(s, max);
            if trucated.len() < s.len() {
                trucated.push_str(&tail);
            }

            trucated
        }));
        self
    }

    /// Sets max width of value,
    /// when limit is reached next chars will be placed on the next line.
    pub fn wrap(&mut self, max: usize) -> &mut Self {
        self.format_value = Some(Box::new(move |s| wrap(s, max)));
        self
    }
}

impl std::fmt::Display for ExpandedDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.records.is_empty() {
            return Ok(());
        }

        let format_value = |value: &String| match &self.format_value {
            Some(f) => (f)(value),
            None => value.to_string(),
        };

        // It's possible that field|header can be a multiline string so
        // we escape it and trim \" chars.
        let fields = self
            .fields
            .iter()
            .map(|f| {
                let escaped = format!("{:?}", f);
                escaped
                    .chars()
                    .skip(1)
                    .take(escaped.len() - 1 - 1)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        let max_field_width = fields
            .iter()
            .map(|f| string_width_multiline(f))
            .max()
            .unwrap_or_default();

        let values = self
            .records
            .iter()
            .map(|record| {
                assert_eq!(record.len(), fields.len());

                record.iter().map(format_value).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_values_length = values
            .iter()
            .map(|record| record.iter().map(|v| string_width_multiline(v)).max())
            .max()
            .unwrap_or_default()
            .unwrap_or_default();

        for (i, records) in values.iter().enumerate() {
            match self.format_record_splitter {
                Some(f_header) => {
                    let header = (f_header)(i);
                    write!(f, "{}", header)?;
                }
                None => {
                    write_header_template(f, i, max_field_width, max_values_length)?;
                }
            }

            for (value, field) in records.iter().zip(fields.iter()) {
                writeln!(f)?;
                write_record(f, field, value, max_field_width)?;
            }

            let is_last_record = i + 1 == values.len();
            if !is_last_record {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for ExpandedDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpandedDisplay")
            .field("format_record_splitter", &self.format_record_splitter)
            .field("format_value", &self.format_value.is_some())
            .field("fields", &self.fields)
            .field("records", &self.records)
            .finish()
    }
}

fn write_header_template(
    f: &mut std::fmt::Formatter<'_>,
    index: usize,
    max_field_width: usize,
    max_values_length: usize,
) -> std::fmt::Result {
    let mut template = format!("-[ RECORD {} ]-", index);
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

    write!(f, "{}", template)?;

    Ok(())
}

fn write_record(
    f: &mut std::fmt::Formatter<'_>,
    field: &str,
    value: &str,
    max_field_width: usize,
) -> std::fmt::Result {
    if value.is_empty() {
        write!(f, "{:width$} | {}", field, value, width = max_field_width)?;
        return Ok(());
    }

    for (i, line) in value.lines().enumerate() {
        if i > 0 {
            writeln!(f)?;
        }

        let field = if i == 0 { field } else { "" };
        write!(f, "{:width$} | {}", field, line, width = max_field_width)?;
    }

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    cut_str(s, max).into_owned()
}

fn wrap(s: &str, max: usize) -> String {
    wrap_text(s, max, false)
}
