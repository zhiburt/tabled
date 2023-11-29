use core::marker::PhantomData;

use crate::grid::config::HorizontalLine;
use crate::grid::config::VerticalLine as Line;
use crate::settings::style::On;
use crate::settings::Style;

/// A vertical split line which can be used to set a border.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerticalLine<T, B, I> {
    line: Line<char>,
    _top: PhantomData<T>,
    _bottom: PhantomData<B>,
    _intersection: PhantomData<I>,
}

impl VerticalLine<(), (), ()> {
    /// Creates a new vertical split line.
    pub const fn new(main: char) -> Self {
        Self::update(Line::new(Some(main), None, None, None))
    }
}

impl<T, B, I> VerticalLine<T, B, I> {
    /// Creates a stub horizontal line.
    pub const fn empty() -> Self {
        Self::update(Line::empty())
    }

    /// Fetches vertical line from a style.
    pub const fn inherit<L, R, const HSIZE: usize, const VSIZE: usize>(
        style: Style<T, B, L, R, I, On, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.vertical,
            borders.intersection,
            borders.top_intersection,
            borders.bottom_intersection,
        );

        Self::update(line)
    }

    /// Fetches left vertical line from a style.
    pub const fn inherit_left<R, V, const HSIZE: usize, const VSIZE: usize>(
        style: Style<T, B, On, R, I, V, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.left,
            borders.left_intersection,
            borders.top_left,
            borders.bottom_left,
        );

        Self::update(line)
    }

    /// Fetches right vertical line from a style.
    pub const fn inherit_right<L, V, const HSIZE: usize, const VSIZE: usize>(
        style: Style<T, B, L, On, I, V, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.right,
            borders.right_intersection,
            borders.top_right,
            borders.bottom_right,
        );

        Self::update(line)
    }
}

impl VerticalLine<On, On, On> {
    /// Creates a new vertical split line.
    pub const fn full(main: char, intersection: char, top: char, bottom: char) -> Self {
        Self::update(Line::new(
            Some(main),
            Some(intersection),
            Some(top),
            Some(bottom),
        ))
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
        self.line.top = Some(c);
        VerticalLine::update(self.line)
    }

    /// Set a bottom character.
    pub const fn bottom(mut self, c: char) -> VerticalLine<T, On, I> {
        self.line.bottom = Some(c);
        VerticalLine::update(self.line)
    }
}

impl<T, B, I> VerticalLine<T, B, I> {
    pub(crate) const fn update(line: Line<char>) -> VerticalLine<T, B, I> {
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
    pub const fn into_inner(&self) -> Line<char> {
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
        opt_get(self.line.top)
    }

    /// Remove a vertical top character.
    pub const fn remove_top(mut self) -> VerticalLine<(), B, I> {
        self.line.top = None;
        VerticalLine::update(self.line)
    }
}

impl<T, I> VerticalLine<T, On, I> {
    /// Get a bottom character.
    pub const fn get_bottom(&self) -> char {
        opt_get(self.line.bottom)
    }

    /// Remove a vertical bottom character.
    pub const fn remove_bottom(mut self) -> VerticalLine<T, (), I> {
        self.line.bottom = None;
        VerticalLine::update(self.line)
    }
}

impl<T, B, I> From<VerticalLine<T, B, I>> for Line<char> {
    fn from(value: VerticalLine<T, B, I>) -> Self {
        value.line
    }
}

impl<T, B, I> From<VerticalLine<T, B, I>> for HorizontalLine<char> {
    fn from(value: VerticalLine<T, B, I>) -> Self {
        HorizontalLine::new(
            value.line.main,
            value.line.intersection,
            value.line.top,
            value.line.bottom,
        )
    }
}

impl<T, B, I> From<Line<char>> for VerticalLine<T, B, I> {
    fn from(value: Line<char>) -> Self {
        let mut line = Self::empty();
        line.line = value;
        line
    }
}

const fn opt_get(opt: Option<char>) -> char {
    match opt {
        Some(value) => value,
        None => unreachable!(),
    }
}
