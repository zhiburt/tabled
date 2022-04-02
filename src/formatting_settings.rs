//! This module contains settings for render strategy of papergrid.

use papergrid::{Entity, Grid, Settings};

use crate::CellOption;

/// RenderSettings responsible for strategy of how we view cells.
///
/// It allows to set [crate::Alignment] settings via [Self::alignement] and [Self::trim].
#[derive(Debug, Default, Clone)]
pub struct RenderSettings {
    alignment_policy: Option<AlignmentStrategy>,
    trim_policy: Option<TrimStrategy>,
    tab_size: Option<usize>,
}

impl RenderSettings {
    /// Set an alignement strategy.
    pub fn alignement(mut self, policy: AlignmentStrategy) -> Self {
        self.alignment_policy = Some(policy);
        self
    }

    /// Set a trim strategy.
    pub fn trim(mut self, policy: TrimStrategy) -> Self {
        self.trim_policy = Some(policy);
        self
    }

    /// Set a tab size.
    ///
    /// The size is used in order to calculate width correctly.
    ///
    /// Default value is 4 (basically 1 '\t' equals 4 spaces).
    ///
    /// IMPORTANT: The tab character might be not present in output,
    /// it might be replaced by spaces.
    pub fn set_tab_size(mut self, n: usize) -> Self {
        self.tab_size = Some(n);
        self
    }
}

/// AlignmentStrategy is a responsible for a flow how we apply an aligment.
/// It mostly matters for multiline strings.
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table, Style, Modify, Alignment, object::Full,
///     formatting_settings::{RenderSettings, AlignmentStrategy}
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
///     .with(
///         Modify::new(Full)
///             .with(Alignment::right())
///             .with(RenderSettings::default().alignement(AlignmentStrategy::PerCell))
///     );
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
/// └───────────────────────────────────────────────────────────────┘
/// "#);
///
/// let table = table.with(Modify::new(Full).with(RenderSettings::default().alignement(AlignmentStrategy::PerLine)));
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
/// └───────────────────────────────────────────────────────────────┘
/// "#);
/// ```
#[derive(Debug, Clone)]
pub enum AlignmentStrategy {
    /// Apply aligment for cell content as a whole.
    PerCell,
    /// Apply aligment for each line of a cell content as a whole.
    PerLine,
}

/// TrimStrategy determins if it's alowed to use empty space while doing [crate::Alignment].
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table, Style, Modify, Alignment, object::Full,
///     formatting_settings::{RenderSettings, TrimStrategy, AlignmentStrategy}
/// };
///
/// let table = Table::new(&["   Hello World"])
///     .with(Style::modern())
///     .with(
///         Modify::new(Full)
///             .with(Alignment::left())
///             .with(RenderSettings::default().trim(TrimStrategy::Horizontal))
///     );
///
/// // Note that nothing was changed exactly.
///
/// assert_eq!(
///     table.to_string(),
///     "┌────────────────┐\n\
///      │ &str           │\n\
///      ├────────────────┤\n\
///      │    Hello World │\n\
///      └────────────────┘\n"
/// );
///
/// // To trim lines you would need also set [AlignmentStrategy]
/// let table = table.with(Modify::new(Full).with(RenderSettings::default().alignement(AlignmentStrategy::PerLine)));
///
/// assert_eq!(
///     table.to_string(),
///     "┌────────────────┐\n\
///      │ &str           │\n\
///      ├────────────────┤\n\
///      │ Hello World    │\n\
///      └────────────────┘\n"
/// );
///
/// let table = Table::new(&["   \n\n\n    Hello World"])
///     .with(Style::modern())
///     .with(
///         Modify::new(Full)
///             .with(Alignment::top())
///             .with(RenderSettings::default().trim(TrimStrategy::Vertical))
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
///      └─────────────────┘\n"
/// );
/// ```
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

impl CellOption for RenderSettings {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        let mut formatting = grid.style(&Entity::Cell(row, column)).formatting;

        if let Some(policy) = &self.alignment_policy {
            match policy {
                AlignmentStrategy::PerCell => formatting.allow_lines_alignement = false,
                AlignmentStrategy::PerLine => formatting.allow_lines_alignement = true,
            }
        }

        if let Some(policy) = &self.trim_policy {
            match policy {
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
        }

        if let &Some(n) = &self.tab_size {
            formatting.tab_width = n;
        }

        grid.set(
            &Entity::Cell(row, column),
            Settings::new().formatting(formatting),
        )
    }
}
