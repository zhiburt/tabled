//! The example can be run using `cargo run --example merge_duplicates2`

use tabled::{
    merge::Merge,
    object::{Cell, Columns, Object, Rows},
    Border, Modify, Style, Table, Tabled,
};

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

    let theme = |table: Table| {
        // we make 1 vertical line
        table
            .with(Style::rounded().off_vertical())
            .with(Modify::new(Columns::first()).with(Border::default().right('│')))
            .with(
                Modify::new(Cell(0, 0)).with(
                    Border::default()
                        .top_right_corner('┬')
                        .bottom_right_corner('┼'),
                ),
            )
            .with(
                Modify::new(Columns::first().intersect(Rows::last()))
                    .with(Border::default().bottom_right_corner('┴')),
            )
    };

    let mut builder = tabled::Table::builder(data).index();
    builder.transpose();

    let table = builder.build();
    let table = theme(table)
        .with(Style::correct_spans())
        .with(Merge::horizontal());

    println!("{}", table);
}
