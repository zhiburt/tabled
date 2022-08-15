//! The module contains a [Records] abstraction of a [`Grid`] trait and its implementators.
//!
//! [`Grid`]: crate::Grid

use crate::{width::WidthFunc, Position};

pub mod cell_info;
pub mod empty;
pub mod vec_records;

#[cfg(feature = "color")]
pub mod tcell;

/// The representaion of data, rows and columns of a [`Grid`].
///
/// [`Grid`]: crate::Grid
pub trait Records {
    /// Returns amount of rows on a grid.
    fn count_rows(&self) -> usize;

    /// Returns amount of columns on a grid.
    fn count_columns(&self) -> usize;

    /// Returns a text of a cell by an index.
    fn get_text(&self, pos: Position) -> &str;

    /// Returns a line of a text of a cell by an index.
    fn get_line(&self, pos: Position, i: usize) -> &str;

    /// Returns an amount of lines of a text of a cell by an index.
    fn count_lines(&self, pos: Position) -> usize;

    /// Returns a width of a text of a cell by an index.
    fn get_width<W>(&self, pos: Position, width_ctrl: W) -> usize
    where
        W: WidthFunc;

    /// Returns a width of line of a text of a cell by an index.
    fn get_line_width<W>(&self, pos: Position, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc;

    /// Prints a prefix of a text of a cell by an index.
    ///
    /// Maybe be usefull in order to emit ANSI sequences.
    fn fmt_text_prefix(&self, f: &mut std::fmt::Formatter<'_>, pos: Position) -> std::fmt::Result;

    /// Prints a suffix of a text of a cell by an index.
    ///
    /// Maybe be usefull in order to emit ANSI sequences.
    fn fmt_text_suffix(&self, f: &mut std::fmt::Formatter<'_>, pos: Position) -> std::fmt::Result;
}

impl<R> Records for &R
where
    R: Records,
{
    fn count_rows(&self) -> usize {
        R::count_rows(self)
    }

    fn count_columns(&self) -> usize {
        R::count_columns(self)
    }

    fn get_text(&self, pos: Position) -> &str {
        R::get_text(self, pos)
    }

    fn get_line(&self, pos: Position, i: usize) -> &str {
        R::get_line(self, pos, i)
    }

    fn count_lines(&self, pos: Position) -> usize {
        R::count_lines(self, pos)
    }

    fn get_width<W>(&self, pos: Position, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        R::get_width(self, pos, width_ctrl)
    }

    fn get_line_width<W>(&self, pos: Position, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        R::get_line_width(self, pos, i, width_ctrl)
    }

    fn fmt_text_prefix(&self, f: &mut std::fmt::Formatter<'_>, pos: Position) -> std::fmt::Result {
        R::fmt_text_prefix(self, f, pos)
    }

    fn fmt_text_suffix(&self, f: &mut std::fmt::Formatter<'_>, pos: Position) -> std::fmt::Result {
        R::fmt_text_suffix(self, f, pos)
    }
}

/// A [`Grid`] representation of a data set which can be modified.
///
/// [`Grid`]: crate::Grid
pub trait RecordsMut<T> {
    /// Sets a text to a given cell by index.
    fn set<W>(&mut self, pos: Position, text: T, width_ctrl: W)
    where
        W: WidthFunc;

    /// Updates a given cell by index.
    ///
    /// Maybe used if width function was changed.
    fn update<W>(&mut self, pos: Position, width_ctrl: W)
    where
        W: WidthFunc;
}

/// A [`Grid`] representation of a data set which can be modified by moving rows/columns around.
///
/// [`Grid`]: crate::Grid
pub trait Resizable {
    /// Swap cells with one another.
    fn swap(&mut self, lhs: Position, rhs: Position);
    /// Swap rows with one another.
    fn swap_row(&mut self, lhs: usize, rhs: usize);
    /// Swap columns with one another.
    fn swap_column(&mut self, lhs: usize, rhs: usize);
    /// Adds a new row to a data set.
    fn push_row(&mut self);
    /// Adds a new column to a data set.
    fn push_column(&mut self);
    /// Removes a row from a data set by index.
    fn remove_row(&mut self, row: usize);
    /// Removes a column from a data set by index.
    fn remove_column(&mut self, column: usize);
    /// Inserts a row to specific by row index.
    fn insert_row(&mut self, row: usize);
}
