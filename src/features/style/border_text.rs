use std::borrow::Cow;

use crate::{grid::config::GridConfig, records::ExactRecords, TableOption};

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
pub struct BorderText<'a, Line> {
    // todo: offset from which we start overriding border
    // offset: usize,
    text: Cow<'a, str>,
    offset: Offset,
    line: Line,
}

pub struct LineIndex(usize);

pub struct LineFirst;

pub struct LineLast;

impl<'a> BorderText<'a, ()> {
    /// Creates a [`BorderText`] instance.
    ///
    /// Lines are numbered from 0 to the `count_rows` included
    /// (`line >= 0 && line <= count_rows`).
    pub fn new<S: Into<Cow<'a, str>>>(line: usize, text: S) -> BorderText<'a, LineIndex> {
        BorderText::create(text.into(), Offset::Begin(0), LineIndex(line))
    }

    /// Creates a [`BorderText`] instance for a top line.
    pub fn first<S>(text: S) -> BorderText<'a, LineFirst>
    where
        S: Into<Cow<'a, str>>,
    {
        BorderText::create(text.into(), Offset::Begin(0), LineFirst)
    }

    /// Creates a [`BorderText`] instance for a bottom line.
    pub fn last<S>(text: S) -> BorderText<'a, LineLast>
    where
        S: Into<Cow<'a, str>>,
    {
        BorderText::create(text.into(), Offset::Begin(0), LineLast)
    }

    /// Set an offset from which the text will be started.
    pub fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }

    fn create<L>(text: Cow<'a, str>, offset: Offset, line: L) -> BorderText<'a, L> {
        BorderText { text, line, offset }
    }
}

impl<R, D> TableOption<R, D> for BorderText<'_, LineFirst> {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        cfg.override_split_line(0, self.text.as_ref(), self.offset.into());
    }
}

impl<R, D> TableOption<R, D> for BorderText<'_, LineIndex> {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        cfg.override_split_line(self.line.0, self.text.as_ref(), self.offset.into());
    }
}

impl<R, D> TableOption<R, D> for BorderText<'_, LineLast>
where
    R: ExactRecords,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        cfg.override_split_line(records.count_rows(), self.text.as_ref(), self.offset.into());
    }
}
