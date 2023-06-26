use papergrid::{
    config::Entity,
    records::{ExactRecords, PeekableRecords},
};

use crate::{
    grid::records::{Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// A structure to handle special chars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Charset;

impl Charset {
    /// Returns [`CleanCharset`] which removes all `\t` and `\r` occurences.
    ///
    /// Notice that tab is just removed rather then being replaced with spaces.
    /// You might be better call [`TabSize`] first if you not expect such behavior.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, settings::formatting::Charset};
    ///
    /// let text = "Some\ttext\t\twith \\tabs";
    ///
    /// let mut table = Table::new([text]);
    /// table.with(Charset::clean());
    ///
    /// assert_eq!(
    ///     table.to_string(),
    ///     "+--------------------+\n\
    ///      | &str               |\n\
    ///      +--------------------+\n\
    ///      | Sometextwith \\tabs |\n\
    ///      +--------------------+"
    /// )
    /// ```
    ///
    /// [`TabSize`]: crate::settings::formatting::TabSize
    pub fn clean() -> CleanCharset {
        CleanCharset
    }
}

/// [`CleanCharset`] removes all `\t` and `\r` occurences.
///
/// # Example
///
/// ```
/// use tabled::{Table, settings::formatting::Charset};
///
/// let text = "Some text which was created on windows \r\n yes they use this \\r\\n";
///
/// let mut builder = Table::builder([text]);
/// builder.set_header(["win. text"]);
///
/// let mut table = builder.build();
/// table.with(Charset::clean());
///
/// assert_eq!(
///     table.to_string(),
///     "+-----------------------------------------+\n\
///      | win. text                               |\n\
///      +-----------------------------------------+\n\
///      | Some text which was created on windows  |\n\
///      |  yes they use this \\r\\n                 |\n\
///      +-----------------------------------------+"
/// )
/// ```
#[derive(Debug, Default, Clone)]
pub struct CleanCharset;

impl<R, D, C> TableOption<R, D, C> for CleanCharset
where
    for<'a> &'a R: Records,
    R: RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let mut list = vec![];
        for (row, cells) in records.iter_rows().into_iter().enumerate() {
            for (col, text) in cells.into_iter().enumerate() {
                let text = text.as_ref().replace(['\t', '\r'], "");
                list.push(((row, col), text));
            }
        }

        for (pos, text) in list {
            records.set(pos, text);
        }
    }
}

impl<R, C> CellOption<R, C> for CleanCharset
where
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();
        for pos in entity.iter(count_rows, count_cols) {
            let text = records.get_text(pos);
            let text = text.replace(['\t', '\r'], "");
            records.set(pos, text);
        }
    }
}
