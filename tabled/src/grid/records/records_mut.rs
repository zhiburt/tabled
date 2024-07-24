use crate::grid::config::Position;
#[cfg(feature = "std")]
use crate::grid::records::vec_records::{Text, VecRecords};

/// A [`Records`] representation which can modify cell by (row, column) index.
///
/// [`Records`]: crate::grid::records::Records
pub trait RecordsMut<Text> {
    /// Sets a text to a given cell by index.
    fn set(&mut self, pos: Position, text: Text);
}

impl<T, Text> RecordsMut<Text> for &'_ mut T
where
    T: RecordsMut<Text>,
{
    fn set(&mut self, pos: Position, text: Text) {
        T::set(self, pos, text)
    }
}

#[cfg(feature = "std")]
impl RecordsMut<String> for VecRecords<Text<String>> {
    fn set(&mut self, (row, col): Position, text: String) {
        self[row][col] = Text::new(text);
    }
}

#[cfg(feature = "std")]
impl RecordsMut<&str> for VecRecords<Text<String>> {
    fn set(&mut self, (row, col): Position, text: &str) {
        self[row][col] = Text::new(text.to_string());
    }
}
