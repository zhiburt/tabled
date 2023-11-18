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

impl<L, R, I> VerticalLine<L, R, I> {
    /// Creates a stub horizontal line.
    pub const fn empty() -> Self {
        Self {
            line: Line::new(None, None, None, None),
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

    /// Creates a new vertical split line.
    pub const fn filled(main: char) -> Self {
        Self::full(main, main, main, main)
    }
}

impl<T, B, I> VerticalLine<T, B, I> {
    /// Set a vertical character.
    pub const fn vertical(mut self, c: char) -> VerticalLine<T, B, I> {
        self.line.main = Some(c);
        VerticalLine::update(self.line)
    }

    /// Set a vertical intersection character.
    pub const fn intersection(mut self, c: char) -> VerticalLine<T, B, On> {
        self.line.intersection = Some(c);
        VerticalLine::update(self.line)
    }

    /// Set a top character.
    pub const fn top(mut self, c: char) -> VerticalLine<On, B, I> {
        self.line.connector1 = Some(c);
        VerticalLine::update(self.line)
    }

    /// Set a bottom character.
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

    /// Get a vertical character.
    pub const fn get_vertical(&self) -> char {
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

impl<T, B> VerticalLine<T, B, On> {
    /// Set a horizontal intersection character.
    pub const fn get_intersection(&self) -> char {
        match self.line.intersection {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a horizontal intersection character.
    pub const fn remove_intersection(mut self) -> VerticalLine<T, B, ()> {
        self.line.intersection = None;
        VerticalLine::update(self.line)
    }
}

impl<B, I> VerticalLine<On, B, I> {
    /// Get a top character.
    pub const fn get_top(&self) -> char {
        match self.line.connector1 {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a vertical top character.
    pub const fn remove_top(mut self) -> VerticalLine<(), B, I> {
        self.line.connector1 = None;
        VerticalLine::update(self.line)
    }
}

impl<T, I> VerticalLine<T, On, I> {
    /// Get a bottom character.
    pub const fn get_bottom(&self) -> char {
        match self.line.connector2 {
            Some(c) => c,
            None => unreachable!(),
        }
    }

    /// Remove a vertical bottom character.
    pub const fn remove_bottom(mut self) -> VerticalLine<T, (), I> {
        self.line.connector2 = None;
        VerticalLine::update(self.line)
    }
}

impl<T, B, I> From<VerticalLine<T, B, I>> for Line {
    fn from(value: VerticalLine<T, B, I>) -> Self {
        value.line
    }
}
