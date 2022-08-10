use std::borrow::Cow;

use crate::{width::WidthFunc, Position};

pub mod empty;
pub mod records_info;
pub mod small_records;

#[cfg(feature = "color")]
pub mod records_info_colored;

pub trait Records {
    type Cell;

    /// Returns row length and a count of columns.
    ///
    /// It's suppesed to be safe to use the constrains in order to get a Cell
    fn size(&self) -> (usize, usize);
    fn get(&self, pos: Position) -> Self::Cell;
    fn get_text(&self, pos: Position) -> &str;
}

impl<R> Records for &R
where
    R: Records,
{
    type Cell = R::Cell;

    fn size(&self) -> (usize, usize) {
        R::size(self)
    }

    fn get(&self, pos: Position) -> Self::Cell {
        R::get(self, pos)
    }

    fn get_text(&self, pos: Position) -> &str {
        R::get_text(self, pos)
    }
}

impl<R> Records for &mut R
where
    R: Records,
{
    type Cell = R::Cell;

    fn size(&self) -> (usize, usize) {
        R::size(self)
    }

    fn get(&self, pos: Position) -> Self::Cell {
        R::get(self, pos)
    }

    fn get_text(&self, pos: Position) -> &str {
        R::get_text(self, pos)
    }
}

pub trait RecordsMut {
    fn set_text<W>(&mut self, pos: Position, text: String, width: W)
    where
        W: WidthFunc;
    fn update<W>(&mut self, pos: Position, width: W)
    where
        W: WidthFunc;
}

impl<'a, R> RecordsMut for &'a mut R
where
    R: RecordsMut,
{
    fn set_text<W>(&mut self, pos: Position, text: String, width: W)
    where
        W: WidthFunc,
    {
        R::set_text(self, pos, text, width)
    }

    fn update<W>(&mut self, pos: Position, width: W)
    where
        W: WidthFunc,
    {
        R::update(self, pos, width)
    }
}

pub trait Resizable {
    fn swap(&mut self, lhs: Position, rhs: Position);
    fn swap_row(&mut self, lhs: usize, rhs: usize);
    fn swap_column(&mut self, lhs: usize, rhs: usize);
    fn push_row(&mut self);
    fn push_column(&mut self);
    fn remove_row(&mut self, row: usize);
    fn remove_column(&mut self, column: usize);
}

impl<'a, R> Resizable for &'a mut R
where
    R: Resizable,
{
    fn swap(&mut self, lhs: Position, rhs: Position) {
        R::swap(self, lhs, rhs)
    }

    fn swap_column(&mut self, lhs: usize, rhs: usize) {
        R::swap_column(self, lhs, rhs)
    }

    fn swap_row(&mut self, lhs: usize, rhs: usize) {
        R::swap_row(self, lhs, rhs)
    }

    fn push_row(&mut self) {
        R::push_row(self)
    }

    fn push_column(&mut self) {
        R::push_column(self)
    }

    fn remove_row(&mut self, row: usize) {
        R::remove_row(self, row)
    }

    fn remove_column(&mut self, column: usize) {
        R::remove_column(self, column)
    }
}

pub trait Cell {
    type Text;
    type Lines;

    fn lines(&self) -> Self::Lines;
    fn get_line(&self, i: usize) -> Option<Self::Text>;
    fn count_lines(&self) -> usize;
    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc;
}

impl<C> Cell for &C
where
    C: Cell,
{
    type Text = C::Text;
    type Lines = C::Lines;

    fn lines(&self) -> Self::Lines {
        C::lines(self)
    }

    fn get_line(&self, i: usize) -> Option<Self::Text> {
        C::get_line(self, i)
    }

    fn count_lines(&self) -> usize {
        C::count_lines(self)
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        C::width(self, width)
    }
}

pub trait Text {
    fn as_str(&self) -> &str;
    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc;
}

impl<T> Text for &T
where
    T: Text,
{
    fn as_str(&self) -> &str {
        T::as_str(self)
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        T::width(self, width)
    }
}

impl Text for &str {
    fn as_str(&self) -> &str {
        self
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        width.width(self)
    }
}

impl Text for str {
    fn as_str(&self) -> &str {
        self
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        width.width(self)
    }
}

impl Text for Cow<'_, str> {
    fn as_str(&self) -> &str {
        self
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        width.width(self)
    }
}
