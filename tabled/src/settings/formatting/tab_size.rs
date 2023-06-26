use crate::{
    grid::records::{Records, RecordsMut},
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

impl<R, D, C> TableOption<R, D, C> for TabSize
where
    for<'a> &'a R: Records,
    R: RecordsMut<String>,
{
    fn change(self, records: &mut R, _: &mut C, _: &mut D) {
        let mut list = vec![];
        for (row, cells) in records.iter_rows().into_iter().enumerate() {
            for (col, text) in cells.into_iter().enumerate() {
                let text = text.as_ref().replace('\t', &" ".repeat(self.0));
                list.push(((row, col), text));
            }
        }

        for (pos, text) in list {
            records.set(pos, text);
        }
    }
}
