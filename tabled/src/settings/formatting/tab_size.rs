use crate::{
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::TableOption,
};

/// Set a tab size.
///
/// The size is used in order to calculate width correctly.
///
/// Default value is 4 (basically 1 '\t' equals 4 spaces).
///
/// IMPORTANT: The tab character might be not present in output,
/// it might be replaced by spaces.
///
/// # Example
///
/// ```
/// use tabled::{Table, settings::formatting::TabSize};
///
/// let text = "Some\ttext\t\twith \\tabs";
///
/// let mut table = Table::new([text]);
/// table.with(TabSize::new(4));
///
/// assert_eq!(
///     table.to_string(),
///     "+--------------------------------+\n\
///      | &str                           |\n\
///      +--------------------------------+\n\
///      | Some    text        with \\tabs |\n\
///      +--------------------------------+"
/// )
/// ```
#[derive(Debug, Default, Clone)]
pub struct TabSize(usize);

impl TabSize {
    /// Creates new [`TabSize`] object.
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<R, D, C> TableOption<R, C, D> for TabSize
where
    R: Records + ExactRecords + RecordsMut<String> + PeekableRecords,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let tab_size = self.0;

        for row in 0..records.count_rows() {
            for col in 0..records.count_columns() {
                let pos = (row, col);
                let text = records.get_text(pos);
                let text = text.replace('\t', &" ".repeat(tab_size));
                records.set(pos, text);
            }
        }
    }
}
