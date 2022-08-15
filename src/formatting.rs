//! This module contains settings for render strategy of papergrid.
//!
//! [`TrimStrategy`] and [`AlignmentStrategy`] allows to set [`Alignment`] settings.
//!
//! [`TabSize`] sets a default tab size.
//!
//! [`Alignment`]: crate::Alignment

use papergrid::{
    records::{Records, RecordsMut},
    Entity,
};

use crate::{CellOption, Table, TableOption};

/// Set a tab size.
///
/// The size is used in order to calculate width correctly.
///
/// Default value is 4 (basically 1 '\t' equals 4 spaces).
///
/// IMPORTANT: The tab character might be not present in output,
/// it might be replaced by spaces.
#[derive(Debug, Default, Clone)]
pub struct TabSize(pub usize);

impl<R> TableOption<R> for TabSize
where
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        table.get_config_mut().set_tab_width(self.0);
        table.update_records();
    }
}

/// `AlignmentStrategy` is a responsible for a flow how we apply an alignment.
/// It mostly matters for multiline strings.
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table, Style, Modify, Alignment, object::Segment,
///     formatting::{AlignmentStrategy, TrimStrategy}
/// };
///
/// // sample_from: https://opensource.adobe.com/Spry/samples/data_region/JSONDataSetSample.html
/// let json = r#"
/// {
///     "id": "0001",
///     "type": "donut",
///     "name": "Cake",
///     "ppu": 0.55,
///     "batters": {
///         "batter": [
///             { "id": "1001", "type": "Regular" },
///             { "id": "1002", "type": "Chocolate" },
///         ]
///     },
///     "topping": [
///         { "id": "5001", "type": "None" },
///         { "id": "5006", "type": "Chocolate with Sprinkles" },
///         { "id": "5003", "type": "Chocolate" },
///         { "id": "5004", "type": "Maple" }
///     ]
/// }"#;
///
/// let table = Table::new(&[json])
///     .with(Style::modern())
///     .with(Modify::new(Segment::all()).with(Alignment::right()))
///     .with(Modify::new(Segment::all()).with(TrimStrategy::None));
///
/// println!("{}", table);
///
/// assert_eq!(
///     format!("\n{}", table),
///     r#"
/// ┌───────────────────────────────────────────────────────────────┐
/// │                                                          &str │
/// ├───────────────────────────────────────────────────────────────┤
/// │                                                               │
/// │ {                                                             │
/// │     "id": "0001",                                             │
/// │     "type": "donut",                                          │
/// │     "name": "Cake",                                           │
/// │     "ppu": 0.55,                                              │
/// │     "batters": {                                              │
/// │         "batter": [                                           │
/// │             { "id": "1001", "type": "Regular" },              │
/// │             { "id": "1002", "type": "Chocolate" },            │
/// │         ]                                                     │
/// │     },                                                        │
/// │     "topping": [                                              │
/// │         { "id": "5001", "type": "None" },                     │
/// │         { "id": "5006", "type": "Chocolate with Sprinkles" }, │
/// │         { "id": "5003", "type": "Chocolate" },                │
/// │         { "id": "5004", "type": "Maple" }                     │
/// │     ]                                                         │
/// │ }                                                             │
/// └───────────────────────────────────────────────────────────────┘"#);
///
/// let table = table
///     .with(Modify::new(Segment::all()).with(AlignmentStrategy::PerCell))
///     .with(Modify::new(Segment::all()).with(TrimStrategy::Horizontal));
///
/// assert_eq!(
///     format!("\n{}", table),
///     r#"
/// ┌───────────────────────────────────────────────────────────────┐
/// │                                                          &str │
/// ├───────────────────────────────────────────────────────────────┤
/// │                                                               │
/// │         {                                                     │
/// │         "id": "0001",                                         │
/// │         "type": "donut",                                      │
/// │         "name": "Cake",                                       │
/// │         "ppu": 0.55,                                          │
/// │         "batters": {                                          │
/// │         "batter": [                                           │
/// │         { "id": "1001", "type": "Regular" },                  │
/// │         { "id": "1002", "type": "Chocolate" },                │
/// │         ]                                                     │
/// │         },                                                    │
/// │         "topping": [                                          │
/// │         { "id": "5001", "type": "None" },                     │
/// │         { "id": "5006", "type": "Chocolate with Sprinkles" }, │
/// │         { "id": "5003", "type": "Chocolate" },                │
/// │         { "id": "5004", "type": "Maple" }                     │
/// │         ]                                                     │
/// │         }                                                     │
/// └───────────────────────────────────────────────────────────────┘"#);
///
/// let table = table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));
///
/// assert_eq!(
///     format!("\n{}", table),
///     r#"
/// ┌───────────────────────────────────────────────────────────────┐
/// │                                                          &str │
/// ├───────────────────────────────────────────────────────────────┤
/// │                                                               │
/// │                                                             { │
/// │                                                 "id": "0001", │
/// │                                              "type": "donut", │
/// │                                               "name": "Cake", │
/// │                                                  "ppu": 0.55, │
/// │                                                  "batters": { │
/// │                                                   "batter": [ │
/// │                          { "id": "1001", "type": "Regular" }, │
/// │                        { "id": "1002", "type": "Chocolate" }, │
/// │                                                             ] │
/// │                                                            }, │
/// │                                                  "topping": [ │
/// │                             { "id": "5001", "type": "None" }, │
/// │         { "id": "5006", "type": "Chocolate with Sprinkles" }, │
/// │                        { "id": "5003", "type": "Chocolate" }, │
/// │                             { "id": "5004", "type": "Maple" } │
/// │                                                             ] │
/// │                                                             } │
/// └───────────────────────────────────────────────────────────────┘"#);
/// ```
#[derive(Debug, Clone)]
pub enum AlignmentStrategy {
    /// Apply alignment for cell content as a whole.
    PerCell,
    /// Apply alignment for each line of a cell content as a whole.
    PerLine,
}

impl<R> CellOption<R> for AlignmentStrategy {
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let mut formatting = *table.get_config().get_formatting(entity);
        match &self {
            AlignmentStrategy::PerCell => formatting.allow_lines_alignement = false,
            AlignmentStrategy::PerLine => formatting.allow_lines_alignement = true,
        }

        table.get_config_mut().set_formatting(entity, formatting);
    }
}

/// `TrimStrategy` determines if it's allowed to use empty space while doing [`Alignment`].
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table, Style, Modify, Alignment, object::Segment,
///     formatting::{TrimStrategy, AlignmentStrategy}
/// };
///
/// let table = Table::new(&["   Hello World"])
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
/// let table = table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));
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
/// let table = Table::new(&["   \n\n\n    Hello World"])
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
/// [`Alignment`]: crate::Alignment
#[derive(Debug, Clone)]
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

impl<R> CellOption<R> for TrimStrategy {
    fn change_cell(&mut self, table: &mut Table<R>, entity: Entity) {
        let mut formatting = *table.get_config().get_formatting(entity);

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

        table.get_config_mut().set_formatting(entity, formatting);
    }
}
