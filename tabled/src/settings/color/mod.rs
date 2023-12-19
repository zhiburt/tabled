//! This module contains a configuration of a [`Border`] or a [`Table`] to set its borders color via [`Color`].
//!
//! [`Border`]: crate::settings::Border
//! [`Table`]: crate::Table

use std::{fmt, ops::BitOr};

use crate::{
    grid::{
        ansi::{ANSIBuf, ANSIFmt, ANSIStr as StaticColor},
        config::{ColoredConfig, Entity},
    },
    settings::{CellOption, TableOption},
};

/// Color represents a color which can be set to things like [`Border`], [`Padding`] and [`Margin`].
///
/// # Example
///
/// ```
/// use tabled::{settings::Color, Table};
///
/// let data = [
///     (0u8, "Hello"),
///     (1u8, "World"),
/// ];
///
/// let table = Table::new(data)
///     .with(Color::BG_BLUE)
///     .to_string();
///
/// println!("{}", table);
/// ```
///
/// [`Padding`]: crate::settings::Padding
/// [`Margin`]: crate::settings::Margin
/// [`Border`]: crate::settings::Border
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    inner: ColorInner,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ColorInner {
    Static(StaticColor<'static>),
    Allocated(ANSIBuf),
}

#[rustfmt::skip]
impl Color {
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLACK:          Self = Self::new_static("\u{1b}[30m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLUE:           Self = Self::new_static("\u{1b}[34m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLACK:   Self = Self::new_static("\u{1b}[90m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLUE:    Self = Self::new_static("\u{1b}[94m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_CYAN:    Self = Self::new_static("\u{1b}[96m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_GREEN:   Self = Self::new_static("\u{1b}[92m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_MAGENTA: Self = Self::new_static("\u{1b}[95m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_RED:     Self = Self::new_static("\u{1b}[91m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_WHITE:   Self = Self::new_static("\u{1b}[97m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_YELLOW:  Self = Self::new_static("\u{1b}[93m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_CYAN:           Self = Self::new_static("\u{1b}[36m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_GREEN:          Self = Self::new_static("\u{1b}[32m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_MAGENTA:        Self = Self::new_static("\u{1b}[35m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_RED:            Self = Self::new_static("\u{1b}[31m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_WHITE:          Self = Self::new_static("\u{1b}[37m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_YELLOW:         Self = Self::new_static("\u{1b}[33m", "\u{1b}[39m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.

    pub const BG_BLACK:          Self = Self::new_static("\u{1b}[40m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BLUE:           Self = Self::new_static("\u{1b}[44m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLACK:   Self = Self::new_static("\u{1b}[100m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLUE:    Self = Self::new_static("\u{1b}[104m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_CYAN:    Self = Self::new_static("\u{1b}[106m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_GREEN:   Self = Self::new_static("\u{1b}[102m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_MAGENTA: Self = Self::new_static("\u{1b}[105m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_RED:     Self = Self::new_static("\u{1b}[101m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_WHITE:   Self = Self::new_static("\u{1b}[107m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_YELLOW:  Self = Self::new_static("\u{1b}[103m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_CYAN:           Self = Self::new_static("\u{1b}[46m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_GREEN:          Self = Self::new_static("\u{1b}[42m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_MAGENTA:        Self = Self::new_static("\u{1b}[45m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_RED:            Self = Self::new_static("\u{1b}[41m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_WHITE:          Self = Self::new_static("\u{1b}[47m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_YELLOW:         Self = Self::new_static("\u{1b}[43m", "\u{1b}[49m");
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BOLD:              Self = Self::new_static("\u{1b}[1m", "\u{1b}[22m");
}

impl Color {
    /// Creates a new [`Color`]` instance, with ANSI prefix and ANSI suffix.
    /// You can use [`TryFrom`] to construct it from [`String`].
    pub fn new<P, S>(prefix: P, suffix: S) -> Self
    where
        P: Into<String>,
        S: Into<String>,
    {
        let color = ANSIBuf::new(prefix, suffix);
        let inner = ColorInner::Allocated(color);

        Self { inner }
    }

    /// Creates a new empty [`Color`]`.
    pub fn empty() -> Self {
        Self::new_static("", "")
    }

    const fn new_static(prefix: &'static str, suffix: &'static str) -> Self {
        let color = StaticColor::new(prefix, suffix);
        let inner = ColorInner::Static(color);

        Self { inner }
    }

    /// Return a prefix.
    pub fn get_prefix(&self) -> &str {
        match &self.inner {
            ColorInner::Static(color) => color.get_prefix(),
            ColorInner::Allocated(color) => color.get_prefix(),
        }
    }

    /// Return a suffix.
    pub fn get_suffix(&self) -> &str {
        match &self.inner {
            ColorInner::Static(color) => color.get_suffix(),
            ColorInner::Allocated(color) => color.get_suffix(),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            inner: ColorInner::Static(StaticColor::default()),
        }
    }
}

impl From<Color> for ANSIBuf {
    fn from(color: Color) -> Self {
        match color.inner {
            ColorInner::Static(color) => ANSIBuf::from(color),
            ColorInner::Allocated(color) => color,
        }
    }
}

impl From<ANSIBuf> for Color {
    fn from(color: ANSIBuf) -> Self {
        Self {
            inner: ColorInner::Allocated(color),
        }
    }
}

impl BitOr for Color {
    type Output = Color;

    fn bitor(self, rhs: Self) -> Self::Output {
        let l_prefix = self.get_prefix();
        let l_suffix = self.get_suffix();
        let r_prefix = rhs.get_prefix();
        let r_suffix = rhs.get_suffix();

        let mut prefix = l_prefix.to_string();
        if l_prefix != r_prefix {
            prefix.push_str(r_prefix);
        }

        let mut suffix = l_suffix.to_string();
        if l_suffix != r_suffix {
            suffix.push_str(r_suffix);
        }

        Self::new(prefix, suffix)
    }
}

#[cfg(feature = "ansi")]
impl std::convert::TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let buf = ANSIBuf::try_from(value)?;

        Ok(Color {
            inner: ColorInner::Allocated(buf),
        })
    }
}

#[cfg(feature = "ansi")]
impl std::convert::TryFrom<String> for Color {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let buf = ANSIBuf::try_from(value)?;

        Ok(Color {
            inner: ColorInner::Allocated(buf),
        })
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let color = self.into();
        let _ = cfg.set_color(Entity::Global, color);
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

impl<R> CellOption<R, ColoredConfig> for Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let color = self.into();
        let _ = cfg.set_color(entity, color);
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

impl<R> CellOption<R, ColoredConfig> for &Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let color = self.clone().into();
        let _ = cfg.set_color(entity, color);
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

impl ANSIFmt for Color {
    fn fmt_ansi_prefix<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match &self.inner {
            ColorInner::Static(color) => color.fmt_ansi_prefix(f),
            ColorInner::Allocated(color) => color.fmt_ansi_prefix(f),
        }
    }

    fn fmt_ansi_suffix<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match &self.inner {
            ColorInner::Static(color) => color.fmt_ansi_suffix(f),
            ColorInner::Allocated(color) => color.fmt_ansi_suffix(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ansi")]
    use ::{owo_colors::OwoColorize, std::convert::TryFrom};

    #[test]
    fn test_xor_operation() {
        assert_eq!(
            Color::FG_BLACK | Color::FG_BLUE,
            Color::new("\u{1b}[30m\u{1b}[34m", "\u{1b}[39m")
        );
        assert_eq!(
            Color::FG_BRIGHT_GREEN | Color::BG_BLUE,
            Color::new("\u{1b}[92m\u{1b}[44m", "\u{1b}[39m\u{1b}[49m")
        );
        assert_eq!(
            Color::new("...", "!!!") | Color::new("@@@", "###"),
            Color::new("...@@@", "!!!###")
        );
        assert_eq!(
            Color::new("...", "!!!") | Color::new("@@@", "###") | Color::new("$$$", "%%%"),
            Color::new("...@@@$$$", "!!!###%%%")
        );
    }

    #[cfg(feature = "ansi")]
    #[test]
    fn test_try_from() {
        assert_eq!(Color::try_from(""), Err(()));
        assert_eq!(Color::try_from("".red().on_green().to_string()), Err(()));
        assert_eq!(Color::try_from("."), Ok(Color::new("", "")));
        assert_eq!(Color::try_from("...."), Ok(Color::new("", "")));
        assert_eq!(
            Color::try_from(".".red().on_green().to_string()),
            Ok(Color::new("\u{1b}[31m\u{1b}[42m", "\u{1b}[39m\u{1b}[49m"))
        );
        assert_eq!(
            Color::try_from("....".red().on_green().to_string()),
            Ok(Color::new("\u{1b}[31m\u{1b}[42m", "\u{1b}[39m\u{1b}[49m"))
        );
    }
}
