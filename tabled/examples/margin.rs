use tabled::{settings::Margin, Table};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = Table::new(data)
        .with(Margin::new(4, 3, 2, 1).fill('<', '>', 'v', '^'))
        .to_string();

    println!("{table}");
}
