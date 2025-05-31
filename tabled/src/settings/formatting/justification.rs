use crate::{
    grid::{
        ansi::ANSIBuf,
        config::{ColoredConfig, Entity},
    },
    settings::{CellOption, Color, TableOption},
};

/// Set a justification character and a color.
///
/// Default value is `' '` (`<space>`) with no color.
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
///
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
///
/// Use different justification for different columns.
///
/// ```
/// use tabled::{
///     Table,
///     settings::{Modify, object::Columns, formatting::Justification},
/// };
///
/// let mut table = Table::new(&[("Hello", ""), ("", "World")]);
/// table.with(Modify::new(Columns::one(0)).with(Justification::new('#')));
/// table.with(Modify::new(Columns::one(1)).with(Justification::new('@')));
///
/// assert_eq!(
///     table.to_string(),
///     "+-------+-------+\n\
///      | &str# | &str@ |\n\
///      +-------+-------+\n\
///      | Hello | @@@@@ |\n\
///      +-------+-------+\n\
///      | ##### | World |\n\
///      +-------+-------+"
/// );
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct Justification {
    c: Option<char>,
    color: Option<ANSIBuf>,
}

impl Justification {
    /// Creates new [`Justification`] object.
    pub fn new(c: char) -> Self {
        Self {
            c: Some(c),
            color: None,
        }
    }

    /// Creates new [`Justification`] object.
    pub fn colored(c: char, color: Color) -> Self {
        Self {
            c: Some(c),
            color: Some(color.into()),
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

impl<R, D> TableOption<R, ColoredConfig, D> for Justification {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        let c = self.c.unwrap_or(' ');
        let color = self.color;

        cfg.set_justification(Entity::Global, c);
        cfg.set_justification_color(Entity::Global, color);
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}

impl<R> CellOption<R, ColoredConfig> for Justification {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let c = self.c.unwrap_or(' ');
        let color = self.color;

        cfg.set_justification(entity, c);
        cfg.set_justification_color(entity, color);
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}
