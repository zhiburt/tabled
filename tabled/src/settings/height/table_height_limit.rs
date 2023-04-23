use crate::{
    grid::{
        config::ColoredConfig,
        dimension::CompleteDimensionVecRecords,
        records::{ExactRecords, PeekableRecords, Records, RecordsMut},
        util::string::{count_lines, get_lines},
    },
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        Height, TableOption,
    },
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

impl<R, W, P> TableOption<R, CompleteDimensionVecRecords<'static>, ColoredConfig>
    for TableHeightLimit<W, P>
where
    W: Measurement<Height>,
    P: Peaker + Clone,
    R: ExactRecords + PeekableRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(
        self,
        records: &mut R,
        cfg: &mut ColoredConfig,
        dims: &mut CompleteDimensionVecRecords<'static>,
    ) {
        let count_rows = records.count_rows();
        let count_cols = (&*records).count_columns();

        if count_rows == 0 || count_cols == 0 {
            return;
        }

        let height = self.height.measure(&*records, cfg);
        let (total, mut heights) = get_table_height(&*records, cfg);
        if total <= height {
            return;
        }

        decrease_list(&mut heights, total, height, self.priority);

        for (row, &height) in heights.iter().enumerate() {
            for col in 0..count_cols {
                let text = records.get_text((row, col));
                let count_lines = count_lines(text);

                if count_lines <= height {
                    continue;
                }

                let text = limit_lines(text, height);

                records.set((row, col), text);
            }
        }

        let _ = dims.set_heights(heights);
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
