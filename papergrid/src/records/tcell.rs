use std::{
    fmt::Formatter,
    ops::{Deref, DerefMut},
};

use crate::{width::WidthFunc, Color};

use super::vec_records::{Cell, CellMut};

#[derive(Debug, Clone, Default)]
pub struct TCell<T, C> {
    cell: T,
    color: C,
}

impl<T, C> TCell<T, C> {
    pub fn new(cell: T, color: C) -> Self {
        Self { cell, color }
    }

    pub fn get_data(&self) -> &C {
        &self.color
    }

    pub fn get_data_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<T, C> Deref for TCell<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}

impl<T, C> DerefMut for TCell<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cell
    }
}

impl<T, C> Cell for TCell<T, C>
where
    T: Cell,
    C: Color,
{
    fn get_line(&self, i: usize) -> &str {
        self.cell.get_line(i)
    }

    fn count_lines(&self) -> usize {
        self.cell.count_lines()
    }

    fn width<W>(&self, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        self.cell.width(width_ctrl)
    }

    fn line_width<W>(&self, i: usize, width_ctrl: W) -> usize
    where
        W: WidthFunc,
    {
        self.cell.line_width(i, width_ctrl)
    }

    fn fmt_prefix(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.color.fmt_prefix(f)
    }

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.color.fmt_suffix(f)
    }
}

impl<T, C, Q> CellMut<Q> for TCell<T, C>
where
    T: CellMut<Q>,
{
    fn set<W>(&mut self, text: Q, width_ctrl: W)
    where
        W: WidthFunc,
    {
        self.cell.set(text, width_ctrl);
    }

    fn update<W>(&mut self, width_ctrl: W)
    where
        W: WidthFunc,
    {
        self.cell.update(width_ctrl);
    }
}

impl<T, C> AsRef<str> for TCell<T, C>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.cell.as_ref()
    }
}

impl<T, C> From<T> for TCell<T, C>
where
    C: Default,
{
    fn from(cell: T) -> Self {
        Self::new(cell, C::default())
    }
}
