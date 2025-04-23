use tabled::{
    settings::{style::Style, themes::BorderCorrection, Merge},
    Table, Tabled,
};

#[derive(Tabled)]
struct DatabaseTable {
    #[tabled(rename = "db")]
    db_name: String,
    #[tabled(rename = "table")]
    table_name: String,
    total: usize,
}

impl DatabaseTable {
    fn new(db_name: &str, table_name: &str, total: usize) -> Self {
        Self {
            db_name: db_name.to_string(),
            table_name: table_name.to_string(),
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

    let mut table = Table::new(data);
    table
        .with(Merge::vertical())
        .with(Style::modern())
        .with(BorderCorrection::span());

    println!("{table}");
}
