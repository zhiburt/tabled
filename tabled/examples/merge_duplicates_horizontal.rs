use tabled::{
    settings::{
        style::{HorizontalLine, Style, VerticalLine},
        Border, Merge,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Database {
    #[tabled(rename = "db")]
    db_name: String,
    origin_db: String,
    #[tabled(rename = "table")]
    table_name: String,
    total: usize,
}

impl Database {
    fn new(db_name: &str, origin_db: &str, table_name: &str, total: usize) -> Self {
        Self {
            db_name: db_name.to_string(),
            origin_db: origin_db.to_string(),
            table_name: table_name.to_string(),
            total,
        }
    }
}

fn main() {
    let data = [
        Database::new("db_1", "db_1", "users", 10712),
        Database::new("db_1", "db_1", "devices", 57),
        Database::new("db_1", "db_1", "users", 57),
        Database::new("db_2", "", "users", 72),
        Database::new("db_2", "", "users", 75),
        Database::new("db_3", "db_3", "users", 20),
        Database::new("db_3", "", "devices", 21339),
        Database::new("db_3", "", "io", 141723),
    ];

    let mut table = Table::builder(data).index().transpose().build();
    table.with(
        Style::modern()
            .frame(Border::inherit(Style::rounded()))
            .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
            .verticals([(1, VerticalLine::inherit(Style::modern()))])
            .remove_horizontal()
            .remove_vertical(),
    );
    table.with(Merge::horizontal());

    println!("{table}");
}
