use papergrid::records::Records;

use crate::{
    measurement::Measurement,
    peaker::{Peaker, PriorityNone},
    Height, Table, TableOption,
};

use super::get_table_total_height2;

/// A modification of a table to increase the table height.
#[derive(Debug, Clone)]
pub struct TableHeightIncrease<W = usize, P = PriorityNone> {
    height: W,
    priority: P,
}

impl<W> TableHeightIncrease<W, PriorityNone> {
    /// Creates a new object.
    pub fn new(height: W) -> Self
    where
        W: Measurement<Height>,
    {
        Self {
            height,
            priority: PriorityNone::default(),
        }
    }

    /// Sets a different priority logic.
    pub fn priority<P>(self) -> TableHeightIncrease<W, P>
    where
        P: Peaker,
    {
        TableHeightIncrease {
            priority: P::create(),
            height: self.height,
        }
    }
}

impl<R, W, P> TableOption<R> for TableHeightIncrease<W, P>
where
    R: Records,
    W: Measurement<Height>,
    P: Peaker + Clone,
{
    fn change(&mut self, table: &mut Table<R>) {
        if table.is_empty() {
            return;
        }

        let height = self.height.measure(table.get_records(), table.get_config());
        let (total, heights) = get_table_total_height2(table.get_records(), table.get_config());
        if total >= height {
            return;
        }

        increase_total_height(table, heights, total, height, self.priority.clone());
    }
}

fn increase_total_height<P, R>(
    table: &mut Table<R>,
    mut list: Vec<usize>,
    total: usize,
    expected: usize,
    priority: P,
) where
    P: Peaker,
    R: Records,
{
    get_increase_list(&mut list, expected, total, priority);
    table.cache_height(list);
}

fn get_increase_list<P>(list: &mut [usize], total: usize, mut value: usize, mut peaker: P)
where
    P: Peaker,
{
    while value != total {
        let col = match peaker.peak(&[], list) {
            Some(col) => col,
            None => break,
        };

        list[col] += 1;
        value += 1;
    }
}
