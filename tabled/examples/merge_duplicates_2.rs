//! This example demonstrates using the [`Merge`] [`TableOption`] to clarify
//! redundancies in a [`Table`] display.
//!
//! * Note how a custom theme is applied to give the [`Merged`](Merge) cells
//! a unique look.
//!
//! * Merge supports both [`Merge::vertical()`] and [`Merge::horizontal()`].

use tabled::{
    settings::{style::Style, Merge},
    Table, Tabled,
};

fn main() {
    let data = [
        DatabaseTable::new("database_1", "database_1", "table_1", 10712),
        DatabaseTable::new("database_1", "database_1", "table_2", 57),
        DatabaseTable::new("database_1", "database_1", "table_3", 57),
        DatabaseTable::new("database_2", "", "table_1", 72),
        DatabaseTable::new("database_2", "", "table_2", 75),
        DatabaseTable::new("database_3", "database_3", "table_1", 20),
        DatabaseTable::new("database_3", "", "table_2", 21339),
        DatabaseTable::new("database_3", "", "table_3", 141723),
    ];

    let mut table = Table::builder(data).index().transpose().build();
    config_theme(&mut table);
    table.with(Merge::horizontal());

    println!("{table}");
}

#[derive(Tabled)]
struct DatabaseTable {
    #[tabled(rename = "db")]
    db_name: &'static str,
    origin_db: &'static str,
    #[tabled(rename = "table")]
    table_name: &'static str,
    total: usize,
}

impl DatabaseTable {
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
        .frame(Style::rounded().get_frame())
        .horizontals([(1, Style::modern().get_horizontal_line())])
        .verticals([(1, Style::modern().get_vertical_line())])
        .remove_horizontal()
        .remove_vertical();

    table.with(style);
}
