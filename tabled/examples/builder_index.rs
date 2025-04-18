use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution<'a> {
    name: &'a str,
    #[tabled(display("tabled::derive::display::option", ""))]
    base: Option<&'a str>,
    mainteined: bool,
}

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Distribution { name: "Debian", base: None, mainteined: true },
        Distribution { name: "Arch", base: None, mainteined: true },
        Distribution { name: "Manjaro", base: Some("Arch"), mainteined: true },
    ];

    let mut table = Table::builder(data)
        .index()
        .column(0)
        .transpose()
        .name(None)
        .build();

    table.with(Style::modern_rounded());

    println!("{table}");
}
