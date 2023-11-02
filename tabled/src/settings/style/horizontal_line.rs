use core::marker::PhantomData;

use crate::settings::style::{Line, On};

/// A horizontal split line which can be used to set a border.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HorizontalLine<L, R, I> {
    line: Line,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
    _intersection: PhantomData<I>,
}

impl HorizontalLine<(), (), ()> {
    /// Creates a new horizontal split line.
    pub const fn new(main: char) -> Self {
        Self {
            line: Line::new(Some(main), None, None, None),
            _left: PhantomData,
            _right: PhantomData,
            _intersection: PhantomData,
        }
    }
}

impl HorizontalLine<On, On, On> {
    /// Creates a new horizontal split line.
    pub const fn full(main: char, left: char, right: char, intersection: char) -> Self {
        Self {
            line: Line::new(Some(main), Some(intersection), Some(left), Some(right)),
            _left: PhantomData,
            _right: PhantomData,
            _intersection: PhantomData,
        }
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    pub(crate) const fn update(line: Line) -> HorizontalLine<L, R, I> {
        Self {
            line,
            _left: PhantomData,
            _right: PhantomData,
            _intersection: PhantomData,
        }
    }

    /// Sets a horizontal character.
    pub const fn horizontal(mut self, c: char) -> HorizontalLine<L, R, I> {
        self.line.main = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: char) -> HorizontalLine<L, R, On> {
        self.line.intersection = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Sets a left character.
    pub const fn left(mut self, c: char) -> HorizontalLine<On, R, I> {
        self.line.connector1 = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Sets a right character.
    pub const fn right(mut self, c: char) -> HorizontalLine<L, On, I> {
        self.line.connector2 = Some(c);
        HorizontalLine::update(self.line)
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    /// Gets a horizontal character.
    pub const fn get_horizontal(&self) -> char {
        match self.line.main {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Gets a general structure of line.
    pub const fn into_inner(&self) -> Line {
        self.line
    }
}

impl<L, R> HorizontalLine<L, R, On> {
    /// Sets a vertical intersection character.
    pub const fn get_intersection(&self) -> char {
        match self.line.intersection {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl<R, I> HorizontalLine<On, R, I> {
    /// Gets a left character.
    pub const fn get_left(&self) -> char {
        match self.line.connector1 {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl<L, I> HorizontalLine<L, On, I> {
    /// Gets a right character.
    pub const fn get_right(&self) -> char {
        match self.line.connector2 {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl<L, R, I> From<HorizontalLine<L, R, I>> for Line {
    fn from(value: HorizontalLine<L, R, I>) -> Self {
        value.line
    }
}
