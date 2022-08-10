use std::{borrow::Cow, fmt};

use crate::{
    records::{Cell, Records},
    util::get_lines,
    width::WidthFunc,
    Position,
};

#[derive(Debug, Clone)]
pub struct RecordsSmall<'a, T> {
    data: &'a [T],
}

impl<'a, T> RecordsSmall<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self { data }
    }
}

impl<'a, T, S> Records for RecordsSmall<'a, T>
where
    &'a T: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    type Cell = RecordsSmallCell<'a>;

    fn size(&self) -> (usize, usize) {
        let rows = self.data.len();

        let mut cols = 0;
        for row in self.data {
            cols = std::cmp::max(cols, row.into_iter().count());
        }

        (rows, cols)
    }

    fn get(&self, pos: Position) -> Self::Cell {
        let text = &self.data[pos.0].into_iter().nth(pos.1).unwrap().as_ref();
        RecordsSmallCell { text }
    }

    fn get_text(&self, (row, col): Position) -> &str {
        self.data[row].into_iter().nth(col).unwrap().as_ref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RecordsSmallCell<'a> {
    text: &'a str,
}

impl<'a> Cell for RecordsSmallCell<'a> {
    type Text = Cow<'a, str>;
    type Lines = std::vec::IntoIter<Self::Text>;

    fn lines(&self) -> Self::Lines {
        get_lines(self.text).collect::<Vec<_>>().into_iter()
    }

    fn get_line(&self, i: usize) -> Option<Self::Text> {
        get_lines(self.text).nth(i)
    }

    fn count_lines(&self) -> usize {
        get_lines(self.text).count()
    }

    fn width<W>(&self, width: W) -> usize
    where
        W: WidthFunc,
    {
        get_lines(self.text)
            .map(|l| width.width(&l))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(feature = "color")]
impl crate::Color for RecordsSmallCell<'_> {
    fn fmt_prefix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }

    fn fmt_suffix(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
