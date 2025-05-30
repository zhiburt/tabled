use std::borrow::Cow;

use crate::{
    grid::config::{Entity, Position},
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// A structure to handle special chars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Charset;

impl Charset {
    /// Returns [`CleanCharset`] which removes all `\t` and `\r` occurrences.
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

/// [`CleanCharset`] removes all `\t` and `\r` occurrences.
///
/// # Example
///
/// ```
/// use std::iter::FromIterator;
/// use tabled::{
///     Table, builder::Builder,
///     settings::formatting::Charset,
/// };
///
/// let text = "Some text which was created on windows \r\n yes they use this \\r\\n";
///
/// let mut builder = Builder::from(Table::from_iter([[text]]));
/// builder.insert_record(0, ["win. text"]);
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

impl CleanCharset {
    /// Removes all symbols which may break the layout such as `\t`, `\r` and more.
    ///
    /// Notice that tab is just removed rather then being replaced with spaces.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::settings::formatting::CleanCharset;
    ///
    /// assert_eq!(
    ///     CleanCharset::clean("Some\ttext\t\twith \\tabs\r\nSome"),
    ///     "Sometextwith \\tabs\nSome"
    /// )
    /// ```
    pub fn clean(s: &str) -> Cow<'_, str> {
        Cow::Owned(clean_charset(s))
    }
}

impl<R, D, C> TableOption<R, C, D> for CleanCharset
where
    R: Records + ExactRecords + RecordsMut<String> + PeekableRecords,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        // TODO: Add a grid iterator which produces POS to squash these for loops

        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let pos = Position::new(row, col);
                let text = records.get_text(pos);
                let text = clean_charset(text);
                records.set(pos, text);
            }
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
            let text = clean_charset(text);
            records.set(pos, text);
        }
    }
}

fn clean_charset(text: &str) -> String {
    // It's enough for covering '\t' and '\r'
    // as well as a list of other unwanted escapes.
    text.replace(|c| c != '\n' && c < ' ', "")
}
