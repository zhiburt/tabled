use crate::{
    grid::config::ColoredConfig,
    grid::config::Entity,
    grid::dimension::CompleteDimensionVecRecords,
    grid::records::{ExactRecords, PeekableRecords, Records, RecordsMut},
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

    /// The priority makes scence only for table, so the function
    /// converts it to [`TableHeightIncrease`] with a given priority.
    pub fn priority<P>(self) -> TableHeightIncrease<W, P>
    where
        P: Peaker,
        W: Measurement<Height>,
    {
        TableHeightIncrease::new(self.height).priority::<P>()
    }
}

impl<W, R> CellOption<R, ColoredConfig> for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let height = self.height.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            let is_valid_pos = pos.0 < count_rows && pos.1 < count_columns;
            if !is_valid_pos {
                continue;
            }

            let text = records.get_text(pos);

            let cell_height = count_lines(text);
            if cell_height >= height {
                continue;
            }

            let content = add_lines(text, height - cell_height);
            records.set(pos, content);
        }
    }
}

impl<R, W> TableOption<R, CompleteDimensionVecRecords<'static>, ColoredConfig>
    for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + PeekableRecords,
    for<'a> &'a R: Records,
{
    fn change(
        self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'static>,
    ) {
        let height = self.height.measure(&*records, cfg);
        TableHeightIncrease::new(height).change(records, cfg, dims)
    }
}

fn add_lines(s: &str, n: usize) -> String {
    let mut text = String::with_capacity(s.len() + n);
    text.push_str(s);
    text.extend(std::iter::repeat('\n').take(n));

    text
}
