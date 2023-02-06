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

use crate::{
    grid::config::{GridConfig, Position},
    records::{self, ExactRecords, Records, RecordsMut, Resizable},
    width::wrap_text,
    Table, TableOption,
};

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
        T: AsRef<str> + Clone,
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

    fn get_text(&self) -> String
    where
        S: AsRef<str>,
    {
        let text = if self.text_width > 0 {
            wrap_text(self.text.as_ref(), self.text_width, false)
        } else {
            self.text.as_ref().to_owned()
        };
        text
    }
}

impl<S, R, D> TableOption<R, D> for VerticalPanel<S>
where
    S: AsRef<str> + Clone,
    R: Records + ExactRecords + Resizable + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.pos.1 > count_cols || self.pos.0 > count_rows {
            return;
        }

        move_columns_aside(records, self.pos.1);
        move_column_spans(cfg, self.pos.1);

        let text = self.get_text();
        records.set(self.pos, text);

        let length = count_rows.checked_sub(self.pos.0).unwrap_or(1);
        cfg.set_row_span(self.pos, length);
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
        T: AsRef<str> + Clone,
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

impl<S, R, D> TableOption<R, D> for HorizontalPanel<S>
where
    S: AsRef<str> + Clone,
    R: Records + ExactRecords + Resizable + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.pos.0 > count_rows {
            return;
        }

        move_rows_aside(records, self.pos.0);
        move_row_spans(cfg, self.pos.0);

        let text = self.text.as_ref().to_owned();
        records.set(self.pos, text);

        let length = count_cols.checked_sub(self.pos.1).unwrap_or(1);
        cfg.set_column_span(self.pos, length);
    }
}

/// Header inserts a [`Panel`] at the top.
/// See [`Panel`].
#[derive(Debug)]
pub struct Header<S>(S);

impl<S, R, D> TableOption<R, D> for Header<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        HorizontalPanel {
            pos: (0, 0),
            text: self.0.as_ref(),
        }
        .change(records, cfg, dimension);
    }
}

/// Footer renders a [`Panel`] at the bottom.
/// See [`Panel`].
#[derive(Debug)]
pub struct Footer<S>(S);

impl<S, R, D> TableOption<R, D> for Footer<S>
where
    S: AsRef<str> + Clone,
    R: Records + ExactRecords + Resizable + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        HorizontalPanel {
            pos: (records.count_rows(), 0),
            text: self.0.as_ref(),
        }
        .change(records, cfg, dimension);
    }
}

fn move_rows_aside<R>(records: &mut R, row: usize)
where
    R: ExactRecords + Resizable,
{
    records.push_row();

    let count_rows = records.count_rows();

    let shift_count = count_rows - row;
    for i in 1..shift_count {
        let row = count_rows - i;
        records.swap_row(row, row - 1);
    }
}

fn move_columns_aside<R>(records: &mut R, column: usize)
where
    R: Records + Resizable,
{
    records.push_column();

    let count_columns = records.count_columns();
    let shift_count = count_columns - column;
    for i in 1..shift_count {
        let col = count_columns - i;
        records.swap_column(col, col - 1);
    }
}

fn move_row_spans(cfg: &mut GridConfig, target_row: usize) {
    let spans = cfg.iter_span_columns().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            cfg.set_column_span((row, col), 1);
            cfg.set_column_span((row + 1, col), span);
        }
    }

    let spans = cfg.iter_span_rows().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            cfg.set_row_span((row, col), 1);
            cfg.set_row_span((row + 1, col), span);
        }
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
