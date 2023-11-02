use core::marker::PhantomData;

use crate::settings::style::{Line, On};

/// A vertical split line which can be used to set a border.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerticalLine<T, B, I> {
    line: Line,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _intersection: PhantomData<I>,
}

impl VerticalLine<(), (), ()> {
    /// Creates a new vertical split line.
    pub const fn new(main: char) -> Self {
        Self {
            line: Line::new(Some(main), None, None, None),
            _top: PhantomData,
            _bottom: PhantomData,
            _intersection: PhantomData,
        }
    }
}

impl VerticalLine<On, On, On> {
    /// Creates a new vertical split line.
    pub const fn full(main: char, top: char, bottom: char, intersection: char) -> Self {
        Self {
            line: Line::new(Some(main), Some(intersection), Some(top), Some(bottom)),
            _top: PhantomData,
            _bottom: PhantomData,
            _intersection: PhantomData,
        }
    }
}

impl<T, B, I> VerticalLine<T, B, I> {
    /// Sets a vertical character.
    pub const fn vertical(mut self, c: char) -> VerticalLine<T, B, I> {
        self.line.main = Some(c);
        VerticalLine::update(self.line)
    }

    /// Sets a vertical intersection character.
    pub const fn intersection(mut self, c: char) -> VerticalLine<T, B, On> {
        self.line.intersection = Some(c);
        VerticalLine::update(self.line)
    }

    /// Sets a top character.
    pub const fn top(mut self, c: char) -> VerticalLine<On, B, I> {
        self.line.connector1 = Some(c);
        VerticalLine::update(self.line)
    }

    /// Sets a bottom character.
    pub const fn bottom(mut self, c: char) -> VerticalLine<T, On, I> {
        self.line.connector2 = Some(c);
        VerticalLine::update(self.line)
    }
}

impl<T, B, I> VerticalLine<T, B, I> {
    pub(crate) const fn update(line: Line) -> VerticalLine<T, B, I> {
        Self {
            line,
            _top: PhantomData,
            _bottom: PhantomData,
            _intersection: PhantomData,
        }
    }

    /// Gets a vertical character.
    pub const fn get_vertical(&self) -> char {
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

impl<T, B> VerticalLine<T, B, On> {
    /// Sets a horizontal intersection character.
    pub const fn get_intersection(&self) -> char {
        match self.line.intersection {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl<B, I> VerticalLine<On, B, I> {
    /// Gets a top character.
    pub const fn get_top(&self) -> char {
        match self.line.connector1 {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}

impl<T, I> VerticalLine<T, On, I> {
    /// Gets a bottom character.
    pub const fn get_bottom(&self) -> char {
        match self.line.connector2 {
            Some(c) => c,
            None => unreachable!(),
        }
    }
}
