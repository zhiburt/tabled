use crate::{
    grid::config::ColoredConfig,
    grid::config::Entity,
    settings::{CellOption, TableOption},
};

/// `TrimStrategy` determines if it's allowed to use empty space while doing [`Alignment`].
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table,
///     settings::{
///         Style, Modify, Alignment, object::Segment,
///         formatting::{TrimStrategy, AlignmentStrategy}
///     }
/// };
///
/// let mut table = Table::new(&["   Hello World"]);
/// table
///     .with(Style::modern())
///     .with(
///         Modify::new(Segment::all())
///             .with(Alignment::left())
///             .with(TrimStrategy::Horizontal)
///     );
///
/// // Note that nothing was changed exactly.
///
/// assert_eq!(
///     table.to_string(),
///     "┌────────────────┐\n\
///      │ &str           │\n\
///      ├────────────────┤\n\
///      │ Hello World    │\n\
///      └────────────────┘"
/// );
///
/// // To trim lines you would need also set [`AlignmentStrategy`].
/// table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));
///
/// assert_eq!(
///     table.to_string(),
///     "┌────────────────┐\n\
///      │ &str           │\n\
///      ├────────────────┤\n\
///      │ Hello World    │\n\
///      └────────────────┘"
/// );
///
/// let mut table = Table::new(&["   \n\n\n    Hello World"]);
/// table
///     .with(Style::modern())
///     .with(
///         Modify::new(Segment::all())
///             .with(Alignment::center())
///             .with(Alignment::top())
///             .with(TrimStrategy::Vertical)
///     );
///
/// assert_eq!(
///     table.to_string(),
///     "┌─────────────────┐\n\
///      │      &str       │\n\
///      ├─────────────────┤\n\
///      │     Hello World │\n\
///      │                 │\n\
///      │                 │\n\
///      │                 │\n\
///      └─────────────────┘"
/// );
/// ```
///
/// [`Alignment`]: crate::settings::Alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrimStrategy {
    /// Allow vertical trim.
    Vertical,
    /// Allow horizontal trim.
    Horizontal,
    /// Allow horizontal and vertical trim.
    Both,
    /// Doesn't allow any trim.
    None,
}

impl<R> CellOption<R, ColoredConfig> for TrimStrategy {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        match self {
            TrimStrategy::Vertical => {
                cfg.set_trim_vertical(entity, true);
            }
            TrimStrategy::Horizontal => {
                cfg.set_trim_horizontal(entity, true);
            }
            TrimStrategy::Both => {
                cfg.set_trim_horizontal(entity, true);
                cfg.set_trim_vertical(entity, true);
            }
            TrimStrategy::None => {
                cfg.set_trim_horizontal(entity, false);
                cfg.set_trim_vertical(entity, false);
            }
        }
    }
}

impl<R, D> TableOption<R, ColoredConfig, D> for TrimStrategy {
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<_, _>>::change(self, records, cfg, Entity::Global)
    }

    fn hint_change(&self) -> Option<Entity> {
        None
    }
}
