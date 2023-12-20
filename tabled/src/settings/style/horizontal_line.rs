use core::marker::PhantomData;

use crate::grid::config::HorizontalLine as Line;
use crate::grid::config::VerticalLine;
use crate::settings::style::On;
use crate::settings::Style;

/// A horizontal split line which can be used to set a border.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HorizontalLine<L, R, I> {
    line: Line<char>,
    _left: PhantomData<L>,
    _right: PhantomData<R>,
    _intersection: PhantomData<I>,
}

impl HorizontalLine<(), (), ()> {
    /// Creates a new horizontal split line.
    pub const fn new(main: char) -> Self {
        Self::update(Line::new(Some(main), None, None, None))
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    /// Creates a stub horizontal line.
    pub const fn empty() -> Self {
        Self::update(Line::new(None, None, None, None))
    }

    /// Fetches vertical line from a style.
    pub const fn inherit<T, B, const HSIZE: usize, const VSIZE: usize>(
        style: Style<T, B, L, R, On, I, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.horizontal,
            borders.intersection,
            borders.left_intersection,
            borders.right_intersection,
        );

        Self::update(line)
    }

    /// Fetches left vertical line from a style.
    pub const fn inherit_top<T, B, H, V, const HSIZE: usize, const VSIZE: usize>(
        style: Style<On, B, L, R, H, I, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.top,
            borders.top_intersection,
            borders.top_left,
            borders.top_right,
        );

        Self::update(line)
    }

    /// Fetches right vertical line from a style.
    pub const fn inherit_bottom<T, B, H, V, const HSIZE: usize, const VSIZE: usize>(
        style: Style<T, On, L, R, I, V, HSIZE, VSIZE>,
    ) -> Self {
        let borders = style.get_borders();
        let line = Line::new(
            borders.bottom,
            borders.bottom_intersection,
            borders.bottom_left,
            borders.bottom_right,
        );

        Self::update(line)
    }
}

impl HorizontalLine<On, On, On> {
    /// Creates a new horizontal split line.
    pub const fn full(main: char, intersection: char, left: char, right: char) -> Self {
        let line = Line::new(Some(main), Some(intersection), Some(left), Some(right));
        Self::update(line)
    }

    /// Creates a new horizontal split line.
    pub const fn filled(main: char) -> Self {
        Self::full(main, main, main, main)
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    pub(crate) const fn update(line: Line<char>) -> HorizontalLine<L, R, I> {
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
        self.line.left = Some(c);
        HorizontalLine::update(self.line)
    }

    /// Set a right character.
    pub const fn right(mut self, c: char) -> HorizontalLine<L, On, I> {
        self.line.right = Some(c);
        HorizontalLine::update(self.line)
    }
}

impl<L, R, I> HorizontalLine<L, R, I> {
    /// Get a horizontal character.
    pub const fn get_horizontal(&self) -> char {
        opt_get(self.line.main)
    }

    /// Get a general structure of line.
    pub const fn into_inner(&self) -> Line<char> {
        self.line
    }
}

impl<L, R> HorizontalLine<L, R, On> {
    /// Set a vertical intersection character.
    pub const fn get_intersection(&self) -> char {
        opt_get(self.line.intersection)
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
        opt_get(self.line.left)
    }

    /// Remove a horizontal left character.
    pub const fn remove_left(mut self) -> HorizontalLine<(), R, I> {
        self.line.left = None;
        HorizontalLine::update(self.line)
    }
}

impl<L, I> HorizontalLine<L, On, I> {
    /// Get a right character.
    pub const fn get_right(&self) -> char {
        opt_get(self.line.right)
    }

    /// Remove a horizontal right character.
    pub const fn remove_right(mut self) -> HorizontalLine<L, (), I> {
        self.line.right = None;
        HorizontalLine::update(self.line)
    }
}

impl<L, R, I> From<HorizontalLine<L, R, I>> for Line<char> {
    fn from(value: HorizontalLine<L, R, I>) -> Self {
        value.line
    }
}

impl<L, R, I> From<Line<char>> for HorizontalLine<L, R, I> {
    fn from(value: Line<char>) -> Self {
        let mut line = Self::empty();
        line.line = value;
        line
    }
}

impl<T, B, I> From<HorizontalLine<T, B, I>> for VerticalLine<char> {
    fn from(value: HorizontalLine<T, B, I>) -> Self {
        VerticalLine::new(
            value.line.main,
            value.line.intersection,
            value.line.left,
            value.line.right,
        )
    }
}

const fn opt_get(opt: Option<char>) -> char {
    match opt {
        Some(value) => value,
        None => unreachable!(),
    }
}
