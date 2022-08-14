use crate::{width::WidthFunc, Position};

pub mod cell_info;
pub mod empty;
pub mod vec_records;

#[cfg(feature = "color")]
pub mod tcell;

pub trait Records {
    fn count_rows(&self) -> usize;
    fn count_columns(&self) -> usize;
    fn get_text(&self, pos: Position) -> &str;
    fn get_line(&self, pos: Position, i: usize) -> &str;
    fn count_lines(&self, pos: Position) -> usize;
    fn get_width<W>(&self, pos: Position, width_ctrl: W) -> usize
    where
        W: WidthFunc;
    fn get_line_width<W>(&self, pos: Position, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc;
    fn fmt_text_prefix(&self, f: &mut std::fmt::Formatter<'_>, pos: Position) -> std::fmt::Result;
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

pub trait RecordsMut<T> {
    fn set<W>(&mut self, pos: Position, text: T, width_ctrl: W)
    where
        W: WidthFunc;
    fn update<W>(&mut self, pos: Position, width_ctrl: W)
    where
        W: WidthFunc;
}

pub trait Resizable {
    fn swap(&mut self, lhs: Position, rhs: Position);
    fn swap_row(&mut self, lhs: usize, rhs: usize);
    fn swap_column(&mut self, lhs: usize, rhs: usize);
    fn push_row(&mut self);
    fn push_column(&mut self);
    fn remove_row(&mut self, row: usize);
    fn remove_column(&mut self, column: usize);
    fn insert_row(&mut self, row: usize);
}
