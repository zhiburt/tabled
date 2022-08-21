//! This module contains primitivies to create a spread row.
//! Ultimately it is a cell with a span set to a number of columns on the [`Table`].
//!
//! You can use a [`Span`] to set a custom span.
//!
//! # Example
//!
//! ```
//! use tabled::{TableIteratorExt, Panel};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Panel::vertical(1).text("Split").text_width(1))
//!     .with(Panel::header("Numbers"))
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+---+\n",
//!         "| Numbers       |\n",
//!         "+---+---+---+---+\n",
//!         "| 0 | S | 1 | 2 |\n",
//!         "+---+ p +---+---+\n",
//!         "| 1 | l | 2 | 3 |\n",
//!         "+---+ i +---+---+\n",
//!         "| 4 | t | 5 | 6 |\n",
//!         "+---+---+---+---+",
//!     )
//! )
//! ```
//!
//! [`Table`]: crate::Table
//! [`Span`]: crate::Span

use papergrid::{
    records::{Records, RecordsMut, Resizable},
    width::CfgWidthFunction,
    Position,
};

use crate::{wrap::wrap_text, Table, TableOption};

/// Panel allows to add a Row which has 1 continues Cell to a [`Table`].
///
/// See `examples/panel.rs`.
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Panel;

impl Panel {
    /// Creates an empty vertical row at given index.
    ///
    /// ```
    /// use tabled::{Panel, TableIteratorExt};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = data.table()
    ///     .with(Panel::vertical(1).text("Tabled Releases"))
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---+-----------------+---+---+\n",
    ///         "| 0 | Tabled Releases | 1 | 2 |\n",
    ///         "+---+                 +---+---+\n",
    ///         "| 1 |                 | 2 | 3 |\n",
    ///         "+---+                 +---+---+\n",
    ///         "| 4 |                 | 5 | 6 |\n",
    ///         "+---+-----------------+---+---+",
    ///     )
    /// )
    /// ```
    pub fn vertical(column: usize) -> VerticalPanel<&'static str> {
        VerticalPanel {
            pos: (0, column),
            text: "",
            text_width: 0,
        }
    }

    /// Creates an empty horizontal row at given index.
    ///
    /// ```
    /// use tabled::{Panel, TableIteratorExt};
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    ///
    /// let table = data.table()
    ///     .with(Panel::vertical(1))
    ///     .to_string();
    ///
    /// println!("{}", table);
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "+---+--+---+---+\n",
    ///         "| 0 |  | 1 | 2 |\n",
    ///         "+---+  +---+---+\n",
    ///         "| 1 |  | 2 | 3 |\n",
    ///         "+---+  +---+---+\n",
    ///         "| 4 |  | 5 | 6 |\n",
    ///         "+---+--+---+---+",
    ///     )
    /// )
    /// ```
    pub fn horizontal(row: usize) -> HorizontalPanel<&'static str> {
        HorizontalPanel {
            text: "",
            pos: (row, 0),
        }
    }

    /// Creates an horizontal row at first row.
    pub fn header<S>(text: S) -> Header<S> {
        Header(text)
    }

    /// Creates an horizontal row at last row.
    pub fn footer<S>(text: S) -> Footer<S> {
        Footer(text)
    }
}

/// A vertical/row span from 0 to a count columns.
#[derive(Debug)]
pub struct VerticalPanel<S> {
    text: S,
    pos: Position,
    text_width: usize,
}

impl<S> VerticalPanel<S> {
    /// Set a text for a panel.
    pub fn text<T>(self, text: T) -> VerticalPanel<T>
    where
        T: Into<String> + Clone,
    {
        VerticalPanel {
            pos: self.pos,
            text_width: self.text_width,
            text,
        }
    }

    /// Changes the default row=0 to a custom row.
    /// So the panel will be limited from row to count rows.
    pub fn row(mut self, row: usize) -> Self {
        self.pos.0 = row;
        self
    }

    /// Define a maximum width of text.
    pub fn text_width(mut self, width: usize) -> Self {
        self.text_width = width;
        self
    }
}

impl<S, R> TableOption<R> for VerticalPanel<S>
where
    S: Into<String> + Clone,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        if self.pos.1 > count_cols || self.pos.0 > count_rows {
            return;
        }

        table.get_records_mut().push_column();

        let shift_count = count_cols - self.pos.1;
        for i in 0..shift_count {
            let col = count_cols - i;
            table.get_records_mut().swap_column(col, col - 1);
        }

        // move existing spans
        {
            let spans = table
                .get_config()
                .iter_column_spans(table.shape())
                .collect::<Vec<_>>();
            for ((row, col), span) in spans {
                if col >= self.pos.1 {
                    table.get_config_mut().set_column_span((row, col), 1);
                    table.get_config_mut().set_column_span((row, col + 1), span);
                }
            }

            let spans = table
                .get_config()
                .iter_row_spans(table.shape())
                .collect::<Vec<_>>();
            for ((row, col), span) in spans {
                if col >= self.pos.1 {
                    table.get_config_mut().set_row_span((row, col), 1);
                    table.get_config_mut().set_row_span((row, col + 1), span);
                }
            }
        }

        let ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let mut text = self.text.clone().into();
        if self.text_width > 0 {
            text = wrap_text(&text, self.text_width, false);
        }

        table.get_records_mut().set(self.pos, text, ctrl);
        let length = count_rows.checked_sub(self.pos.0).unwrap_or(1);
        table.get_config_mut().set_row_span(self.pos, length);
    }
}

/// A horizontal/column span from 0 to a count rows.
#[derive(Debug)]
pub struct HorizontalPanel<S> {
    text: S,
    pos: Position,
}

impl<S> HorizontalPanel<S> {
    /// Sets a text value.
    pub fn text<T>(self, text: T) -> HorizontalPanel<T>
    where
        T: Into<String> + Clone,
    {
        HorizontalPanel {
            pos: self.pos,
            text,
        }
    }

    pub fn column(mut self, column: usize) -> Self {
        self.pos.1 = column;
        self
    }
}

impl<S, R> TableOption<R> for HorizontalPanel<S>
where
    S: Into<String> + Clone,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        if self.pos.0 > count_rows {
            return;
        }

        table.get_records_mut().push_row();

        let shift_count = count_rows - self.pos.0;
        for i in 0..shift_count {
            let row = count_rows - i;
            table.get_records_mut().swap_row(row, row - 1);
        }

        {
            // move existing spans

            let spans = table
                .get_config()
                .iter_column_spans(table.shape())
                .collect::<Vec<_>>();
            for ((row, col), span) in spans {
                if row >= self.pos.0 {
                    table.get_config_mut().set_column_span((row, col), 1);
                    table.get_config_mut().set_column_span((row + 1, col), span);
                }
            }
            let spans = table
                .get_config()
                .iter_row_spans(table.shape())
                .collect::<Vec<_>>();
            for ((row, col), span) in spans {
                if row >= self.pos.0 {
                    table.get_config_mut().set_row_span((row, col), 1);
                    table.get_config_mut().set_row_span((row + 1, col), span);
                }
            }
        }

        let ctrl = CfgWidthFunction::from_cfg(table.get_config());
        let text = self.text.clone().into();

        table.get_records_mut().set(self.pos, text, ctrl);
        let length = count_cols.checked_sub(self.pos.1).unwrap_or(1);
        table.get_config_mut().set_column_span(self.pos, length);
    }
}

/// Header inserts a [`Panel`] at the top.
/// See [`Panel`].
#[derive(Debug)]
pub struct Header<S>(S);

impl<S, R> TableOption<R> for Header<S>
where
    S: Into<String> + Clone,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        HorizontalPanel {
            pos: (0, 0),
            text: self.0.clone().into(),
        }
        .change(table);
    }
}

/// Footer renders a [`Panel`] at the bottom.
/// See [`Panel`].
#[derive(Debug)]
pub struct Footer<S>(S);

impl<S, R> TableOption<R> for Footer<S>
where
    S: Into<String> + Clone,
    R: Records + RecordsMut<String> + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        HorizontalPanel {
            pos: (table.shape().0, 0),
            text: self.0.clone().into(),
        }
        .change(table);
    }
}
