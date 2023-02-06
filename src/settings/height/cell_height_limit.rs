use papergrid::util::string::{count_lines, get_lines};

use crate::{
    grid::config::{Entity, GridConfig},
    measurement::Measurement,
    peaker::Peaker,
    records::{ExactRecords, Records, RecordsMut},
    CellOption, Height,
};

use super::table_height_limit::TableHeightLimit;

/// A modification for cell/table to increase its height.
///
/// If used for a [`Table`] [`PriorityNone`] is used.
///
/// [`PriorityNone`]: crate::peaker::PriorityNone
#[derive(Debug)]
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
    pub fn priority<P>(self) -> TableHeightLimit<W, P>
    where
        P: Peaker,
        W: Measurement<Height>,
    {
        TableHeightLimit::new(self.height).priority::<P>()
    }
}

impl<W, R> CellOption<R> for CellHeightLimit<W>
where
    W: Measurement<Height>,
    R: Records + ExactRecords + RecordsMut<Text = String>,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let height = self.height.measure(&*records, cfg);

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        for pos in entity.iter(count_rows, count_columns) {
            let text = records.get_cell(pos);
            let count_lines = count_lines(text.as_ref());

            if count_lines <= height {
                continue;
            }

            let content = limit_lines(text.as_ref(), height);
            records.set(pos, content);
        }
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
