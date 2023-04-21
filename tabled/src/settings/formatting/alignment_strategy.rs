use crate::{
    grid::config::{ColoredConfig, CompactMultilineConfig, Entity},
    settings::{CellOption, TableOption},
};

/// `AlignmentStrategy` is a responsible for a flow how we apply an alignment.
/// It mostly matters for multiline strings.
///
/// # Examples
///
/// ```
/// use tabled::{
///     Table,
///     settings::{
///         Style, Modify, Alignment, object::Segment,
///         formatting::{AlignmentStrategy, TrimStrategy}
///     }
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
/// let mut table = Table::new(&[json]);
/// table
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
/// table
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
/// table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlignmentStrategy {
    /// Apply alignment for cell content as a whole.
    PerCell,
    /// Apply alignment for each line of a cell content as a whole.
    PerLine,
}

impl<R> CellOption<R, ColoredConfig> for AlignmentStrategy {
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, entity: Entity) {
        let mut formatting = *cfg.get_formatting(entity);
        match &self {
            AlignmentStrategy::PerCell => formatting.allow_lines_alignment = false,
            AlignmentStrategy::PerLine => formatting.allow_lines_alignment = true,
        }

        cfg.set_formatting(entity, formatting);
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for AlignmentStrategy {
    fn change(self, records: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        <Self as CellOption<R, ColoredConfig>>::change(self, records, cfg, Entity::Global)
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for AlignmentStrategy {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        let mut f = cfg.get_formatting();
        match &self {
            AlignmentStrategy::PerCell => f.allow_lines_alignment = false,
            AlignmentStrategy::PerLine => f.allow_lines_alignment = true,
        }

        *cfg = cfg.set_formatting(f);
    }
}
