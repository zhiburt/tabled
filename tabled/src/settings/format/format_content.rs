use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::CellOption,
};

use super::FormatContentPositioned;

/// A lambda which formats cell content.
#[derive(Debug)]
pub struct FormatContent<F>(pub F)
where
    F: FnMut(&str) -> String;

impl<F> FormatContent<F>
where
    F: FnMut(&str) -> String,
{
    /// Multiline a helper function for changing multiline content of cell.
    /// Using this formatting applied for all rows not to a string as a whole.
    ///
    /// ```rust,no_run
    /// use tabled::{Table, settings::{Format, object::Segment, Modify}};
    ///
    /// let data: Vec<&'static str> = Vec::new();
    /// let table = Table::new(&data)
    ///     .with(Modify::new(Segment::all()).with(Format::content(|s| s.to_string()).multiline()))
    ///     .to_string();
    /// ```
    pub fn multiline(self) -> FormatContent<impl FnMut(&str) -> String> {
        FormatContent(multiline(self.0))
    }
}

impl<F, R, C> CellOption<R, C> for FormatContent<F>
where
    F: FnMut(&str) -> String,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut C, entity: Entity) {
        FormatContentPositioned(|s, _| (self.0)(s)).change(records, cfg, entity);
    }
}

fn multiline<F>(mut f: F) -> impl FnMut(&str) -> String
where
    F: FnMut(&str) -> String,
{
    move |s: &str| {
        let mut v = Vec::new();
        for line in s.lines() {
            v.push(f(line));
        }

        v.join("\n")
    }
}
