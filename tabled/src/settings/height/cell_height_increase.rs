use crate::{
    grid::config::{ColoredConfig, Entity, Position},
    grid::dimension::CompleteDimension,
    grid::records::{ExactRecords, IntoRecords, PeekableRecords, Records, RecordsMut},
    grid::util::string::count_lines,
    settings::{measurement::Measurement, peaker::Peaker, CellOption, Height, TableOption},
};

use super::TableHeightIncrease;

/// A modification for cell/table to increase its height.
///
/// If used for a [`Table`] [`PriorityNone`] is used.
///
/// [`PriorityNone`]: crate::settings::peaker::PriorityNone
/// [`Table`]: crate::Table
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellHeightIncrease<W = usize> {
    height: W,
}

impl<W> CellHeightIncrease<W> {
    /// Creates a new object of the structure.
    pub fn new(height: W) -> Self
    where
        W: Measurement<Height>,
    {
        Self { height }
    }

    /// The priority makes sense only for table, so the function
    /// converts it to [`TableHeightIncrease`] with a given priority.
    pub fn priority<P>(self, priority: P) -> TableHeightIncrease<W, P>
    where
        P: Peaker,
        W: Measurement<Height>,
    {
        TableHeightIncrease::new(self.height).priority(priority)
    }
}

impl<W, R> CellOption<R, ColoredConfig> for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let height = self.height.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();
        let max_pos = Position::new(count_rows, count_columns);

        for pos in entity.iter(count_rows, count_columns) {
            if !max_pos.has_coverage(pos) {
                continue;
            }

            let text = records.get_text(pos);

            // TOOD: We have a Cell?
            // I mean we could use Cell trait here

            let cell_height = count_lines(text);
            if cell_height >= height {
                continue;
            }

            let content = add_lines(text, height - cell_height);
            records.set(pos, content);
        }
    }
}

impl<R, W> TableOption<R, ColoredConfig, CompleteDimension<'_>> for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, dims: &mut CompleteDimension<'_>) {
        let height = self.height.measure(&*records, cfg);
        TableHeightIncrease::new(height).change(records, cfg, dims)
    }

    fn hint_change(&self) -> Option<Entity> {
        Some(Entity::Row(0))
    }
}

fn add_lines(s: &str, n: usize) -> String {
    let mut text = String::with_capacity(s.len() + n);
    text.push_str(s);
    text.extend(std::iter::repeat_n('\n', n));

    text
}
