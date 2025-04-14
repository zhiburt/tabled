use crate::{
    grid::{
        config::{ColoredConfig, Entity, Position},
        dimension::CompleteDimensionVecRecords,
        records::{ExactRecords, IntoRecords, PeekableRecords, Records, RecordsMut},
        util::string::{count_lines, get_lines},
    },
    settings::{measurement::Measurement, peaker::Peaker, CellOption, Height, TableOption},
};

use super::table_height_limit::TableHeightLimit;

/// A modification for cell/table to increase its height.
///
/// If used for a [`Table`] [`PriorityNone`] is used.
///
/// [`PriorityNone`]: crate::settings::peaker::PriorityNone
/// [`Table`]: crate::Table
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellHeightLimit<W = usize> {
    height: W,
}

impl<W> CellHeightLimit<W> {
    /// Constructs a new object.
    pub fn new(height: W) -> Self
    where
        W: Measurement<Height>,
    {
        Self { height }
    }

    /// Set's a priority by which the limit logic will be applied.
    pub fn priority<P>(self, priority: P) -> TableHeightLimit<W, P>
    where
        P: Peaker,
        W: Measurement<Height>,
    {
        TableHeightLimit::new(self.height).priority(priority)
    }
}

impl<W, R> CellOption<R, ColoredConfig> for CellHeightLimit<W>
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
            let count_lines = count_lines(text);

            if count_lines <= height {
                continue;
            }

            let content = limit_lines(text, height);
            records.set(pos, content);
        }
    }
}

impl<R, W> TableOption<R, ColoredConfig, CompleteDimensionVecRecords<'_>> for CellHeightLimit<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
    for<'a> <<&'a R as Records>::Iter as IntoRecords>::Cell: AsRef<str>,
{
    fn change(
        self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'_>,
    ) {
        let height = self.height.measure(&*records, cfg);
        TableHeightLimit::new(height).change(records, cfg, dims)
    }
}

fn limit_lines(s: &str, n: usize) -> String {
    let mut text = String::new();
    for (i, line) in get_lines(s).take(n).enumerate() {
        if i > 0 {
            text.push('\n');
        }

        text.push_str(&line);
    }

    text
}
