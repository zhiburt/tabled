//! This example demonstrates using the [`Merge`] [`TableOption`] to clarify
//! redundancies in a [`Table`] display.
//!
//! * Note how repetative entries must be consecutive, in their specified direction,
//! to be merged together.
//!
//! * Note how [`BorderSpanCorrection`] is used to resolve display issues incurred
//! from [`Span`] decisions made through duplicate detection.
//!
//! * Merge supports both [`Merge::vertical()`] and [`Merge::horizontal()`].

use tabled::{
    settings::{
        style::{BorderSpanCorrection, Style},
        Merge,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct DatabaseTable {
    #[tabled(rename = "db")]
    db_name: &'static str,
    #[tabled(rename = "table")]
    table_name: &'static str,
    total: usize,
}

impl DatabaseTable {
    fn new(db_name: &'static str, table_name: &'static str, total: usize) -> Self {
        Self {
            db_name,
            table_name,
            total,
        }
    }
}

fn main() {
    let data = [
        DatabaseTable::new("database_1", "table_1", 10712),
        DatabaseTable::new("database_1", "table_2", 57),
        DatabaseTable::new("database_1", "table_3", 57),
        DatabaseTable::new("database_2", "table_1", 72),
        DatabaseTable::new("database_2", "table_2", 75),
        DatabaseTable::new("database_3", "table_1", 20),
        DatabaseTable::new("database_3", "table_2", 21339),
        DatabaseTable::new("database_3", "table_3", 141723),
    ];

    let table = Table::new(data)
        .with(Merge::vertical())
        .with(Style::modern())
        .with(BorderSpanCorrection)
        .to_string();

    println!("{table}");
}
