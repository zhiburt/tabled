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

impl<L, R, I> HorizontalLine<L, R, I> {
    /// Creates a stub horizontal line.
    pub const fn empty() -> Self {
        Self {
            line: Line::new(None, None, None, None),
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

    /// Creates a new horizontal split line.
    pub const fn filled(main: char) -> Self {
        Self::full(main, main, main, main)
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

    /// Set a horizontal character.
    pub const fn horizontal(mut self, c: char) -> HorizontalLine<L, R, I> {
        self.line.main = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Set a vertical intersection character.
    pub const fn intersection(mut self, c: char) -> HorizontalLine<L, R, On> {
        self.line.intersection = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Set a left character.
    pub const fn left(mut self, c: char) -> HorizontalLine<On, R, I> {
        self.line.connector1 = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Set a right character.
    pub const fn right(mut self, c: char) -> HorizontalLine<L, On, I> {
        self.line.connector2 = Some(c);
        HorizontalLine::update(self.line)
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    /// Get a horizontal character.
    pub const fn get_horizontal(&self) -> char {
        match self.line.main {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Get a general structure of line.
    pub const fn into_inner(&self) -> Line {
        self.line
    }
}

impl<L, R> HorizontalLine<L, R, On> {
    /// Set a vertical intersection character.
    pub const fn get_intersection(&self) -> char {
        match self.line.intersection {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a vertical intersection character.
    pub const fn remove_intersection(mut self) -> HorizontalLine<L, R, ()> {
        self.line.intersection = None;
        HorizontalLine::update(self.line)
    }
}

impl<R, I> HorizontalLine<On, R, I> {
    /// Get a left character.
    pub const fn get_left(&self) -> char {
        match self.line.connector1 {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a horizontal left character.
    pub const fn remove_left(mut self) -> HorizontalLine<(), R, I> {
        self.line.connector1 = None;
        HorizontalLine::update(self.line)
    }
}

impl<L, I> HorizontalLine<L, On, I> {
    /// Get a right character.
    pub const fn get_right(&self) -> char {
        match self.line.connector2 {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a horizontal right character.
    pub const fn remove_right(mut self) -> HorizontalLine<I, (), I> {
        self.line.connector2 = None;
        HorizontalLine::update(self.line)
    }
}

impl<L, R, I> From<HorizontalLine<L, R, I>> for Line {
    fn from(value: HorizontalLine<L, R, I>) -> Self {
        value.line
    }
}
