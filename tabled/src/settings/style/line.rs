#[cfg(feature = "std")]
use crate::grid::config::{HorizontalLine, VerticalLine};

/// The structure represent a vertical or horizontal line.
#[derive(Debug, Default, Clone, Copy)]
pub struct Line {
    pub(crate) main: Option<char>,
    pub(crate) intersection: Option<char>,
    pub(crate) connector1: Option<char>,
    pub(crate) connector2: Option<char>,
}

impl Line {
    /// Creates a new [`Line`] object.
    pub const fn new(
        main: Option<char>,
        intersection: Option<char>,
        connector1: Option<char>,
        connector2: Option<char>,
    ) -> Self {
        Self {
            main,
            intersection,
            connector1,
            connector2,
        }
    }

    /// Creates a new [`Line`] object with all chars set.
    pub const fn full(main: char, intersection: char, connector1: char, connector2: char) -> Self {
        Self::new(
            Some(main),
            Some(intersection),
            Some(connector1),
            Some(connector2),
        )
    }

    /// Creates a new [`Line`] object with all chars set to the provided one.
    pub const fn filled(c: char) -> Self {
        Self::full(c, c, c, c)
    }

    /// Creates a new [`Line`] object with all chars not set.
    pub const fn empty() -> Self {
        Self::new(None, None, None, None)
    }

    /// Checks if the line has nothing set.
    pub const fn is_empty(&self) -> bool {
        self.main.is_none()
            && self.intersection.is_none()
            && self.connector1.is_none()
            && self.connector2.is_none()
    }
}

#[cfg(feature = "std")]
impl From<Line> for HorizontalLine {
    fn from(l: Line) -> Self {
        Self {
            main: l.main,
            intersection: l.intersection,
            left: l.connector1,
            right: l.connector2,
        }
    }
}

#[cfg(feature = "std")]
impl From<Line> for VerticalLine {
    fn from(l: Line) -> Self {
        Self {
            main: l.main,
            intersection: l.intersection,
            top: l.connector1,
            bottom: l.connector2,
        }
    }
}

impl From<Line> for papergrid::config::Line<char> {
    fn from(l: Line) -> Self {
        Self {
            main: l.main.unwrap_or(' '),
            intersection: l.intersection,
            connect1: l.connector1,
            connect2: l.connector2,
        }
    }
}
