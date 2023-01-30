//! The example can be run by this command
//! `cargo run --example table_width`

use tabled::{peaker::PriorityMax, Height, Style, Table};

fn main() {
    let data = vec![("Multi\nline\nstring", 123), ("Single line", 234)];

    let mut table = Table::builder(data).build();
    table.with(Style::markdown());

    println!("Table\n");
    println!("{table}");
    println!();

    let table_increase_height = table.clone().with(Height::increase(10)).to_string();

    println!("Table increase height to 10\n");
    println!("{table_increase_height}");
    println!();

    let table_decrease_height = table
        .clone()
        .with(Height::limit(4).priority::<PriorityMax>())
        .to_string();

    println!("Table decrease height to 4\n");
    println!("{table_decrease_height}");
}
