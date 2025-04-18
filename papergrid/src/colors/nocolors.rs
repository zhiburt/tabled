use core::fmt;

use crate::colors::{ANSIFmt, Colors};
use crate::config::Position;

/// The structure represents empty [`Colors`] map.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoColors;

impl Colors for NoColors {
    type Color = NoColor;

    fn get_color(&self, _: Position) -> Option<&Self::Color> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }
}

/// A color which is actually has not value.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoColor;

impl ANSIFmt for NoColor {
    fn fmt_ansi_prefix<W: fmt::Write>(&self, _: &mut W) -> fmt::Result {
        Ok(())
    }

    fn fmt_ansi_suffix<W: fmt::Write>(&self, _: &mut W) -> fmt::Result {
        Ok(())
    }
}
