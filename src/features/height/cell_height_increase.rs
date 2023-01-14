use papergrid::{
    records::{Records, RecordsMut},
    width::CfgWidthFunction,
    Entity,
};

use crate::{measurement::Measurement, peaker::Peaker, CellOption, Height, Table, TableOption};

use super::TableHeightIncrease;

/// A modification for cell/table to increase its height.
///
/// If used for a [`Table`] [`PriorityNone`] is used.
///
/// [`PriorityNone`]: crate::peaker::PriorityNone
#[derive(Debug)]
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

impl<W, R> CellOption<R> for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records + RecordsMut,
{
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let height = self.height.measure(table.get_records(), table.get_config());

        let (count_rows, count_cols) = table.shape();
        for pos in entity.iter(count_rows, count_cols) {
            let records = table.get_records();
            let cell_height = records.count_lines(pos);
            if cell_height >= height {
                continue;
            }

            let content = records.get_text(pos);
            let content = add_lines(content, height - cell_height);
            let ctrl = CfgWidthFunction::from_cfg(table.get_config());
            table.get_records_mut().set(pos, content, &ctrl);
        }

        table.destroy_height_cache();
    }
}

fn add_lines(s: &str, n: usize) -> String {
    let mut text = String::with_capacity(s.len() + n);
    text.push_str(s);
    text.extend(std::iter::repeat('\n').take(n));

    text
}

impl<W, R> TableOption<R> for CellHeightIncrease<W>
where
    W: Measurement<Height>,
    R: Records,
{
    fn change(&mut self, table: &mut Table<R>) {
        TableHeightIncrease::new(self.height.measure(table.get_records(), table.get_config()))
            .change(table)
    }
}
