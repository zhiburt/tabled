use std::borrow::Cow;

use papergrid::records::Records;

use crate::{Table, TableOption};

use super::Offset;

/// [`BorderText`] writes a custom text on a border.
///
/// # Example
///
/// ```rust
/// use tabled::{Table, BorderText};
///
/// let mut table = Table::new(["Hello World"]);
/// table
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
    offset: Offset,
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
            offset: Offset::Begin(0),
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
            offset: Offset::Begin(0),
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
            offset: Offset::Begin(0),
        }
    }

    /// Set an offset from which the text will be started.
    pub fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }
}

impl<'a, R> TableOption<R> for BorderText<'a>
where
    R: Records,
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
            .override_split_line(row, self.text.as_ref(), self.offset.into());
    }
}
