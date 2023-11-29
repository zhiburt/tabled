//! This example demonstrates using the [`Merge`] [`TableOption`] to clarify
//! redundancies in a [`Table`] display.
//!
//! * Note how a custom theme is applied to give the [`Merged`](Merge) cells
//! a unique look.
//!
//! * Merge supports both [`Merge::vertical()`] and [`Merge::horizontal()`].

use tabled::{
    settings::{
        style::{HorizontalLine, Style, VerticalLine},
        Border, Merge,
    },
    Table, Tabled,
};

fn main() {
    let data = [
        Database::new("database_1", "database_1", "table_1", 10712),
        Database::new("database_1", "database_1", "table_2", 57),
        Database::new("database_1", "database_1", "table_3", 57),
        Database::new("database_2", "", "table_1", 72),
        Database::new("database_2", "", "table_2", 75),
        Database::new("database_3", "database_3", "table_1", 20),
        Database::new("database_3", "", "table_2", 21339),
        Database::new("database_3", "", "table_3", 141723),
    ];

    let mut table = Table::builder(data).index().transpose().build();
    config_theme(&mut table);
    table.with(Merge::horizontal());

    println!("{table}");
}

#[derive(Tabled)]
struct Database {
    #[tabled(rename = "db")]
    db_name: &'static str,
    origin_db: &'static str,
    #[tabled(rename = "table")]
    table_name: &'static str,
    total: usize,
}

impl Database {
    fn new(
        db_name: &'static str,
        origin_db: &'static str,
        table_name: &'static str,
        total: usize,
    ) -> Self {
        Self {
            db_name,
            origin_db,
            table_name,
            total,
        }
    }
}

fn config_theme(table: &mut Table) {
    let style = Style::modern()
        .frame(Border::inherit(Style::rounded()))
        .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
        .verticals([(1, VerticalLine::inherit(Style::modern()))])
        .remove_horizontal()
        .remove_vertical();

    table.with(style);
}
