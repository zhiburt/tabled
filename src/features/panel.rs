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

use crate::{width::wrap_text, Table, TableOption};

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

impl<S, R> TableOption<R> for VerticalPanel<S>
where
    S: AsRef<str> + Clone,
    R: Records + RecordsMut + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        if self.pos.1 > count_cols || self.pos.0 > count_rows {
            return;
        }

        move_columns_aside(table, self.pos.1);
        move_column_spans(table, self.pos.1);
        // move_right_borders(table, self.pos.0, self.pos.1);
        // #[cfg(feature = "color")]
        // move_right_border_colors(table, self.pos.0, self.pos.1);

        let text = self.get_text();
        set_text(table, self.pos, text);

        let length = count_rows.checked_sub(self.pos.0).unwrap_or(1);
        table.get_config_mut().set_row_span(self.pos, length);

        table.destroy_width_cache();
        table.destroy_height_cache();
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

impl<S, R> TableOption<R> for HorizontalPanel<S>
where
    S: AsRef<str> + Clone,
    R: Records + RecordsMut + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        if self.pos.0 > count_rows {
            return;
        }

        move_rows_aside(table, self.pos.0);
        move_row_spans(table, self.pos.0);
        // move_lines_aside(table, self.pos.0);
        // move_text_on_lines_aside(table, self.pos.0);
        // move_aside_borders(table, self.pos.0);
        // #[cfg(feature = "color")]
        // move_aside_border_colors(table, self.pos.0);

        set_text(table, self.pos, self.text.as_ref().to_owned());

        let length = count_cols.checked_sub(self.pos.1).unwrap_or(1);
        table.get_config_mut().set_column_span(self.pos, length);

        table.destroy_width_cache();
        table.destroy_height_cache();
    }
}

/// Header inserts a [`Panel`] at the top.
/// See [`Panel`].
#[derive(Debug)]
pub struct Header<S>(S);

impl<S, R> TableOption<R> for Header<S>
where
    S: AsRef<str>,
    R: Records + RecordsMut + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        HorizontalPanel {
            pos: (0, 0),
            text: self.0.as_ref(),
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
    S: AsRef<str> + Clone,
    R: Records + RecordsMut + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        HorizontalPanel {
            pos: (table.shape().0, 0),
            text: self.0.as_ref(),
        }
        .change(table);
    }
}

fn move_rows_aside<R>(table: &mut Table<R>, row: usize)
where
    R: Records + Resizable,
{
    table.get_records_mut().push_row();

    let count_rows = table.get_records().count_rows();
    let shift_count = count_rows - row;
    for i in 0..shift_count {
        let row = count_rows - i;
        table.get_records_mut().swap_row(row, row - 1);
    }
}

fn move_columns_aside<R>(table: &mut Table<R>, column: usize)
where
    R: Records + Resizable,
{
    table.get_records_mut().push_column();

    let count_columns = table.get_records().count_columns();
    let shift_count = count_columns - column;
    for i in 0..shift_count {
        let col = count_columns - i;
        table.get_records_mut().swap_column(col, col - 1);
    }
}

fn move_row_spans<R>(table: &mut Table<R>, target_row: usize)
where
    R: Records,
{
    let spans = table
        .get_config()
        .iter_column_spans(table.shape())
        .collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            table.get_config_mut().set_column_span((row, col), 1);
            table.get_config_mut().set_column_span((row + 1, col), span);
        }
    }

    let spans = table
        .get_config()
        .iter_row_spans(table.shape())
        .collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            table.get_config_mut().set_row_span((row, col), 1);
            table.get_config_mut().set_row_span((row + 1, col), span);
        } else {
            // let span_covers_row = target_row <= row + span;
            // if span_covers_row {
            //     table.get_config_mut().set_row_span((row, col), span + 1);
            // }
        }
    }
}

fn move_column_spans<R>(table: &mut Table<R>, target_column: usize)
where
    R: Records,
{
    let spans = table
        .get_config()
        .iter_column_spans(table.shape())
        .collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if col >= target_column {
            table.get_config_mut().set_column_span((row, col), 1);
            table.get_config_mut().set_column_span((row, col + 1), span);
        } else {
            // let span_covers_column = target_column <= col + span;
            // if span_covers_column {
            //     table.get_config_mut().set_column_span((row, col), span + 1);
            // }
        }
    }

    let spans = table
        .get_config()
        .iter_row_spans(table.shape())
        .collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if col >= target_column {
            table.get_config_mut().set_row_span((row, col), 1);
            table.get_config_mut().set_row_span((row, col + 1), span);
        }
    }
}

fn set_text<R>(table: &mut Table<R>, pos: Position, text: String)
where
    R: RecordsMut,
{
    let ctrl = CfgWidthFunction::from_cfg(table.get_config());
    table.get_records_mut().set(pos, text, ctrl);
}

// fn move_lines_aside<R>(table: &mut Table<R>, row: usize)
// where
//     R: Records,
// {
//     let count_rows = table.get_records().count_rows();
//     if count_rows < 2 {
//         return;
//     }

//     for i in (row..count_rows - 1).rev() {
//         if let Some(line) = table.get_config().get_split_line(i).cloned() {
//             table.get_config_mut().remove_split_line(i);
//             table.get_config_mut().set_split_line(i + 1, line);
//         }
//     }
// }

// fn move_text_on_lines_aside<R>(table: &mut Table<R>, row: usize)
// where
//     R: Records,
// {
//     let count_rows = table.get_records().count_rows();
//     if count_rows < 2 {
//         return;
//     }

//     for i in (row..count_rows - 1).rev() {
//         if let Some(line) = table.get_config_mut().remove_split_line_text(i) {
//             table.get_config_mut().override_split_line(i + 1, line);
//         }
//     }
// }

// fn move_aside_borders<R>(table: &mut Table<R>, row: usize)
// where
//     R: Records,
// {
//     let count_rows = table.get_records().count_rows();
//     if count_rows < 2 {
//         return;
//     }

//     let count_columns = table.get_records().count_columns();

//     for i in (row + 1..count_rows).rev() {
//         for col in 0..count_columns {
//             let mut border = table
//                 .get_config()
//                 .get_border((i - 1, col), (count_rows, count_columns));

//             if border.is_empty() {
//                 continue;
//             }

//             if col > 0 {
//                 // because we delete border and we set borders column by column we need to do this swap.

//                 border.left_top_corner = border.left_bottom_corner;
//                 border.left_bottom_corner = None;
//             }

//             table
//                 .get_config_mut()
//                 .remove_border((i - 1, col), count_columns);
//             table.get_config_mut().set_border((i, col), border);
//         }
//     }
// }

// #[cfg(feature = "color")]
// fn move_aside_border_colors<R>(table: &mut Table<R>, row: usize)
// where
//     R: Records,
// {
//     let count_rows = table.get_records().count_rows();
//     if count_rows < 2 {
//         return;
//     }

//     let count_columns = table.get_records().count_columns();

//     for i in (row + 1..count_rows).rev() {
//         for col in 0..count_columns {
//             let mut border = table
//                 .get_config()
//                 .get_border_color((i - 1, col), (count_rows, count_columns))
//                 .cloned();

//             if border.is_empty() {
//                 continue;
//             }

//             if col > 0 {
//                 // because we delete border and we set borders column by column we need to do this swap.

//                 border.left_top_corner = border.left_bottom_corner;
//                 border.left_bottom_corner = None;
//             }

//             table
//                 .get_config_mut()
//                 .remove_border_color((i - 1, col), (count_rows, count_columns));
//             table.get_config_mut().set_border_color((i, col), border);
//         }
//     }
// }

// fn move_right_borders<R>(table: &mut Table<R>, row: usize, col: usize)
// where
//     R: Records,
// {
//     let count_columns = table.get_records().count_columns();
//     let count_rows = table.get_records().count_rows();
//     if count_columns < 2 {
//         return;
//     }

//     for col in (col + 1..count_columns).rev() {
//         for row in row..count_rows {
//             let mut border = table
//                 .get_config()
//                 .get_border((row, col - 1), (count_rows, count_columns));

//             if border.is_empty() {
//                 continue;
//             }

//             if row > 0 {
//                 // because we delete border and we set borders column by column we need to do this swap.

//                 border.left_top_corner = border.right_top_corner;
//                 border.right_top_corner = None;
//             }

//             table
//                 .get_config_mut()
//                 .remove_border((row, col - 1), count_columns);
//             table.get_config_mut().set_border((row, col), border);
//         }
//     }
// }

// #[cfg(feature = "color")]
// fn move_right_border_colors<R>(table: &mut Table<R>, init_row: usize, col: usize)
// where
//     R: Records,
// {
//     let count_columns = table.get_records().count_columns();
//     let count_rows = table.get_records().count_rows();
//     if count_columns < 2 {
//         return;
//     }

//     for col in (col + 1..count_columns).rev() {
//         for row in init_row..count_rows {
//             let mut border = table
//                 .get_config()
//                 .get_border_color((row, col - 1), (count_rows, count_columns))
//                 .cloned();

//             if border.is_empty() {
//                 continue;
//             }

//             if row > 0 {
//                 // because we delete border and we set borders column by column we need to do this swap.

//                 border.left_top_corner = border.right_top_corner;
//                 border.right_top_corner = None;
//             }

//             table
//                 .get_config_mut()
//                 .remove_border_color((row, col - 1), (count_rows, count_columns));
//             table.get_config_mut().set_border_color((row, col), border);
//         }
//     }
// }
