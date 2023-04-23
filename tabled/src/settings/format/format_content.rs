use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// A lambda which formats cell content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatContent<F> {
    f: F,
    multiline: bool,
}

impl<F> FormatContent<F> {
    pub(crate) fn new(f: F) -> Self {
        Self {
            f,
            multiline: false,
        }
    }
}

impl<F> FormatContent<F> {
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
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }
}

impl<F, R, D, C> TableOption<R, D, C> for FormatContent<F>
where
    F: FnMut(&str) -> String + Clone,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut C, _: &mut D) {
        CellOption::change(self, records, cfg, Entity::Global);
    }
}

impl<F, R, C> CellOption<R, C> for FormatContent<F>
where
    F: FnMut(&str) -> String + Clone,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(mut self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            let is_valid_pos = pos.0 < count_rows && pos.1 < count_cols;
            if !is_valid_pos {
                continue;
            }

            let content = records.get_text(pos);
            let content = if self.multiline {
                multiline(self.f.clone())(content)
            } else {
                (self.f)(content)
            };
            records.set(pos, content);
        }
    }
}

fn multiline<F: FnMut(&str) -> String>(mut f: F) -> impl FnMut(&str) -> String {
    move |s: &str| {
        let mut v = Vec::new();
        for line in s.lines() {
            v.push(f(line));
        }

        v.join("\n")
    }
}
