use papergrid::{records::Records, GridConfig};

use crate::{
    measurement::Measurement,
    peaker::{Peaker, PriorityNone},
    records::ExactRecords,
    table::TableDimension,
    Height, TableOption,
};

use super::util::get_table_height;

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
        W: Measurement<Height>,
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

impl<R, W, P> TableOption<R, TableDimension<'static>> for TableHeightLimit<W, P>
where
    W: Measurement<Height>,
    P: Peaker + Clone,
    for<'a> &'a R: Records + ExactRecords,
{
    fn change(
        &mut self,
        records: &mut R,
        cfg: &mut GridConfig,
        dims: &mut TableDimension<'static>,
    ) {
        let records = &*records;

        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let height = self.height.measure(records, cfg);
        let (total, mut heights) = get_table_height(records, cfg);
        if total <= height {
            return;
        }

        decrease_list(&mut heights, total, height, self.priority.clone());

        dims.set_heights(heights);
    }
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
