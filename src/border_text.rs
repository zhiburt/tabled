use std::borrow::Cow;

use papergrid::records::Records;

use crate::{Table, TableOption};

/// [`BorderText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, style::BorderText};
///
/// let table = Table::new(["Hello World"])
///     .with(BorderText::first("+-.table"));
///
/// assert_eq!(
///     table.to_string(),
///     "+-.table------+\n\
///      | &str        |\n\
///      +-------------+\n\
///      | Hello World |\n\
///      +-------------+"
/// );
/// ```
#[derive(Debug)]
pub struct BorderText<'a> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
    row: SplitLineIndex,
}

#[derive(Debug)]
enum SplitLineIndex {
    First,
    Last,
    Line(usize),
}

impl<'a> BorderText<'a> {
    /// Creates a [`BorderText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S>(line: usize, text: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            text: text.into(),
            row: SplitLineIndex::Line(line),
        }
    }

    /// Creates a [`BorderText`] instance for a top line.
    pub fn first<S>(text: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            text: text.into(),
            row: SplitLineIndex::First,
        }
    }

    /// Creates a [`BorderText`] instance for a bottom line.
    pub fn last<S>(text: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            text: text.into(),
            row: SplitLineIndex::Last,
        }
    }
}

impl<'a, R> TableOption<R> for BorderText<'a>
where
    for<'b> &'b R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        let row = match self.row {
            SplitLineIndex::First => 0,
            SplitLineIndex::Last => table.shape().0,
            SplitLineIndex::Line(row) => {
                if row > table.shape().0 {
                    return;
                }

                row
            }
        };

        table
            .get_config_mut()
            .override_split_line(row, self.text.as_ref());
    }
}
