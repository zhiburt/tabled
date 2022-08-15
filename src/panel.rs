//! This module contains primitivies to create a spread row.
//! Ultimately it is a cell with a span set to a number of columns on the [`Table`].
//!
//! You can use a [`Span`] to set a custom span.
//!
//! # Example
//!
//! ```
//! use tabled::{object::Cell, Panel, Modify, TableIteratorExt, Span};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Panel("Tabled Releases", 0))
//!     .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+-----+-----+-----+\n",
//!         "| Tabled Releases |\n",
//!         "+-----+-----+-----+\n",
//!         "| 0   | 1   | 2   |\n",
//!         "+-----+-----+-----+\n",
//!         "| 1         | 3   |\n",
//!         "+-----+-----+-----+\n",
//!         "| 4   | 5   | 6   |\n",
//!         "+-----+-----+-----+",
//!     )
//! )
//! ```
//!
//! [`Table`]: crate::Table
//! [`Span`]: crate::Span

use papergrid::{
    records::{Records, RecordsMut, Resizable},
    width::CfgWidthFunction,
};

use crate::{Table, TableOption};

/// Panel allows to add a Row which has 1 continues Cell to a [`Table`].
///
/// See `examples/panel.rs`.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Panel<S>(pub S, pub usize)
where
    S: AsRef<str>;

impl<S, R> TableOption<R> for Panel<S>
where
    S: AsRef<str>,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        if self.1 > count_rows {
            return;
        }

        table.get_records_mut().push_row();

        let shift_count = count_rows - self.1;
        for i in 0..shift_count {
            let row = count_rows - i;
            table.get_records_mut().swap_row(row, row - 1);
        }

        // move existing spans
        let spans = table.get_config().iter_column_spans().collect::<Vec<_>>();
        for ((row, col), span) in spans {
            if row >= self.1 {
                table.get_config_mut().set_span((row, col), 1);
                table.get_config_mut().set_span((row + 1, col), span);
            }
        }

        let ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let pos = (self.1, 0);
        let text = self.0.as_ref().to_owned();
        table.get_records_mut().set(pos, text, ctrl);
        table.get_config_mut().set_span(pos, count_cols);
    }
}

/// Header inserts a [`Panel`] at the top.
/// See [`Panel`].
#[derive(Debug)]
pub struct Header<S: AsRef<str>>(pub S);

impl<S, R> TableOption<R> for Header<S>
where
    S: AsRef<str>,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        Panel(self.0.as_ref(), 0).change(table);
    }
}

/// Footer renders a [`Panel`] at the bottom.
/// See [`Panel`].
#[derive(Debug)]
pub struct Footer<S: AsRef<str>>(pub S);

impl<S, R> TableOption<R> for Footer<S>
where
    S: AsRef<str>,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        Panel(self.0.as_ref(), table.shape().0).change(table);
    }
}
