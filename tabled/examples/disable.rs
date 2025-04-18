use tabled::{
    settings::{location::ByColumnName, Remove},
    Table, Tabled,
};

#[derive(Tabled)]
struct Distribution {
    name: String,
    based_on: String,
}

fn main() {
    #[rustfmt::skip]
    let data = [
        Distribution { name: String::from("Debian"), based_on: String::from("") },
        Distribution { name: String::from("Arch"), based_on: String::from("") },
        Distribution { name: String::from("Manjaro"), based_on: String::from("Arch") },
    ];

    let mut table = Table::new(data);
    table.with(Remove::column(ByColumnName::new("based_on")));

    println!("{table}");
}
