use crate::{
    grid::color::AnsiColor,
    grid::config::ColoredConfig,
    settings::{Color, TableOption},
};

/// Set a justification character and a color.
///
/// Default value is ' ' (<space>) with no color.
///
/// # Examples
///
/// Setting a justification character.
///
/// ```
/// use tabled::{
///     Table,
///     settings::formatting::Justification,
/// };
///
/// let mut table = Table::new(&[("Hello", ""), ("", "World")]);
/// table.with(Justification::new('#'));
///
/// assert_eq!(
///     table.to_string(),
///     "+-------+-------+\n\
///      | &str# | &str# |\n\
///      +-------+-------+\n\
///      | Hello | ##### |\n\
///      +-------+-------+\n\
///      | ##### | World |\n\
///      +-------+-------+"
/// );
/// ```
/// Setting a justification color.
///
/// ```
/// use tabled::{
///     Table,
///     settings::{formatting::Justification, Color},
/// };
///
/// let mut table = Table::new(&[("Hello", ""), ("", "World")]);
/// table.with(Justification::default().color(Color::BG_BRIGHT_RED));
///
/// assert_eq!(
///     table.to_string(),
///     "+-------+-------+\n\
///      | &str\u{1b}[101m \u{1b}[49m | &str\u{1b}[101m \u{1b}[49m |\n\
///      +-------+-------+\n\
///      | Hello | \u{1b}[101m     \u{1b}[49m |\n\
///      +-------+-------+\n\
///      | \u{1b}[101m     \u{1b}[49m | World |\n\
///      +-------+-------+"
/// );
/// ```
#[derive(Debug, Default, Clone)]
pub struct Justification {
    c: Option<char>,
    color: Option<AnsiColor<'static>>,
}

impl Justification {
    /// Creates new [`Justification`] object.
    pub fn new(c: char) -> Self {
        Self {
            c: Some(c),
            color: None,
        }
    }

    /// Sets a color for a justification.
    pub fn color(self, color: Color) -> Self {
        Self {
            c: self.c,
            color: Some(color.into()),
        }
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Justification {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let justification_char = self.c.unwrap_or(' ');
        let justification_color = self.color;

        cfg.set_justification(justification_char);
        cfg.set_justification_color(justification_color);
    }
}
