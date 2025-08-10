use tabled::{
    settings::{Height, Style},
    Table,
};

fn main() {
    let data = vec![("Multi\nline\nstring", 1), ("One line", 2), ("One line", 3)];

    let mut table = Table::builder(data).build();
    table.with(Style::markdown());

    let table1 = table.clone().with(Height::increase(10)).to_string();
    let table2 = table.clone().with(Height::limit(4)).to_string();
    let table3 = table.clone().with(Height::limit(0)).to_string();

    println!("Table");
    println!("{table}");
    println!();

    println!("Table increase height to 10");
    println!("{table1}");
    println!();

    println!("Table decrease height to 4");
    println!("{table2}");

    println!("Table decrease height to 0");
    println!("{table3}");
}
