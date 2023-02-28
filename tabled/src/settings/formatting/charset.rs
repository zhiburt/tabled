use papergrid::records::Records;

use crate::{records::RecordsMut, settings::TableOption};

/// A structure to handle special chars.
#[derive(Debug, Default, Clone)]
pub struct Charset;

impl Charset {
    /// Returns [`CleanCharset`] which removes all `\t` and `\r` occurences.
    pub fn clean() -> CleanCharset {
        CleanCharset
    }
}

/// [`CleanCharset`] removes all `\t` and `\r` occurences.
#[derive(Debug, Default, Clone)]
pub struct CleanCharset;

impl<R, D, C> TableOption<R, D, C> for CleanCharset
where
    for<'a> &'a R: Records,
    R: RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, _: &mut C, _: &mut D) {
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
