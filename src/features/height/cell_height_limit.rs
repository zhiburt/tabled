use papergrid::{
    records::{Records, RecordsMut},
    util::get_lines,
    width::CfgWidthFunction,
    Entity,
};

use crate::{measurement::Measurement, peaker::Peaker, CellOption, Height, Table, TableOption};

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
    R: Records + RecordsMut<String>,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let height = self.height.measure(table.get_records(), table.get_config());

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let cell_height = records.count_lines(pos);
            if cell_height <= height {
                continue;
            }

            let content = records.get_text(pos);
            let content = limit_lines(content, height);
            let ctrl = CfgWidthFunction::from_cfg(table.get_config());
            table.get_records_mut().set(pos, content, &ctrl);
        }

        table.destroy_height_cache();
        table.destroy_width_cache();
    }
}

impl<W, R> TableOption<R> for CellHeightLimit<W>
where
    W: Measurement<Height>,
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        TableHeightLimit::new(self.height.measure(table.get_records(), table.get_config()))
            .change(table)
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
