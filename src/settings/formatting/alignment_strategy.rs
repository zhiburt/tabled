use crate::{
    grid::config::{Entity, GridConfig},
    settings::CellOption,
};

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
#[derive(Debug, Clone)]
pub enum AlignmentStrategy {
    /// Apply alignment for cell content as a whole.
    PerCell,
    /// Apply alignment for each line of a cell content as a whole.
    PerLine,
}

impl<R> CellOption<R> for AlignmentStrategy {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let mut formatting = *cfg.get_formatting(entity);
        match &self {
            AlignmentStrategy::PerCell => formatting.allow_lines_alignment = false,
            AlignmentStrategy::PerLine => formatting.allow_lines_alignment = true,
        }

        cfg.set_formatting(entity, formatting);
    }
}
