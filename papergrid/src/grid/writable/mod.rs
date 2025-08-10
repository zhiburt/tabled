//! The module contains a [`WritableGrid`] structure.

mod grid;

use crate::ansi::ANSIFmt;
use std::fmt;

pub use grid::WritableGrid;

/// Typewriter a writer which is enough to implement to be able to render table on it.
pub trait Typewriter {
    /// Start write process.
    fn start(&mut self) -> fmt::Result;

    /// End write process.
    fn finish(&mut self) -> fmt::Result;

    /// Reset line (move cursor to a new line).
    fn reset(&mut self) -> fmt::Result;

    /// Write text.
    ///
    /// Width of the text is given as a first parameter.
    fn write_str(&mut self, text: &str, width: usize) -> fmt::Result;

    /// Write char.
    fn write_char(&mut self, c: char) -> fmt::Result;

    /// Start color for next write commands.
    /// If None is given it's consideren to reset color.
    fn colorize_start<C: ANSIFmt>(&mut self, color: C) -> fmt::Result;

    /// End color for next write commands.
    /// If None is given it's consideren to reset color.
    fn colorize_stop<C: ANSIFmt>(&mut self, color: C) -> fmt::Result;
}

impl<T> Typewriter for &mut T
where
    T: Typewriter,
{
    fn start(&mut self) -> fmt::Result {
        T::start(self)
    }

    fn finish(&mut self) -> fmt::Result {
        T::finish(self)
    }

    fn reset(&mut self) -> fmt::Result {
        T::reset(self)
    }

    fn write_str(&mut self, text: &str, width: usize) -> fmt::Result {
        T::write_str(self, text, width)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        T::write_char(self, c)
    }

    fn colorize_start<C: ANSIFmt>(&mut self, color: C) -> fmt::Result {
        T::colorize_start(self, color)
    }

    fn colorize_stop<C: ANSIFmt>(&mut self, color: C) -> fmt::Result {
        T::colorize_stop(self, color)
    }
}

impl Typewriter for String {
    fn start(&mut self) -> fmt::Result {
        Ok(())
    }

    fn finish(&mut self) -> fmt::Result {
        Ok(())
    }

    fn reset(&mut self) -> fmt::Result {
        self.write_char('\n')
    }

    fn write_str(&mut self, text: &str, _width: usize) -> fmt::Result {
        fmt::Write::write_str(self, text)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        fmt::Write::write_char(self, c)
    }

    fn colorize_start<C: ANSIFmt>(&mut self, color: C) -> fmt::Result {
        color.fmt_ansi_prefix(self)
    }

    fn colorize_stop<C: ANSIFmt>(&mut self, color: C) -> fmt::Result {
        color.fmt_ansi_suffix(self)
    }
}
