use papergrid::records::Records;

use crate::{
    measurment::Measurment,
    peaker::{Peaker, PriorityNone},
    Height, Table, TableOption,
};

use super::get_table_total_height2;

/// A modification of a table to decrease the table height.
#[derive(Debug)]
pub struct TableHeightLimit<W = usize, P = PriorityNone> {
    height: W,
    priority: P,
}

impl<W> TableHeightLimit<W, PriorityNone> {
    /// Creates a new object.
    pub fn new(height: W) -> Self
    where
        W: Measurment<Height>,
    {
        Self {
            height,
            priority: PriorityNone::default(),
        }
    }

    /// Sets a different priority logic.
    pub fn priority<P>(self) -> TableHeightLimit<W, P>
    where
        P: Peaker,
    {
        TableHeightLimit {
            priority: P::create(),
            height: self.height,
        }
    }
}

impl<R, W, P> TableOption<R> for TableHeightLimit<W, P>
where
    R: Records,
    W: Measurment<Height>,
    P: Peaker + Clone,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let height = self.height.measure(table.get_records(), table.get_config());
        let (total, heights) = get_table_total_height2(table.get_records(), table.get_config());
        if total <= height {
            return;
        }

        decrease_total_height(table, heights, total, height, self.priority.clone());
    }
}

fn decrease_total_height<P, R>(
    table: &mut Table<R>,
    mut list: Vec<usize>,
    total: usize,
    expected: usize,
    priority: P,
) where
    P: Peaker,
    R: Records,
{
    decrease_list(&mut list, total, expected, priority);
    table.cache_height(list);
    table.destroy_width_cache();
}

fn decrease_list<P>(list: &mut [usize], total: usize, mut value: usize, mut peaker: P)
where
    P: Peaker,
{
    while value != total {
        let p = peaker.peak(&[], list);
        let row = match p {
            Some(row) => row,
            None => break,
        };

        list[row] -= 1;
        value += 1;
    }
}
