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
        let mut formatting = *cfg.get_formatting(entity);

        // todo: could be changed to be a struct an enum like consts in `impl` block.
        match self {
            TrimStrategy::Vertical => {
                formatting.vertical_trim = true;
            }
            TrimStrategy::Horizontal => {
                formatting.horizontal_trim = true;
            }
            TrimStrategy::Both => {
                formatting.vertical_trim = true;
                formatting.horizontal_trim = true;
            }
            TrimStrategy::None => {
                formatting.vertical_trim = false;
                formatting.horizontal_trim = false;
            }
        }

        cfg.set_formatting(entity, formatting);
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for TrimStrategy {
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<_, _>>::change(self, records, cfg, Entity::Global)
    }
}
