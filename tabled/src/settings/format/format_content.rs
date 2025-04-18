use crate::{
    grid::{
        config::{Entity, Position},
        records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    },
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

impl<F, R, D, C> TableOption<R, C, D> for FormatContent<F>
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
        let max_pos = Position::new(count_rows, count_cols);

        for pos in entity.iter(count_rows, count_cols) {
            if !max_pos.has_coverage(pos) {
                continue;
            }

            let text = records.get_text(pos);

            let new_text = if self.multiline {
                multiline(self.f.clone())(text)
            } else {
                (self.f)(text)
            };

            records.set(pos, new_text);
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
