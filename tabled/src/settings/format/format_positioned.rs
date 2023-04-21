use crate::{
    grid::config::Entity,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
    settings::{CellOption, TableOption},
};

/// [`FormatContentPositioned`] is like a [`FormatContent`] an abstraction over a function you can use against a cell.
///
/// It different from [`FormatContent`] that it provides a row and column index.
///
/// [`FormatContent`]: crate::settings::format::FormatContent
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatContentPositioned<F>(F);

impl<F> FormatContentPositioned<F> {
    pub(crate) fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F, R, D, C> TableOption<R, D, C> for FormatContentPositioned<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
{
    fn change(self, records: &mut R, cfg: &mut C, _: &mut D) {
        CellOption::change(self, records, cfg, Entity::Global);
    }
}

impl<F, R, C> CellOption<R, C> for FormatContentPositioned<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
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
            let content = (self.0)(content, pos);
            records.set(pos, content);
        }
    }
}
