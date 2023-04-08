//! This example demonstrates using the [`Merge`] [`TableOption`] to clarify
//! redundancies in a [`Table`] display.
//!
//! * Note how a custom theme is applied to give the [`Merged`](Merge) cells
//! a unique look.
//!
//! * Merge supports both [`Merge::vertical()`] and [`Merge::horizontal()`].

use tabled::{
    settings::{
        object::{Cell, Columns, Object, Rows},
        style::{Border, BorderSpanCorrection, Style},
        Merge, Modify,
    },
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
    table.with(Merge::horizontal()).with(BorderSpanCorrection);

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
    table
        .with(Style::rounded().remove_vertical())
        .with(Modify::new(Columns::first()).with(Border::default().right('│')))
        .with(
            Modify::new(Cell::new(0, 0)).with(
                Border::default()
                    .corner_top_right('┬')
                    .corner_bottom_right('┼'),
            ),
        )
        .with(
            Modify::new(Columns::first().intersect(Rows::last()))
                .with(Border::default().corner_bottom_right('┴')),
        );
}
