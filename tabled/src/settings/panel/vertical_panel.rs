use crate::{
    grid::config::GridConfig,
    records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
};

/// A vertical/row span from 0 to a count columns.
#[derive(Debug)]
pub struct VerticalPanel<S> {
    text: S,
    col: usize,
}

impl<S> VerticalPanel<S> {
    /// Creates a new vertical panel.
    pub fn new(col: usize, text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self { text, col }
    }
}

impl<S, R, D> TableOption<R, D> for VerticalPanel<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.col > count_cols {
            return;
        }

        move_columns_aside(records, self.col);
        move_column_spans(cfg, self.col);

        let text = self.text.as_ref().to_owned();
        records.set((0, self.col), text);

        cfg.set_row_span((0, self.col), count_rows);
    }
}

fn move_columns_aside<R: Records + Resizable>(records: &mut R, column: usize) {
    records.push_column();

    let count_columns = records.count_columns();
    let shift_count = count_columns - column;
    for i in 1..shift_count {
        let col = count_columns - i;
        records.swap_column(col, col - 1);
    }
}

fn move_column_spans(cfg: &mut GridConfig, target_column: usize) {
    let spans = cfg.iter_span_columns().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if col >= target_column {
            cfg.set_column_span((row, col), 1);
            cfg.set_column_span((row, col + 1), span);
        }
    }

    let spans = cfg.iter_span_rows().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if col >= target_column {
            cfg.set_row_span((row, col), 1);
            cfg.set_row_span((row, col + 1), span);
        }
    }
}
