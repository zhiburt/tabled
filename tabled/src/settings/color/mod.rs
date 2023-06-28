//! This module contains a configuration of a [`Border`] or a [`Table`] to set its borders color via [`Color`].
//!
//! [`Border`]: crate::settings::Border
//! [`Table`]: crate::Table

use std::{borrow::Cow, ops::BitOr};

use crate::{
    grid::{
        color::{AnsiColor, StaticColor},
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(AnsiColor<'static>);

// todo: Add | operation to combine colors

#[rustfmt::skip]
impl Color {
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLACK:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[30m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BLUE:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[34m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLACK:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[90m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_BLUE:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[94m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_CYAN:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[96m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_GREEN:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[92m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_MAGENTA: Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[95m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_RED:     Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[91m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_WHITE:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[97m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_BRIGHT_YELLOW:  Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[93m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_CYAN:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[36m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_GREEN:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[32m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_MAGENTA:        Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[35m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_RED:            Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[31m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_WHITE:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[37m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const FG_YELLOW:         Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[33m"), Cow::Borrowed("\u{1b}[39m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.

    pub const BG_BLACK:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[40m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BLUE:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[44m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLACK:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[100m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_BLUE:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[104m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_CYAN:    Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[106m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_GREEN:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[102m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_MAGENTA: Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[105m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_RED:     Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[101m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_WHITE:   Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[107m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_BRIGHT_YELLOW:  Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[103m"), Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_CYAN:           Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[46m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_GREEN:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[42m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_MAGENTA:        Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[45m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_RED:            Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[41m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_WHITE:          Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[47m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BG_YELLOW:         Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[43m"),  Cow::Borrowed("\u{1b}[49m")));
    /// A color representation.
    /// 
    /// Notice that the colors are constants so you can't combine them.
    pub const BOLD:              Self = Self(AnsiColor::new(Cow::Borrowed("\u{1b}[1m"),  Cow::Borrowed("\u{1b}[22m")));
}

impl Color {
    /// Creates a new [`Color`]` instance, with ANSI prefix and ANSI suffix.
    /// You can use [`TryFrom`] to construct it from [`String`].
    pub fn new(prefix: String, suffix: String) -> Self {
        Self(AnsiColor::new(prefix.into(), suffix.into()))
    }
}

impl From<Color> for AnsiColor<'static> {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<AnsiColor<'static>> for Color {
    fn from(c: AnsiColor<'static>) -> Self {
        Self(c)
    }
}

impl From<StaticColor> for Color {
    fn from(c: StaticColor) -> Self {
        Self(AnsiColor::new(
            Cow::Borrowed(c.get_prefix()),
            Cow::Borrowed(c.get_suffix()),
        ))
    }
}

impl BitOr for Color {
    type Output = Color;

    fn bitor(self, rhs: Self) -> Self::Output {
        let l_prefix = self.0.get_prefix();
        let l_suffix = self.0.get_suffix();
        let r_prefix = rhs.0.get_prefix();
        let r_suffix = rhs.0.get_suffix();

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

#[cfg(feature = "color")]
impl std::convert::TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

#[cfg(feature = "color")]
impl std::convert::TryFrom<String> for Color {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AnsiColor::try_from(value).map(Color)
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let _ = cfg.set_color(Entity::Global, self.0.clone());
    }
}

impl<R> CellOption<R, ColoredConfig> for Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let _ = cfg.set_color(entity, self.0.clone());
    }
}

impl<R> CellOption<R, ColoredConfig> for &Color {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let _ = cfg.set_color(entity, self.0.clone());
    }
}

impl crate::grid::color::Color for Color {
    fn fmt_prefix<W: std::fmt::Write>(&self, f: &mut W) -> std::fmt::Result {
        self.0.fmt_prefix(f)
    }

    fn fmt_suffix<W: std::fmt::Write>(&self, f: &mut W) -> std::fmt::Result {
        self.0.fmt_suffix(f)
    }

    fn colorize<W: std::fmt::Write>(&self, f: &mut W, text: &str) -> std::fmt::Result {
        self.0.colorize(f, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "color")]
    use ::{owo_colors::OwoColorize, std::convert::TryFrom};

    #[test]
    fn test_xor_operation() {
        assert_eq!(
            Color::FG_BLACK | Color::FG_BLUE,
            Color::new(
                String::from("\u{1b}[30m\u{1b}[34m"),
                String::from("\u{1b}[39m")
            )
        );
        assert_eq!(
            Color::FG_BRIGHT_GREEN | Color::BG_BLUE,
            Color::new(
                String::from("\u{1b}[92m\u{1b}[44m"),
                String::from("\u{1b}[39m\u{1b}[49m")
            )
        );
        assert_eq!(
            Color::new(String::from("..."), String::from("!!!"))
                | Color::new(String::from("@@@"), String::from("###")),
            Color::new(String::from("...@@@"), String::from("!!!###"))
        );
        assert_eq!(
            Color::new(String::from("..."), String::from("!!!"))
                | Color::new(String::from("@@@"), String::from("###"))
                | Color::new(String::from("$$$"), String::from("%%%")),
            Color::new(String::from("...@@@$$$"), String::from("!!!###%%%"))
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn test_try_from() {
        assert_eq!(Color::try_from(""), Err(()));
        assert_eq!(Color::try_from("".red().on_green().to_string()), Err(()));
        assert_eq!(
            Color::try_from("."),
            Ok(Color::new(String::new(), String::new()))
        );
        assert_eq!(
            Color::try_from("...."),
            Ok(Color::new(String::new(), String::new()))
        );
        assert_eq!(
            Color::try_from(".".red().on_green().to_string()),
            Ok(Color::new(
                String::from("\u{1b}[31m\u{1b}[42m"),
                String::from("\u{1b}[39m\u{1b}[49m")
            ))
        );
        assert_eq!(
            Color::try_from("....".red().on_green().to_string()),
            Ok(Color::new(
                String::from("\u{1b}[31m\u{1b}[42m"),
                String::from("\u{1b}[39m\u{1b}[49m")
            ))
        );
    }
}
