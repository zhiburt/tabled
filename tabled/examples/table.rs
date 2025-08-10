use tabled::{
    settings::{object::Rows, Alignment, Style},
    Table, Tabled,
};

#[derive(Debug, Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
}

fn main() {
    #[rustfmt::skip]
    let data = [
        Distribution { name: String::from("Debian"), based_on: String::from(""), is_active: true },
        Distribution { name: String::from("Arch"), based_on: String::from(""), is_active: true },
        Distribution { name: String::from("Manjaro"), based_on: String::from("Arch"), is_active: true },
    ];

    let mut table = Table::new(data);
    table
        .with(Style::markdown())
        .modify(Rows::first(), Alignment::center());

    println!("{table}");
}
